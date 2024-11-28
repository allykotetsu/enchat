#[derive(serde::Deserialize)]
pub struct Key {
    rsa_pubkey: String,
    key: String,
    iv: String
}

pub async fn post(axum::Extension(crate::util::Requester(_)): axum::Extension<crate::util::Requester>, axum::extract::Form(Key { rsa_pubkey, key, iv }): axum::extract::Form<Key>) -> Result<axum::http::StatusCode, axum::http::StatusCode> {
    let conn = crate::util::db()?;
    crate::util::execute(&conn, "INSERT INTO key_transfer (rsa_pubkey, key, iv) VALUES (?1, ?2, ?3)", (rsa_pubkey, key, iv))?;
    Ok(axum::http::StatusCode::CREATED)
}