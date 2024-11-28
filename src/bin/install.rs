use std::fs::OpenOptions;
use std::io::Write;

mod app_index;
mod app_script;
mod app_sync;
mod enchat_frontend;
mod index;
mod invite_index;
mod invite_script;
mod script;
mod style;

fn init() -> Result<(), Box<dyn std::error::Error>> {
    // TODO fix

    std::fs::create_dir("/var/www/enchat")?;
    std::fs::create_dir("/var/www/enchat/html")?;
    OpenOptions::new().write(true).create(true).open("/var/www/enchat/html/index.html")?.write_all(index::GET.as_bytes())?;
    OpenOptions::new().write(true).create(true).open("/var/www/enchat/html/script.js")?.write_all(script::GET.as_bytes())?;
    OpenOptions::new().write(true).create(true).open("/var/www/enchat/html/style.css")?.write_all(style::GET.as_bytes())?;

    std::fs::create_dir("/var/www/enchat/html/app")?;
    OpenOptions::new().write(true).create(true).open("/var/www/enchat/html/app/index.html")?.write_all(app_index::GET.as_bytes())?;
    OpenOptions::new().write(true).create(true).open("/var/www/enchat/html/app/script.js")?.write_all(app_script::GET.as_bytes())?;
    OpenOptions::new().write(true).create(true).open("/var/www/enchat/html/app/sync.js")?.write_all(app_sync::GET.as_bytes())?;

    std::fs::create_dir("/var/www/enchat/html/invite")?;
    OpenOptions::new().write(true).create(true).open("/var/www/enchat/html/invite/index.html")?.write_all(invite_index::GET.as_bytes())?;
    OpenOptions::new().write(true).create(true).open("/var/www/enchat/html/invite/script.js")?.write_all(invite_script::GET.as_bytes())?;

    std::fs::create_dir("/var/www/enchat/nginx")?;
    OpenOptions::new().write(true).create(true).open("/var/www/enchat/nginx/enchat_frontend.conf")?.write_all(enchat_frontend::GET.as_bytes())?;

    std::fs::create_dir("/var/www/enchat/unix")?;
    std::fs::create_dir("/var/www/enchat/db")?;

    Ok(())
}

fn main() -> Result<(), ()> {
    tracing_subscriber::fmt::init();
    log::info!("Installing...");

    match init() {
        Ok(_) => Ok(log::info!("Installed!")),
        Err(error) => Err(log::error!("{error}"))
    }
}