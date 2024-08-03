SELECT name,
    SUM(rankpoints.points) AS "points?",
    MIN(time) AS "time!"
FROM (
    SELECT name, MIN(rank) AS "rank", MIN(time) AS "time" FROM (
        SELECT DENSE_RANK() OVER (ORDER BY min(time)) AS rank,
            name,
            MIN(time) AS "time"
        FROM
            teamrace
        WHERE
            map = $1 AND
            timestamp <= $2 AND
            time <= $3
        GROUP BY
            id,
            name
    ) AS ranks GROUP BY name) AS teamrankings
LEFT JOIN
    rankpoints ON rankpoints.rank = teamrankings.rank
GROUP BY
    name
HAVING
    MIN(teamrankings.rank) <= 20
ORDER BY "time!" ASC
