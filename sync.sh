./sync-database.sh
cargo run --release --bin ddstats-cli -- sync

# clear cache
varnishadm 'ban req.url ~ .'
