pub async fn post(axum::extract::Path(pubkey): axum::extract::Path<String>, axum::Form(crate::util::Single { item }): axum::Form<crate::util::Single<String>>) -> Result<(axum::http::StatusCode, axum::Form<crate::util::Single<String>>), axum::http::StatusCode> {
    let db = crate::util::db()?;
    log::info!("so far so good");
    let challenge = uuid::Uuid::new_v4().to_string();
    crate::util::execute(&db, "INSERT INTO session (challenge, user_pubkey, rsa_pubkey) VALUES (?1, ?2, ?3)", (&challenge, pubkey, item))?;
    Ok((axum::http::StatusCode::CREATED, axum::Form(crate::util::Single { item: challenge })))
}
pub async fn delete(axum::extract::Path(pubkey): axum::extract::Path<String>, axum::Extension(crate::util::Requester(requester)): axum::Extension<crate::util::Requester>) -> Result<axum::http::StatusCode, axum::http::StatusCode> {
    let db = crate::util::db()?;
    if pubkey == requester {
        crate::util::execute(&db, "DELETE FROM session WHERE user_pubkey=?1", [&pubkey])?;
        crate::util::execute(&db, "DELETE FROM channel_user WHERE user_pubkey=?1", [&pubkey])?;
        crate::util::execute(&db, "DELETE FROM message WHERE user_pubkey=?1", [&pubkey])?;
        Ok(axum::http::StatusCode::OK)
    } else {
        Err(axum::http::StatusCode::FORBIDDEN)
    }
}