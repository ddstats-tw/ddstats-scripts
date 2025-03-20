-- extensions needed (postgresql-contrib)
CREATE EXTENSION IF NOT EXISTS pg_trgm;

-- race
CREATE INDEX IF NOT EXISTS idx_race_name ON race (name);
CREATE INDEX IF NOT EXISTS idx_race_time_desc ON race (time DESC);
CREATE INDEX IF NOT EXISTS idx_race_map_timestamp_time ON race (map, time ASC, timestamp);

-- teamrace
CREATE INDEX IF NOT EXISTS idx_teamrace_map_timestamp_time ON teamrace (map, timestamp, time ASC);

-- players
CREATE INDEX IF NOT EXISTS trgm_idx_name ON players USING gin (name gin_trgm_ops);

-- rankings
CREATE INDEX IF NOT EXISTS idx_rankings_map ON rankings (map);
CREATE INDEX IF NOT EXISTS idx_rankings_name ON rankings (name);
CREATE INDEX IF NOT EXISTS idx_rankings_rank_top5 ON rankings (rank) WHERE rank <= 5;

-- teamrankings
CREATE INDEX IF NOT EXISTS idx_teamrankings_map ON teamrankings (map);
CREATE INDEX IF NOT EXISTS idx_teamrankings_players on teamrankings USING GIN (players);
CREATE INDEX IF NOT EXISTS idx_teamrankings_rank_top5 ON teamrankings (rank) WHERE rank <= 5;

-- playtime
CREATE INDEX IF NOT EXISTS idx_playtime_name ON playtime (name);
CREATE INDEX IF NOT EXISTS idx_playtime_map ON playtime (map);
CREATE INDEX IF NOT EXISTS idx_playtime_date ON playtime (date);

-- playtime_maps
CREATE INDEX IF NOT EXISTS idx_playtime_maps_name ON playtime_maps (name);
CREATE INDEX IF NOT EXISTS idx_playtime_maps_map_seconds_played ON playtime_maps (map, seconds_played DESC);

-- rankedpoints
CREATE INDEX IF NOT EXISTS idx_rankedpoints_name ON rankedpoints (name);
