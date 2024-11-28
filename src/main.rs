use base64::Engine;
use ed25519_dalek::Verifier;

mod util;
mod session;
mod channels;
mod message;
mod join;
mod key;
pub mod sync;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();
    log::info!("Starting...");

    let router = axum::Router::new()
        .nest("/api", axum::Router::new()
            .route("/channels", axum::routing::get(channels::get).post(channels::post))
            .route("/message", axum::routing::post(message::post))
            .route("/sync", axum::routing::get(sync::get))
            .route("/join", axum::routing::post(join::post))
            .route("/key", axum::routing::post(key::post))
            .layer(axum::middleware::from_fn(authorized))
            .nest("/session", axum::Router::new()
                .route("/:pubkey", axum::routing::delete(session::delete).layer(axum::middleware::from_fn(authorized)).post(session::post)))
            .layer(axum::middleware::from_fn(no_index)));

    // TODO: unix socket
    //let listener2 = tokio::net::UnixListener::bind("/var/www/enchat/unix/socket")?;

    let listener = tokio::net::TcpListener::bind("localhost:3989").await?;
    log::info!("Started!");

    axum::serve(listener, router).await?;
    Ok(())
}

fn get_requester(headers: &axum::http::HeaderMap, uri: &str, conn: &rusqlite::Connection) -> Result<String, util::ErrorType> {
    match base64::prelude::BASE64_STANDARD.decode(get_token(&headers, uri)?) {
        Ok(signature) => {
            let challenge = get_cookie(&headers, "challenge")?;
            match util::query_get(&conn, "SELECT user_pubkey FROM session WHERE challenge=?1", [challenge.clone()]) {
                Ok(pubkey) => {
                    let key = util::from_public_key_pem(format!("-----BEGIN PUBLIC KEY-----\n{pubkey}\n-----END PUBLIC KEY-----").as_str())?;
                    let signature = util::from_slice(signature.as_slice())?;
                    match key.verify(challenge.as_bytes(), &signature) {
                        Ok(_) => Ok(pubkey),
                        Err(_) => Err(util::ErrorType::Unauthorized(uri.to_string()))
                    }
                },
                Err(_) => {
                    Err(util::ErrorType::Unauthorized(uri.to_string()))
                }
            }
        }
        Err(_) => Err(util::ErrorType::Unauthorized(uri.to_string()))
    }
}

async fn authorized(mut request: axum::extract::Request, next: axum::middleware::Next) -> Result<axum::http::Response<axum::body::Body>, axum::http::Response<axum::body::Body>> {
    let conn = util::db()?;
    let pubkey = get_requester(&request.headers(), request.uri().to_string().as_str(), &conn)?;
    request.extensions_mut().insert(util::Requester(pubkey));
    Ok(next.run(request).await)
}

async fn no_index(request: axum::extract::Request, next: axum::middleware::Next) -> axum::response::Response<axum::body::Body> {
    let mut response = next.run(request).await;
    response.headers_mut().insert("X-Robots-Tag", axum::http::HeaderValue::from_static("noindex"));
    response
}

pub fn get_token(headers: &axum::http::HeaderMap, uri: &str) -> Result<String, util::ErrorType> {
    if let Some(authorization) = headers.get(axum::http::header::AUTHORIZATION) {
        match authorization.to_str() {
            Ok(terms) => {
                let terms = terms.split(' ').collect::<Vec<&str>>();
                if let (Some(&t), Some(data)) = (terms.get(0), terms.get(1)) {
                    if t == "Bearer" {
                        Ok(data.to_string())
                    } else {
                        Err(util::ErrorType::BadRequest)
                    }
                } else {
                    Err(util::ErrorType::BadRequest)
                }
            }
            Err(error) => {
                log::error!("{error}");
                Err(util::ErrorType::BadRequest)
            }
        }
    } else {
        Err(util::ErrorType::Unauthorized(uri.to_string()))
    }
}
pub fn get_cookie(headers: &axum::http::HeaderMap, query: &str) -> Result<String, util::ErrorType> {
    if let Some(cookie) = headers.get(axum::http::header::COOKIE) {
        match cookie.to_str() {
            Ok(terms) => {
                let terms = terms.split(',').collect::<Vec<&str>>();
                let mut d = String::new();
                for string in terms {
                    let terms = string.split('=').collect::<Vec<&str>>();
                    if let (Some(&t), Some(data)) = (terms.get(0), terms.get(1)) {
                        if t == query {
                            d = data.to_string()
                        }
                    }
                }
                if d.is_empty() {
                    Err(util::ErrorType::BadRequest)
                } else {
                    Ok(d)
                }
            }
            Err(error) => {
                log::error!("{error}");
                Err(util::ErrorType::BadRequest)
            }
        }
    } else {
        Err(util::ErrorType::BadRequest)
    }
}

