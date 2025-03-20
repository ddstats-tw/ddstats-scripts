BEGIN;
DROP TABLE IF EXISTS teamrankings_tmp;
CREATE TABLE IF NOT EXISTS teamrankings_tmp (LIKE teamrankings INCLUDING ALL);

INSERT INTO 
    teamrankings_tmp (
        rank,
        time,
        players,
        timestamp,
        map,
        id,
        server
)
SELECT a.rank,
    a.time,
    ARRAY_AGG(a.name),
    a.timestamp,
    a.map,
    id,
    COALESCE(server, '')
FROM
(
    SELECT DENSE_RANK() OVER (PARTITION BY map ORDER BY min(time)) AS rank,
        MIN(time) as time,
        name,
        timestamp,
        map,
        id,
        (
            SELECT
                server
            FROM 
                race
            WHERE
                map = teamrace.map AND
                name = teamrace.name AND
                time = teamrace.time 
            LIMIT 1
        ) as server
    FROM
        teamrace
    GROUP BY
        map,
        id,
        timestamp,
        name
) AS a
GROUP BY a.map,
    a.id,
    a.rank,
    a.time,
    a.timestamp,
    server;

TRUNCATE teamrankings;
DROP TABLE teamrankings;
ALTER TABLE teamrankings_tmp RENAME TO teamrankings;

ALTER INDEX IF EXISTS teamrankings_tmp_map_idx RENAME TO idx_teamrankings_map;
ALTER INDEX IF EXISTS teamrankings_tmp_players_idx RENAME TO idx_teamrankings_players;
ALTER INDEX IF EXISTS teamrankings_tmp_rank_idx RENAME TO idx_teamrankings_rank_top5;

COMMIT;
