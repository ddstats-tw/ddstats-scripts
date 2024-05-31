INSERT INTO playtime_maps_processed (
    date
)
SELECT date 
FROM
    playtime_processed
ON CONFLICT DO
    NOTHING
