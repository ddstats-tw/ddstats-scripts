INSERT INTO
    maps_playtime (
        map,
        seconds,
        mostaddicted,
        mostaddicted_seconds
)
SELECT playtime_maps.map,
    SUM(playtime_maps.seconds_played),
    mostaddicted.name,
    mostaddicted.seconds_played
FROM playtime_maps
    JOIN (
        SELECT DISTINCT ON (map)
            map,
            name,
            seconds_played
        FROM
            playtime_maps
        WHERE map IN (
            SELECT map FROM maps
        )
        ORDER BY map,
            seconds_played DESC
    ) AS mostaddicted ON mostaddicted.map = playtime_maps.map
WHERE playtime_maps.map IN (
    SELECT map FROM maps
)
GROUP BY
    playtime_maps.map,
    mostaddicted.name,
    mostaddicted.seconds_played
ORDER BY
    mostaddicted.seconds_played DESC

ON CONFLICT (map) DO UPDATE
    SET seconds = excluded.seconds,
        mostaddicted = excluded.mostaddicted,
        mostaddicted_seconds = excluded.mostaddicted_seconds
