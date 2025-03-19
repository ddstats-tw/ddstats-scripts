/* 2024-02-04, RENAME 'pisisik' -> '63 turtles' */
UPDATE rankedpoints SET name = '63 turtles' WHERE name = 'pisisik';
UPDATE playtime SET name = '63 turtles' WHERE name = 'pisisik';
UPDATE maps_playtime SET mostaddicted = '63 turtles' WHERE mostaddicted = 'pisisik';
DELETE FROM playtime_maps WHERE name IN ('63 turtles', 'pisisik');
INSERT INTO playtime_maps (map, name, seconds_played)
SELECT map, '63 turtles', SUM(time) AS "seconds_played" FROM playtime
    WHERE name IN ('63 turtles', 'pisisik') GROUP BY map;

/* 2024-06-03, RENAME GAMETYPE 'F-DDrace idm' -> 'F-DDrace'  */
UPDATE playtime SET gametype = 'F-DDrace' WHERE gametype = 'F-DDrace idm';

/* 2024-11-25, RENAME 'JETFiRE...' -> 'exosphere' */
UPDATE rankedpoints SET name = 'exosphere' WHERE name = 'JETFiRE...';
UPDATE playtime SET name = 'exosphere' WHERE name = 'JETFiRE...';
UPDATE maps_playtime SET mostaddicted = 'exosphere' WHERE mostaddicted = 'JETFiRE...';
DELETE FROM playtime_maps WHERE name IN ('exosphere', 'JETFiRE...');
INSERT INTO playtime_maps (map, name, seconds_played)
SELECT map, 'exosphere', SUM(time) AS "seconds_played" FROM playtime
    WHERE name IN ('exosphere', 'JETFiRE...') GROUP BY map;
