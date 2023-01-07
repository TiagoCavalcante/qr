cargo build --release
cp ./target/release/qr ./scripts/qr
cd ./scripts
tar -czf qr.tar.gz qr
rm qr
cd ..
