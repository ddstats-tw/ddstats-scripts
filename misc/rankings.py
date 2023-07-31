import sqlite3

con = sqlite3.connect('../db/ddnet.sqlite', isolation_level=None)

cur = con.cursor()

cur.execute("""
    CREATE TABLE IF NOT EXISTS rankings (
        Rank INTEGER NOT NULL,
        Timestamp timestamp NOT NULL, 
        Name VARCHAR(16) NOT NULL,
        Time FLOAT NOT NULL,
        Map VARCHAR(128) NOT NULL,
        Server CHAR(4) NOT NULL DEFAULT ''
    )""")

cur.execute("""
    CREATE TABLE IF NOT EXISTS teamrankings (
        Rank INTEGER NOT NULL,
        Timestamp timestamp NOT NULL, 
        ID VARBINARY(16) NOT NULL,
        Name VARCHAR(16) NOT NULL,
        Time FLOAT NOT NULL,
        Map VARCHAR(128) NOT NULL,
        Server CHAR(4) NOT NULL DEFAULT ''
    )""")

cur.execute("""
    CREATE TABLE rankpoints(
        Rank INTEGER NOT NULL,
        Points INTEGER NOT NULL,
        PRIMARY KEY(Rank)
    )
""")

cur.execute("""
    INSERT INTO rankpoints (rank, points) VALUES
        (1, 25),
        (2, 18),
        (3, 15),
        (4, 12),
        (5, 10),
        (6, 8),
        (7, 6),
        (8, 4),
        (9, 2),
        (10, 1);
""")

cur.execute("PRAGMA journal_mode = OFF")
cur.execute("PRAGMA synchronous = 0;")
cur.execute("PRAGMA cache_size = 100000;")
cur.execute("PRAGMA locking_mode = EXCLUSIVE;")
cur.execute("PRAGMA temp_store = MEMORY;")

print("Processing rankings")

cur.execute("""
    INSERT INTO rankings (rank, time, name, timestamp, map, server)
    SELECT RANK() OVER (PARTITION BY map ORDER BY min(time)) AS rank,
        MIN(time) as 'time',
        name,
        timestamp,
        map,
        server
    FROM race
    GROUP BY map, name
""")

print("Processing teamrankings")

cur.execute("""
    INSERT INTO teamrankings (rank, time, name, timestamp, map, id, server)
    SELECT a.rank, a.time, a.name, a.timestamp, a.map, id, race.server FROM 
    (
        SELECT  DENSE_RANK() OVER (PARTITION BY map ORDER BY min(time)) AS rank,
                MIN(time) as 'time',
                name,
                timestamp,
                map,
                id
        FROM   teamrace
        GROUP  BY map, id, name
    ) AS a JOIN race AS race ON a.name = race.name AND a.map = race.map AND a.time = race.time
""")