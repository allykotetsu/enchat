use ed25519_dalek::pkcs8::DecodePublicKey;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Single<T> {
    pub item: T
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Array<T> {
    pub item: Vec<T>
}

#[derive(Clone)]
pub struct Requester(pub String);

pub enum ErrorType {
    Unauthorized(String),
    BadRequest,
    InternalServerError(String),
    NotFound(String)
}

impl From<ErrorType> for axum::http::Response<axum::body::Body> {
    fn from(value: ErrorType) -> Self {
        match value {
            ErrorType::Unauthorized(uri) => axum::http::response::Response::builder().status(axum::http::StatusCode::UNAUTHORIZED).header(axum::http::header::WWW_AUTHENTICATE, format!("Bearer realm=\"{uri}\"").as_str()).body(axum::body::Body::empty()).unwrap(),
            ErrorType::BadRequest => axum::http::response::Response::builder().status(axum::http::StatusCode::BAD_REQUEST).body(axum::body::Body::empty()).unwrap(),
            ErrorType::InternalServerError(why) => {
                log::error!("{why}");
                axum::http::response::Response::builder().status(axum::http::StatusCode::INTERNAL_SERVER_ERROR).body(axum::body::Body::empty()).unwrap()
            },
            ErrorType::NotFound(what) => {
                log::warn!("{what} was not found.");
                axum::http::response::Response::builder().status(axum::http::StatusCode::NOT_FOUND).body(axum::body::Body::empty()).unwrap()
            }
        }
    }
}

impl From<ErrorType> for axum::http::StatusCode {
    fn from(value: ErrorType) -> Self {
        match value {
            ErrorType::Unauthorized(_) => axum::http::StatusCode::UNAUTHORIZED,
            ErrorType::BadRequest => axum::http::StatusCode::BAD_REQUEST,
            ErrorType::InternalServerError(string) => {
                log::error!("{string}");
                axum::http::StatusCode::INTERNAL_SERVER_ERROR
            },
            ErrorType::NotFound(_) => axum::http::StatusCode::NOT_FOUND
        }
    }
}

impl<T: std::fmt::Display> From<Array<T>> for axum::response::Response<String> {
    fn from(Array { item }: Array<T>) -> Self {
        let mut ret = "".to_string();
        for t in item {
            let t = format!("{t}");
            let t = urlencoding::encode(t.as_str());
            ret = format!("{ret}&item={t}");
        }
        if ret.len() > 0 {
            ret = (&ret.as_str()[1..]).to_string();
        }
        axum::response::Response::builder().status(axum::http::StatusCode::OK).header(axum::http::header::CONTENT_TYPE, "application/x-www-form-urlencoded").body(ret).unwrap()
    }
}

fn sqlite_err_to_custom(error: rusqlite::Error) -> ErrorType {
    match error {
        rusqlite::Error::QueryReturnedNoRows => ErrorType::NotFound("".to_string()),
        _ => ErrorType::InternalServerError("internal sqlite error".to_string())
    }
}

pub fn db() -> Result<rusqlite::Connection, ErrorType> {
    match rusqlite::Connection::open("/var/www/enchat/db/database.db") {
        Ok(conn) => Ok(conn),
        Err(error) => {
            log::error!("{error}");
            Err(sqlite_err_to_custom(error))
        }
    }
}

pub fn query_row<T, P, F>(conn: &rusqlite::Connection, sql: &str, params: P, f: F) -> Result<T, ErrorType>
where P: rusqlite::Params, F: FnOnce(&rusqlite::Row<'_>) -> rusqlite::Result<T> {
    match conn.query_row(sql, params, f) {
        Ok(t) => Ok(t),
        Err(error) => {
            log::error!("{error}");
            Err(sqlite_err_to_custom(error))
        }
    }
}

pub fn query_get<T, P>(conn: &rusqlite::Connection, sql: &str, params: P) -> Result<T, ErrorType>
where P: rusqlite::Params, T: rusqlite::types::FromSql {
    query_row(&conn, sql, params, |row| row.get::<usize, T>(0))
}

pub fn execute<P: rusqlite::Params>(conn: &rusqlite::Connection, sql: &str, params: P) -> Result<(), ErrorType> {
    match conn.execute(sql, params) {
        Ok(_) => Ok(()),
        Err(error) => {
            log::error!("{error}");
            Err(sqlite_err_to_custom(error))
        }
    }
}

pub fn prepare<'a>(conn: &'a rusqlite::Connection, sql: &str) -> Result<rusqlite::Statement<'a>, ErrorType> {
    match conn.prepare(sql) {
        Ok(statement) => Ok(statement),
        Err(error) => {
            log::error!("{error}");
            Err(sqlite_err_to_custom(error))
        }
    }
}

pub fn query_map<'a, T, P, F>(statement: &'a mut rusqlite::Statement, params: P, f: F) -> Result<rusqlite::MappedRows<'a, F>, ErrorType>
where
    P: rusqlite::Params,
    F: FnMut(&rusqlite::Row<'_>) -> rusqlite::Result<T>,
{
    match statement.query_map(params, f) {
        Ok(mr) => Ok(mr),
        Err(error) => {
            log::error!("{error}");
            Err(sqlite_err_to_custom(error))
        }
    }
}

pub fn from_public_key_pem(s: &str) -> Result<ed25519_dalek::VerifyingKey, ErrorType> {
    match ed25519_dalek::VerifyingKey::from_public_key_pem(s) {
        Ok(verifying_key) => Ok(verifying_key),
        Err(error) => Err(ErrorType::InternalServerError(error.to_string()))
    }
}

pub fn from_slice(bytes: &[u8]) -> Result<ed25519_dalek::ed25519::Signature, ErrorType> {
    match ed25519_dalek::Signature::from_slice(bytes) {
        Ok(signature) => Ok(signature),
        Err(error) => Err(ErrorType::InternalServerError(error.to_string()))
    }
}