#[derive(serde::Serialize)]
pub struct Sync {
    messages: Vec<Message>,
    keys: Vec<Key>
}
#[derive(serde::Serialize)]
pub struct Message {
    content: String,
    sender: String,
    channel_id: String,
    timestamp: u64,
    iv: String
}
#[derive(serde::Serialize)]
pub struct Key {
    key: String,
    iv: String
}
#[derive(serde::Deserialize)]
pub struct SyncQuery {
    ts: u64
}

pub async fn get(axum::Extension(crate::util::Requester(requester)): axum::Extension<crate::util::Requester>, axum::extract::Query(SyncQuery { ts }): axum::extract::Query<SyncQuery>) -> Result<axum::response::Json<Sync>, axum::http::StatusCode> {
    let conn = crate::util::db()?;

    let mut stmt = crate::util::prepare(&conn, "SELECT channel_id FROM channel_user WHERE user_pubkey=?1")?;
    let mut channels = vec![];
    for channel_id in crate::util::query_map(&mut stmt, [&requester], |row| row.get::<usize, String>(0))? {
        match channel_id {
            Ok(channel_id) => {
                if !channels.contains(&channel_id) {
                    channels.push(channel_id);
                }
            },
            Err(error) => log::error!("{error}")
        }
    }

    let mut messages = vec![];

    if channels.len() > 0 {
        let channels = channels.iter().map(|c| format!("'{c}'")).collect::<Vec<String>>().join(", ");
        let mut stmt = crate::util::prepare(&conn, format!("SELECT channel_id, nick, content, timestamp, iv FROM message WHERE timestamp>?1 AND channel_id IN ({channels})").as_str())?;
        for message in crate::util::query_map(&mut stmt, [ts], |row|
            Ok((row.get::<usize, String>(0)?, row.get::<usize, String>(1)?, row.get::<usize, String>(2)?, row.get::<usize, u64>(3)?, row.get::<usize, String>(4)?))
        )? {
            match message {
                Ok((channel_id, sender, content, timestamp, iv)) => messages.push(Message { sender, content, channel_id, timestamp, iv }),
                Err(error) => log::error!("{error}")
            }
        }
    }

    let rsa_pubkey: String = crate::util::query_get(&conn, "SELECT rsa_pubkey FROM session WHERE user_pubkey=?1", [requester])?;
    let mut keys = vec![];

    let mut stmt = crate::util::prepare(&conn, "SELECT key, iv FROM key_transfer WHERE rsa_pubkey=?1")?;
    for key_transfer in crate::util::query_map(&mut stmt, [rsa_pubkey], |row|
        Ok((row.get::<usize, String>(0)?, row.get::<usize, String>(1)?))
    )? {
        match key_transfer {
            Ok((key, iv)) => keys.push(Key { key, iv }),
            Err(error) => log::error!("{error}")
        }
    }

    let ivs = keys.iter().map(|Key { iv, .. }| format!("'{iv}'")).collect::<Vec<String>>().join(", ");
    crate::util::execute(&conn, format!("DELETE FROM key_transfer WHERE iv IN ({ivs})").as_str(), ())?;

    Ok(axum::response::Json(Sync { messages, keys }))
}