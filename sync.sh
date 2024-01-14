pm2 stop ddstats
pm2 stop ddstats-bot

cd db/
rm -rf ddnet.*

wget http://ddnet.org/stats/ddnet.sqlite.zip
unzip ddnet.sqlite.zip
cd ..

# Create required indexes
sqlite3 db/ddnet.sqlite "CREATE INDEX idx_race_map_name_time ON race (map, name, time);"

# Run scripts that process it
cd rankpoints
node index.js
cd ..
cd master-parser
cargo run --release
cd ..
cd misc
python3 rankings.py
python3 points.py
python3 mapinfo.py
cd ..

# Add maps to playtime DB
sqlite3 db/master.db "CREATE TABLE IF NOT EXISTS maps ( \
  Map varchar(128) PRIMARY KEY NOT NULL \
,  Server varchar(32) NOT NULL \
,  Points integer NOT NULL DEFAULT 0 \
,  Stars integer NOT NULL DEFAULT 0 \
,  Mapper varchar(128) NOT NULL \
,  Timestamp timestamp NOT NULL DEFAULT '0000-00-00 00:00:00' \
);"
sqlite3 db/ddnet.sqlite ".dump maps" | sqlite3 db/master.db 2> /dev/null

sqlite3 db/ddnet.sqlite "CREATE INDEX idx_rankings_rank_name ON rankings (rank ASC, name)"
sqlite3 db/ddnet.sqlite "CREATE INDEX idx_teamrankings_rank ON teamrankings (rank ASC)"

# player page
sqlite3 db/ddnet.sqlite "CREATE INDEX idx_rankings_name ON rankings (name)"
sqlite3 db/ddnet.sqlite "CREATE INDEX idx_rankings_name_map ON rankings (name, map)"
sqlite3 db/ddnet.sqlite "CREATE INDEX idx_teamrankings_name_map ON teamrankings (name, map)"

# Map profiles
sqlite3 db/ddnet.sqlite "CREATE INDEX idx_rankings_map_rank ON rankings (map, rank ASC);"
sqlite3 db/ddnet.sqlite "CREATE INDEX idx_teamrankings_map_rank ON teamrankings (map, rank ASC);"

# RUN ANALYZE https://stackoverflow.com/questions/44973667/sqlite-index-seemingly-broken-after-insertion-need-to-run-analyze-which-is-lock
sqlite3 db/master.db "ANALYZE"
sqlite3 db/ddnet.sqlite "CREATE INDEX idx_teamrace_name_time ON teamrace (name, time DESC);"
sqlite3 db/ddnet.sqlite "CREATE INDEX idx_race_name_time ON race (name, time DESC);"
sqlite3 db/ddnet.sqlite "CREATE INDEX idx_teamrace_id ON teamrace (id);"
sqlite3 db/ddnet.sqlite "CREATE INDEX idx_teamrankings_id ON teamrankings (id);"

sqlite3 db/ddnet.sqlite "CREATE INDEX idx_maps_map ON maps (map);"

# most rank1s
sqlite3 db/ddnet.sqlite "CREATE INDEX idx_rankings_rank_top5 ON rankings (rank) WHERE rank <= 5;"
sqlite3 db/ddnet.sqlite "CREATE INDEX idx_teamrankings_rank_top5 ON teamrankings (rank) WHERE rank <= 5;"

# Rank1s Tabs
sqlite3 db/ddnet.sqlite "CREATE INDEX idx_rankings_name_rank_top10 ON rankings (name, rank) WHERE rank <= 10;"
sqlite3 db/ddnet.sqlite "CREATE INDEX idx_teamrankings_name_rank_top10 ON teamrankings (name, rank) WHERE rank <= 10;"

# worst ranks
sqlite3 db/ddnet.sqlite "CREATE INDEX idx_race_map_time_desc ON race (map, time DESC)"

# Searching
sqlite3 db/ddnet.sqlite "CREATE VIRTUAL TABLE players USING fts4(name TEXT, points INTEGER);"
sqlite3 db/ddnet.sqlite "INSERT INTO players (name, points) SELECT Name, SUM(maps.Points) FROM rankings JOIN maps ON rankings.map = maps.map GROUP BY name;"

# Start
cd ../web
pm2 start index.js -i 6 --name ddstats
cd ../ddstats-bot
pm2 start index.js --name ddstats-bot

# clear cache
varnishadm 'ban req.url ~ .'