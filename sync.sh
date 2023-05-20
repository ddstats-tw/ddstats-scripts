cd db/
/usr/bin/rm -rf ddnet.*

wget http://ddnet.org/stats/ddnet.sqlite.zip
unzip ddnet.sqlite.zip

# Run scripts that process it
cd rankpoints
node index.js
cd ..
cd ddstats-scripts
./target/release/ddstats-scripts
cd ..

# RUN ANALYZE https://stackoverflow.com/questions/44973667/sqlite-index-seemingly-broken-after-insertion-need-to-run-analyze-which-is-lock
#mv ddnet.sqlite ddnet.db
sqlite3 playtime.db "ANALYZE"
sqlite3 ddnet.sqlite "CREATE INDEX idx_teamrace_name_time ON teamrace (name, time DESC);"
sqlite3 ddnet.sqlite "CREATE INDEX idx_race_name_time ON race (name, time DESC);"
sqlite3 ddnet.sqlite "CREATE INDEX idx_teamrace_id ON teamrace (id);"
sqlite3 ddnet.sqlite "CREATE INDEX idx_teamrankings_id ON teamrankings (id);"

cd ..