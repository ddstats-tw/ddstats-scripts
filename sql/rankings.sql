INSERT INTO
    rankings (
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
) AS ranks ORDER BY time
