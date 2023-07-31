import sqlite3
import redis

def connect_to_db():
    try:
        con = sqlite3.connect('../db/ddnet.sqlite', isolation_level=None)
        cur = con.cursor()
        return con, cur
    except Exception as e:
        print(f"Error connecting to database: {e}")
        return None, None

def connect_to_redis():
    try:
        r = redis.Redis(host='localhost', port=6379, decode_responses=True)
        return r
    except Exception as e:
        print(f"Error connecting to Redis: {e}")
        return None

def execute_query(cur, query):
    try:
        res = cur.execute(query)
        results = res.fetchall()
        print(len(results))
        return results
    except Exception as e:
        print(f"Error executing query: {e}")
        return None

def store_in_redis(r, type, results):
    try:
        pipe = r.pipeline()
        for row in results:
            pipe.zadd(f"leaderboard:{type}:{row[0]}", { row[1]: row[2]})
            pipe.hset(f"player:{row[1]}:{type}", row[0], row[3])
        pipe.execute()
    except Exception as e:
        print(f"Error storing in Redis: {e}")

def main():
    con, cur = connect_to_db()
    r = connect_to_redis()
    if con and cur and r:
        with con:
            r.flushall()

            points = """
                SELECT maps.Server, rankings.Name, SUM(maps.Points), RANK() OVER (PARTITION BY maps.Server ORDER BY SUM(maps.Points) DESC) FROM rankings
                    JOIN maps ON maps.Map = rankings.Map
                GROUP BY rankings.Name, maps.Server;
            """
            results1 = execute_query(cur, points)
            store_in_redis(r, 'points', results1)

            rankpoints = """
                SELECT maps.Server, rankings.Name, SUM(rankpoints.points), RANK() OVER (PARTITION BY maps.Server ORDER BY SUM(rankpoints.points) DESC) FROM rankings
                    JOIN maps ON maps.Map = rankings.Map JOIN rankpoints ON rankings.rank = rankpoints.rank
                    WHERE rankings.rank <= 10
                GROUP BY rankings.Name, maps.Server;
            """
            results2 = execute_query(cur, rankpoints)
            store_in_redis(r, 'rankpoints', results2)

            teampoints = """
                SELECT maps.Server, r.name, SUM(rankpoints.points), RANK() OVER (PARTITION BY maps.Server ORDER BY SUM(rankpoints.points) DESC) FROM (
                    SELECT Map, MIN(rank) as rank, Name FROM teamrankings 
                        WHERE rank <= 10
                    GROUP BY Name, Map
                ) as r
                    JOIN maps ON maps.Map = r.Map JOIN rankpoints ON r.rank = rankpoints.rank
                GROUP BY r.name, maps.Server
            """
            results3 = execute_query(cur, teampoints)
            store_in_redis(r, 'teampoints', results3)

if __name__ == "__main__":
    main()