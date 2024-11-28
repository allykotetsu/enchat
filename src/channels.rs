#[derive(serde::Serialize)]
pub struct ChannelsGet {
    id: String,
    name: String
}
#[derive(serde::Deserialize)]
pub struct ChannelsPost {
    name: String,
    nick: String
}

pub async fn get(axum::Extension(crate::util::Requester(requester)): axum::Extension<crate::util::Requester>) -> Result<axum::response::Json<crate::util::Array<ChannelsGet>>, axum::http::StatusCode> {
    let conn = crate::util::db()?;
    let mut stmt = crate::util::prepare(&conn, "SELECT channel_id FROM channel_user WHERE user_pubkey=?1")?;
    let mut channel_ids = vec![];
    for channel_id in crate::util::query_map(&mut stmt, [requester], |row| row.get::<usize, String>(0))? {
        match channel_id {
            Ok(channel_id) => {
                if !channel_ids.contains(&channel_id) {
                    channel_ids.push(channel_id);
                }
            },
            Err(error) => log::error!("{error}")
        }
    }
    let mut item = vec![];
    for id in channel_ids {
        let name: String = crate::util::query_get(&conn, "SELECT channel_name FROM channel WHERE channel_id=?1", [&id])?;
        item.push(ChannelsGet { id, name });
    }
    Ok(axum::response::Json(crate::util::Array { item }))
}
pub async fn post(axum::Extension(crate::util::Requester(requester)): axum::Extension<crate::util::Requester>, axum::extract::Form(ChannelsPost { name, nick }): axum::extract::Form<ChannelsPost>) -> Result<(axum::http::StatusCode, axum::extract::Form<crate::util::Single<String>>), axum::http::StatusCode> {
    let conn = crate::util::db()?;
    // TODO migrate to non uuid
    let item = uuid::Uuid::new_v4().to_string();
    crate::util::execute(&conn, "INSERT INTO channel (channel_id, channel_name) VALUES (?1, ?2)", (&item, name))?;
    let rsa_pubkey: String = crate::util::query_get(&conn, "SELECT rsa_pubkey FROM session WHERE user_pubkey=?1", [&requester])?;
    crate::util::execute(&conn, "INSERT INTO channel_user (channel_id, user_pubkey, rsa_pubkey, nickname) VALUES (?1, ?2, ?3, ?4)", (&item, requester, rsa_pubkey, nick))?;
    Ok((axum::http::StatusCode::CREATED, axum::extract::Form(crate::util::Single { item })))
}