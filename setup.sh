cargo build --package messenger --release --bin install
cargo build --package messenger --release --bin gen_db
cargo build --package messenger --release --bin system_service
cargo build --package messenger --release --bin messenger
./target/release/install
./target/release/gen_db
./target/release/system_service
systemctl daemon-reload
systemctl enable enchat
systemctl start enchat