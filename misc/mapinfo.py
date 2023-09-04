import sqlite3

con = sqlite3.connect('../db/ddnet.sqlite', isolation_level=None)
cur = con.cursor()

cur.execute("""
    CREATE TABLE IF NOT EXISTS mapstats(
        map VARCHAR(128) NOT NULL PRIMARY KEY,
        finishes INTEGER NOT NULL DEFAULT 0,
        finishesrank INTEGER,
        mediantime INTEGER,
        toptime INTEGER,
        toptimeteam INTEGER
    )
""")

res = cur.execute("SELECT map FROM maps")
maps = res.fetchall()

for map in maps:
    cur.execute("""
        SELECT race.map, COUNT(*), median.time, MIN(race.time), (SELECT min(time) FROM teamrace WHERE map = ?) FROM race LEFT JOIN 
            (
                SELECT map, time FROM race WHERE map = ? ORDER BY time
                LIMIT 1 OFFSET (SELECT COUNT(*) FROM race WHERE map = ?) / 2
            ) AS median ON race.map = median.map 
        WHERE race.map = ?
    """, (map[0], map[0], map[0], map[0]))

    values = res.fetchone()

    print(map[0])
    cur.execute("INSERT OR REPLACE INTO mapstats (map, finishes, mediantime, toptime, toptimeteam) VALUES (?, ?, ?, ?, ?)", (map[0], values[1], values[2], values[3], values[4]))

cur.execute("UPDATE mapstats SET finishesRank = t.rank FROM (SELECT map, RANK() OVER (ORDER BY finishes DESC) as rank FROM mapstats) AS t WHERE mapstats.map = t.map")