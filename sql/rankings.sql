BEGIN;
DROP TABLE IF EXISTS rankings_tmp;
CREATE TABLE IF NOT EXISTS rankings_tmp (LIKE rankings INCLUDING ALL);

INSERT INTO
    rankings_tmp (
        rank,
        timestamp,
        name,
        time,
        map,
        server
)
SELECT RANK() OVER (PARTITION BY map ORDER BY time) AS rank, * FROM (
    SELECT distinct on (map, name) 
        timestamp,
        name,
        time,
        map,
        server
    FROM
        race
    ORDER BY
        map,
        name,
        time
) AS ranks ORDER BY time;

TRUNCATE rankings;
DROP TABLE rankings;
ALTER TABLE rankings_tmp RENAME TO rankings;

ALTER INDEX IF EXISTS rankings_tmp_map_idx RENAME TO idx_rankings_map;
ALTER INDEX IF EXISTS rankings_tmp_name_idx RENAME TO idx_rankings_name;
ALTER INDEX IF EXISTS rankings_tmp_rank_idx RENAME TO idx_rankings_rank_top5;

COMMIT;
