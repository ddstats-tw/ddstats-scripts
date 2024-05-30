INSERT INTO rankedpoints (
    date,
    name,
    rankpoints,
    teampoints
)
SELECT * FROM UNNEST(
    $1::DATE[],
    $2::VARCHAR(16)[],
    $3::INTEGER[],
    $4::INTEGER[]
)
