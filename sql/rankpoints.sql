SELECT name, SUM(rankpoints.points) AS "points?", MIN(time) AS "time!" FROM (
        SELECT rank, name, time FROM (
        SELECT RANK() OVER (ORDER BY time) AS rank, * FROM (
        SELECT distinct on (name) 
            timestamp,
            name,
            time
        FROM
            race
        WHERE
            map = $1 AND
            timestamp <= $2 AND
            time <= $3
        ORDER BY
            name,
            time
    ) AS ranks ORDER BY time) AS top_ranks
    WHERE rank <= 20
) AS rankings LEFT JOIN 
    rankpoints ON rankpoints.rank = rankings.rank
GROUP BY
    name
ORDER BY "time!"
