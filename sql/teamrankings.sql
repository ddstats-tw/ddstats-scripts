INSERT INTO 
    teamrankings (
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
    server
