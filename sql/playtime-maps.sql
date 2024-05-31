INSERT INTO playtime_maps (
    map,
    name,
    seconds_played
)
SELECT map,
    name,
    SUM(time) AS "seconds_played"
FROM
    playtime
WHERE
    date NOT IN (SELECT date FROM playtime_maps_processed)
GROUP BY
    map,
    name
ON CONFLICT (map, name) DO
    UPDATE SET
        seconds_played = playtime_maps.seconds_played + excluded.seconds_played
