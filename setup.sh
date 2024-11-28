cargo build --package enchat --release --bin install
cargo build --package enchat --release --bin gen_db
cargo build --package enchat --release --bin system_service
cargo build --package enchat --release --bin enchat
./target/release/install
./target/release/gen_db
./target/release/system_service
systemctl daemon-reload
systemctl enable enchat
systemctl start enchat
