import sqlite3

con = sqlite3.connect('../db/master.db', isolation_level=None)

cur = con.cursor()

cur.execute("""
    CREATE TABLE IF NOT EXISTS maps_playtime (
        map VARCHAR(128) PRIMARY KEY NOT NULL,
        seconds INTEGER NOT NULL DEFAULT 0,
        mostaddicted VARCHAR(32) NOT NULL,
        mostaddicted_seconds INTEGER NOT NULL DEFAULT 0
    )""")

res = cur.execute("SELECT map FROM maps")
maps = res.fetchall()

for map in maps:
    cur.execute("""
        SELECT p.Map, SUM(p.time) AS 'Playtime', mostaddicted.name, mostaddicted.mplaytime FROM record_snapshot as p
            JOIN (SELECT map, name, SUM(time) as mplaytime FROM record_snapshot 
                WHERE map = ? AND name != 'nameless tee' AND name != 'brainless tee' AND name != '(connecting)' AND name != '.'
                GROUP BY name ORDER BY SUM(time) DESC LIMIT 1
            ) as mostaddicted ON mostaddicted.map = p.map
            WHERE p.map = ? AND p.name != 'nameless tee' AND p.name != 'brainless tee' AND p.name != '(connecting)' AND p.name != '.'
        ORDER BY Playtime DESC;
    """, (map[0], map[0]))

    values = res.fetchone()

    print(values[0])
    cur.execute("INSERT OR REPLACE INTO maps_playtime (map, seconds, mostaddicted, mostaddicted_seconds) VALUES (?, ?, ?, ?)", (values[0], values[1], values[2], values[3]))