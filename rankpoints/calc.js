import { ddnet, points, timeCache, timeCacheTeam } from './index.js'

const rankPointsMap = {
    1: 25,
    2: 18,
    3: 15,
    4: 12,
    5: 10,
    6: 8,
    7: 6,
    8: 4,
    9: 2,
    10: 1
}

function rankPointsAmount(rank, category) {
    if(category == 'Fun')
        return 0
    if (rankPointsMap[rank])
        return rankPointsMap[rank]

    return 0
}

function insertRankings(rankings, teamrankings, map) {
    for(const rank of rankings) {
        if(rank.rank > 10)
            continue

        ddnet.prepare(`INSERT INTO rankings (rank, timestamp, name, time, map, points, server)
                            VALUES(?, ?, ?, ?, ?, ?, ?)`).run([
            rank.rank, rank.timestamp, rank.name, rank.time, rank.map, 
            rankPointsAmount(rank.rank, map.Server), rank.server])
    }
    for(const rank of teamrankings) {
        if(rank.rank > 10)
            continue

        ddnet.prepare(`INSERT INTO teamrankings (rank, timestamp, name, time, map, points, server, id)
                            VALUES(?, ?, ?, ?, ?, ?, ?, ?)`).run([
            rank.rank, rank.timestamp, rank.name, rank.time, rank.map, 
            rankPointsAmount(rank.rank, map.Server), rank.Server, rank.id])
    }
}

function insertPoints(date, leaderboard) {
    for (const player in leaderboard) {
        points.prepare(`INSERT INTO rankedpoints (date, player, rankpoints, teampoints)
                    VALUES(?, ?, ?, ?)`).run([date, leaderboard[player].name,
            leaderboard[player].rankpoints, leaderboard[player].teampoints])
    }
    points.prepare(`INSERT INTO processed (date) VALUES(?)`).run([date])
}

function calculatePoints(date, maps) {
    // if statement to check if date is already processed
    if(points.prepare(`SELECT * FROM processed WHERE date = ?`).get(date)) {
        console.log(`Skipping: %s (already processed)`, date)
        return 0
    }

    let rankPoints = {}
    console.log(`Processing: %s`, date)

    for (const map of maps) {
        if (timeCache[map.Map] === undefined) {
            timeCache[map.Map] = Math.pow(10, 9) // ew
            timeCacheTeam[map.Map] = Math.pow(10, 9)
        }

        // Skip maps not released
        if (Date.parse(map.Timestamp) > Date.parse(date)) {
            continue
        }

        const rankings = ddnet.prepare(`
            SELECT * FROM 
            (
                SELECT  RANK() OVER (ORDER BY min(time)) AS rank,
                        MIN(time) as 'time',
                        name,
                        timestamp,
                        map,
                        server
                FROM   race
                    WHERE map = ?
                    AND time <= ?
                    AND timestamp <= ?
                GROUP  BY name
                ORDER  BY rank ASC
            ) AS a
            WHERE  a.rank <= 50
        `).all([map.Map, timeCache[map.Map], date])

        const teamrankings = ddnet.prepare(`
            SELECT a.*, race.server FROM 
            (
                SELECT  DENSE_RANK() OVER (ORDER BY min(time)) AS rank,
                        MIN(time) as 'time',
                        name,
                        timestamp,
                        map,
                        id
                FROM   teamrace
                    WHERE map = ?
                    AND time <= ?
                    AND timestamp <= ?
                GROUP  BY name
                ORDER  BY rank ASC
            ) AS a JOIN race AS race ON a.name = race.name AND a.map = race.map AND a.time = race.time
            WHERE  a.rank <= 50
        `).all([map.Map, timeCacheTeam[map.Map], date])

        // Rank points
        for (const rank of rankings) {
            if (rank.rank > 10)
                break;

            if (rankPoints[rank.name] === undefined)
                rankPoints[rank.name] = { name: rank.name, teampoints: 0, rankpoints: 0 }

            rankPoints[rank.name].rankpoints += rankPointsAmount(rank.rank)
        }

        if (rankings[rankings.length - 1] != undefined)
            if (rankings[rankings.length - 1].rank >= 40)
                timeCache[map.Map] = rankings[rankings.length - 1].time

        // Team points
        for (const rank of teamrankings) {
            if (rank.rank > 10)
                break;

            if (rankPoints[rank.name] === undefined)
                rankPoints[rank.name] = { name: rank.name, teampoints: 0, rankpoints: 0 }

            rankPoints[rank.name].teampoints += rankPointsAmount(rank.rank)
        }

        if (teamrankings[teamrankings.length - 1] != undefined)
            if (teamrankings[teamrankings.length - 1].rank >= 40)
                timeCacheTeam[map.Map] = teamrankings[teamrankings.length - 1].time

        // Store the top10 for later use
        insertRankings(rankings, teamrankings, map)
    }
    console.log(`Inserting: %d rows`, Object.keys(rankPoints).length)
    insertPoints(date, rankPoints)
}

export default {
    calculatePoints,
    rankPointsAmount
}
