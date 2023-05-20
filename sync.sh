cd db/
/usr/bin/rm -rf ddnet.*

wget http://ddnet.org/stats/ddnet.sqlite.zip
unzip ddnet.sqlite.zip
cd ..

# Run scripts that process it
cd rankpoints
node index.js
cd ..
cd master-parser
./target/release/master-parser
cd ..

# RUN ANALYZE https://stackoverflow.com/questions/44973667/sqlite-index-seemingly-broken-after-insertion-need-to-run-analyze-which-is-lock
sqlite3 db/playtime.db "ANALYZE"
sqlite3 db/ddnet.sqlite "CREATE INDEX idx_teamrace_name_time ON teamrace (name, time DESC);"
sqlite3 db/ddnet.sqlite "CREATE INDEX idx_race_name_time ON race (name, time DESC);"
sqlite3 db/ddnet.sqlite "CREATE INDEX idx_teamrace_id ON teamrace (id);"
sqlite3 db/ddnet.sqlite "CREATE INDEX idx_teamrankings_id ON teamrankings (id);"