LAST_MODIFIED=$(curl -s -v -X HEAD https://ddnet.org/stats/ddnet.sqlite.zip 2>&1 | grep '^< last-modified:')
if [[ $? -ne 0 ]]; then
    echo "Failed to get last-modified header"
    exit 1;
fi

LAST_MODIFIED_SYNCED=$(cat data/last-modified.txt)
echo $LAST_MODIFIED_SYNCED

if [ "$LAST_MODIFIED" == "$LAST_MODIFIED_SYNCED" ]; then
    echo "ddnet-sqlite.zip has not updated yet. Aborting..."
    exit 1
fi

echo "$LAST_MODIFIED" > data/last-modified.txt

./sync-database.sh
cargo run --release --bin ddstats-cli -- sync

# restart ddstats-web
systemctl --user restart ddstats

# clear cache
varnishadm 'ban req.url ~ .'
