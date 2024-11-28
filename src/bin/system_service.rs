use std::io::Write;

fn init() -> Result<(), Box<dyn std::error::Error>> {
    std::fs::OpenOptions::new().write(true).create(true).open("/etc/systemd/system/enchat.service")?.write_all(format!(r#"[Unit]
Description=Run enchat.
Wants=network.target
After=syslog.target network-online.target

[Service]
Type=simple
#WorkingDirectory={{}}
#ExecStart=cargo run --package messenger --bin messenger
ExecStart={}/target/release/messenger
Restart=on-failure
RestartSec=10
KillMode=process

[Install]
WantedBy=multi-user.target"#, std::env::current_dir()?.display()).as_bytes())?;

    Ok(())
}

fn main() -> Result<(), ()> {
    tracing_subscriber::fmt::init();
    log::info!("Setting up system service...");

    match init() {
        Ok(_) => Ok(log::info!("Setup system service!")),
        Err(error) => Err(log::error!("{error}"))
    }
}