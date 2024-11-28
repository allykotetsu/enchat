fn init() -> Result<(), Box<dyn std::error::Error>> {
    let conn = rusqlite::Connection::open("/var/www/enchat/db/database.db")?;
    log::info!("Creating session table...");
    conn.execute(r#"CREATE TABLE IF NOT EXISTS session (
        challenge TEXT PRIMARY KEY NOT NULL,
        user_pubkey TEXT NOT NULL,
        rsa_pubkey TEXT NOT NULL
    )"#, ())?;
    log::info!("Created session table.");

    log::info!("Creating channel table...");
    conn.execute(r#"CREATE TABLE IF NOT EXISTS channel (
        channel_id TEXT PRIMARY KEY NOT NULL,
        channel_name TEXT NOT NULL
    )"#, ())?;
    log::info!("Created channel table.");

    log::info!("Creating channel_user table...");
    conn.execute(r#"CREATE TABLE IF NOT EXISTS channel_user (
        channel_user_id INTEGER PRIMARY KEY,
        channel_id TEXT NOT NULL,
        user_pubkey TEXT NOT NULL,
        rsa_pubkey TEXT NOT NULL,
        nickname TEXT NOT NULL
    )"#, ())?;
    log::info!("Created channel_user table.");

    log::info!("Creating message table...");
    conn.execute(r#"CREATE TABLE IF NOT EXISTS message (
        message_id INTEGER PRIMARY KEY,
        channel_id TEXT NOT NULL,
        user_pubkey TEXT NOT NULL,
        nick TEXT NOT NULL,
        content TEXT NOT NULL,
        timestamp INT NOT NULL,
        iv TEXT NOT NULL
    )"#, ())?;
    log::info!("Created member table.");

    log::info!("Creating key_transfer table...");
    conn.execute(r#"CREATE TABLE IF NOT EXISTS key_transfer (
        key_transfer_id INTEGER PRIMARY KEY,
        rsa_pubkey TEXT NOT NULL,
        key TEXT NOT NULL,
        iv TEXT NOT NULL
    )"#, ())?;
    log::info!("Created key_transfer table.");

    Ok(())
}

fn main() -> Result<(), ()> {
    tracing_subscriber::fmt::init();
    log::info!("Generating database...");

    match init() {
        Ok(_) => Ok(log::info!("Generated database!")),
        Err(error) => Err(log::error!("{error}"))
    }
}