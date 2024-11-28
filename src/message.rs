#[derive(serde::Deserialize)]
pub struct MessagePost {
    channel_id: String,
    content: String,
    iv: String
}

pub async fn post(axum::Extension(crate::util::Requester(requester)): axum::Extension<crate::util::Requester>, axum::extract::Form(MessagePost { channel_id, content, iv }): axum::extract::Form<MessagePost>) -> Result<(axum::http::StatusCode, axum::http::Response<String>), axum::http::StatusCode> {
    let conn = crate::util::db()?;
    let nick: String = crate::util::query_get(&conn, "SELECT nickname FROM channel_user WHERE channel_id=?1 AND user_pubkey=?2", (&channel_id, &requester))?;
    crate::util::execute(&conn, "INSERT INTO message (channel_id, user_pubkey, nick, content, timestamp, iv) VALUES (?1, ?2, ?3, ?4, ?5, ?6)", (channel_id, &requester, nick, content, chrono::offset::Local::now().timestamp_millis(), iv))?;

    let mut stmt = crate::util::prepare(&conn, "SELECT rsa_pubkey FROM channel_user WHERE user_pubkey!=?1")?;
    let mut item = vec![];
    for rsa_pubkey in crate::util::query_map(&mut stmt, [requester], |row| row.get::<usize, String>(0))? {
        match rsa_pubkey {
            Ok(rsa_pubkey) => item.push(rsa_pubkey),
            Err(error) => log::error!("{error}")
        }
    }

    Ok((axum::http::StatusCode::CREATED, crate::util::Array::<String> { item }.into()))
}