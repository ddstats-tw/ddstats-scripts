INSERT INTO teamrace (
    map,
    name, 
    time, 
    timestamp,
    id
)
SELECT map, name, time, timestamp, decode(id, 'hex') FROM UNNEST(
    $1::VARCHAR(128)[],
    $2::VARCHAR(16)[],
    $3::FLOAT[],
    $4::TIMESTAMP[],
    $5::TEXT[]
) AS t(map, name, time, timestamp, id)
