import sqlite3

con = sqlite3.connect('../db/playtime.db', isolation_level=None)

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

# this takes forever, please kill me
for map in maps:
    cur.execute("""
        SELECT p.Map, SUM(p.time) AS 'Playtime', mostaddicted.player, mostaddicted.mplaytime 
        FROM record_playtime as p
        JOIN (
            SELECT map, player, SUM(time) as mplaytime 
            FROM record_playtime 
            WHERE map = ? AND player NOT IN ('nameless tee', 'brainless tee', '(connecting)')
            GROUP BY player 
            ORDER BY mplaytime DESC 
        ) as mostaddicted ON mostaddicted.map = p.map
        WHERE p.map = ? AND p.player NOT IN ('nameless tee', 'brainless tee', '(connecting)')
        GROUP BY p.Map
        ORDER BY Playtime DESC;
    """, (map[0], map[0]))

    values = res.fetchone()

    print(values[0])
    cur.execute("INSERT OR REPLACE INTO maps_playtime (map, seconds, mostaddicted, mostaddicted_seconds) VALUES (?, ?, ?, ?)", (values[0], values[1], values[2], values[3]))