./sync-database.sh
cargo run --release --bin ddstats-cli -- sync

# restart ddstats-web
systemctl --user restart ddstats

# clear cache
varnishadm 'ban req.url ~ .'
