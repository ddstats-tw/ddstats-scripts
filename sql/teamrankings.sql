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
    race.server 
FROM
(
    SELECT DENSE_RANK() OVER (PARTITION BY map ORDER BY min(time)) AS rank,
        MIN(time) as time,
        name,
        timestamp,
        map,
        id
    FROM
        teamrace
    GROUP BY
        map,
        id,
        timestamp,
        name
) AS a
JOIN race AS race
    ON a.name = race.name AND 
        a.map = race.map AND 
        a.time = race.time AND
        a.timestamp = race.timestamp
GROUP BY a.map,
    a.id,
    a.rank,
    a.time,
    a.timestamp,
    race.server
