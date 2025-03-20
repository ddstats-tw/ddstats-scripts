BEGIN;
DROP TABLE IF EXISTS players_tmp;
CREATE TABLE IF NOT EXISTS players_tmp (LIKE players INCLUDING ALL);

INSERT INTO players_tmp (name, points, clan, country, skin_name, skin_color_body, skin_color_feet)
SELECT t.name, SUM(points) as points, clan, country, skin_name, skin_color_body, skin_color_feet
FROM (SELECT name, map FROM race GROUP BY name, map)
as t
JOIN maps ON t.map = maps.map
LEFT JOIN (
        SELECT distinct on (name) name, clan, country, skin_name, skin_color_body, skin_color_feet FROM playtime
                WHERE skin_name != '' GROUP BY name, clan, country, skin_name, skin_color_body, skin_color_feet ORDER BY name, COUNT(*) DESC
        ) as t2 ON t.name = t2.name
GROUP BY t.name, clan, country, skin_name, skin_color_body, skin_color_feet
ORDER BY SUM(points) DESC;

TRUNCATE players;
DROP TABLE players;
ALTER TABLE players_tmp RENAME TO players;

ALTER INDEX IF EXISTS players_tmp_name_idx RENAME TO trgm_idx_name;

COMMIT;
