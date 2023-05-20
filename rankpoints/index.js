import Database from 'better-sqlite3'
import dotenv from 'dotenv'
import calc from './calc.js';

dotenv.config()

/* load in db using better-sqlite3 */
export const ddnet = new Database('../ddnet.sqlite', { })
export const points = new Database('../points.db', { })

/* WAL mode */
ddnet.pragma('journal_mode = WAL')
points.pragma('journal_mode = WAL')

/* Unsafe mode */
ddnet.unsafeMode()
points.unsafeMode()

console.log("Loaded in 'ddnet.db'!")
console.log("Loaded in 'points.db'!")

console.log("Creating indexes")

// Add timestamp if going from beginning.
ddnet.exec("CREATE INDEX IF NOT EXISTS idx_race_map_time ON race (map, time ASC)")
ddnet.exec("CREATE INDEX IF NOT EXISTS idx_teamrace_map_time ON teamrace (map, time ASC)")

ddnet.exec(`CREATE TABLE IF NOT EXISTS rankings (
        Rank INTEGER NOT NULL,
        Timestamp timestamp NOT NULL, 
        Name VARCHAR(16) NOT NULL,
        Time FLOAT NOT NULL,
        Map VARCHAR(128) NOT NULL,
        Points INTEGER NOT NULL,
        Server CHAR(4) NOT NULL DEFAULT ''
    )`)

ddnet.exec(`CREATE TABLE IF NOT EXISTS teamrankings (
        Rank INTEGER NOT NULL,
        Timestamp timestamp NOT NULL, 
        ID VARBINARY(16) NOT NULL,
        Name VARCHAR(16) NOT NULL,
        Time FLOAT NOT NULL,
        Map VARCHAR(128) NOT NULL,
        Points INTEGER NOT NULL,
        Server CHAR(4) NOT NULL DEFAULT ''
    )`)
points.exec("CREATE TABLE IF NOT EXISTS rankedpoints (date TEXT, player TEXT, rankpoints INTEGER, teampoints INTEGER)")
points.exec("CREATE TABLE IF NOT EXISTS processed (date TEXT)")


const maps = ddnet.prepare(`SELECT * FROM maps`).all()
const date = ddnet.prepare(`
                    SELECT min(timestamp) as 'start', max(timestamp) as 'end' 
                        FROM race`).all()


export let timeCache = []
export let timeCacheTeam = []

BigInt.prototype.toJSON = function () { return this.toString() }

console.log("Date, %s", Date.parse(date[0].start))
for (let d = new Date(Date.parse(date[0].start)); d <= Date.parse(date[0].end); d.setDate(d.getDate() + 1)) {
    ddnet.exec("DELETE FROM teamrankings")
    ddnet.exec("DELETE FROM rankings")
    calc.calculatePoints(d.toISOString().split('T')[0], maps)
}

// done
process.exit()
