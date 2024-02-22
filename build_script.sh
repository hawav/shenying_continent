cargo build --release # Mac Apple Silicon
cargo build --release --target x86_64-apple-darwin # Mac Intel
cargo xwin build --target x86_64-pc-windows-msvc -r # Windows

mv target/release/shenying_continent target/release/shenying_continent_mac_apple_silicon
mv target/x86_64-apple-darwin/release/shenying_continent target/release/shenying_continent_mac_intel
mv target/x86_64-pc-windows-msvc/release/shenying_continent.exe target/release/shenying_continent_win.exe
