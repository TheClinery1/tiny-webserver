cd ../webserver
cargo rustc --release -- -C link-args=-s
cp target/release/webserver-test ../runtime/webserver
cd ../runtime
./webserver
