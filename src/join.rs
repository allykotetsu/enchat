#[derive(serde::Deserialize)]
pub struct Join {
    channel: String,
    nick: String
}

pub async fn post(axum::Extension(crate::util::Requester(requester)): axum::Extension<crate::util::Requester>, axum::extract::Form(Join { channel, nick }): axum::extract::Form<Join>) -> Result<axum::http::StatusCode, axum::http::StatusCode> {
    let conn = crate::util::db()?;
    let rsa_pubkey: String = crate::util::query_get(&conn, "SELECT rsa_pubkey FROM session WHERE user_pubkey=?1", [&requester])?;
    crate::util::execute(&conn, "INSERT INTO channel_user (channel_id, user_pubkey, rsa_pubkey, nickname) VALUES (?1, ?2, ?3, ?4)", (channel, requester, rsa_pubkey, nick))?;
    Ok(axum::http::StatusCode::CREATED)
}