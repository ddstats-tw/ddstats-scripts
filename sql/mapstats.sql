INSERT INTO mapstats (map, finishes, finishes_rank, median_time, top_time, top_time_team)

SELECT maps.map,
        COALESCE(finishes, 0),
        RANK() OVER (ORDER BY finishes DESC NULLS LAST) as finishes_rank,
        median_time,
        top_time.time,
        top_time_team.time
FROM
        maps
LEFT JOIN (
    SELECT map, COUNT(*) AS finishes,
    ROUND(percentile_cont(0.5) WITHIN GROUP (ORDER BY race.time)::numeric, 2) as median_time
    FROM race
    GROUP BY map
) AS race ON maps.map = race.map 
LEFT JOIN (
        SELECT map, min(time) as "time" FROM race GROUP BY map
) AS top_time ON top_time.map = maps.map
LEFT JOIN (
        SELECT map, min(time) as "time" FROM teamrace GROUP BY map
) AS top_time_team ON top_time_team.map = maps.map

ON CONFLICT (map) DO UPDATE
    SET finishes = excluded.finishes,
        finishes_rank = excluded.finishes_rank,
        median_time = excluded.median_time,
        top_time = excluded.top_time,
        top_time_team = excluded.top_time_team
