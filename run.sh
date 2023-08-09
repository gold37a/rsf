cd /root/rsf
cargo run --release ${times}
git add .
git commit -m "run update"
git push origin main