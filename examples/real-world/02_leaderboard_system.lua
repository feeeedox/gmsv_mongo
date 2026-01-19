--[[
    Leaderboard System - Real World Example

    A dynamic leaderboard system with rankings and statistics
]]

require("mongo")

local client = MongoDB.Client("mongodb://admin:password@localhost:27017")
if not client then
    error("Failed to connect to MongoDB")
end

local db = client:Database("gameserver")
local leaderboards = db:Collection("leaderboards")

-- Create indexes
leaderboards:CreateIndex({ score = -1 }, false, "score_desc")
leaderboards:CreateIndex({ steamid = 1 }, true, "steamid_unique")
leaderboards:CreateIndex({ updated_at = -1 }, false, "updated_desc")

-- Leaderboard System
local Leaderboard = {}

--- Update player score
function Leaderboard.UpdateScore(steamid, username, score, stats)
    stats = stats or {}

    local updated = leaderboards:UpdateOne(
        { steamid = steamid },
        {
            ["$set"] = {
                username = username,
                score = score,
                kills = stats.kills or 0,
                deaths = stats.deaths or 0,
                wins = stats.wins or 0,
                losses = stats.losses or 0,
                updated_at = os.time()
            },
            ["$setOnInsert"] = {
                created_at = os.time(),
                total_matches = 0
            },
            ["$inc"] = {
                total_matches = 1
            }
        },
        true  -- upsert
    )

    print(string.format("Updated score for %s: %d points", username, score))
end

--- Get top N players
function Leaderboard.GetTop(limit)
    limit = limit or 10

    local topPlayers = leaderboards:Find({}, limit)
    table.sort(topPlayers, function(a, b)
        return a.score > b.score
    end)

    return topPlayers
end

--- Get player rank
function Leaderboard.GetRank(steamid)
    local player = leaderboards:FindOne({ steamid = steamid })
    if not player then
        return nil
    end

    -- Count players with higher score
    local higherScores = leaderboards:Count({
        score = { ["$gt"] = player.score }
    })

    return higherScores + 1
end

--- Get player statistics
function Leaderboard.GetStats(steamid)
    local player = leaderboards:FindOne({ steamid = steamid })
    if not player then
        return nil
    end

    local kd_ratio = 0
    if player.deaths > 0 then
        kd_ratio = player.kills / player.deaths
    else
        kd_ratio = player.kills
    end

    local win_rate = 0
    if player.total_matches > 0 then
        win_rate = (player.wins / player.total_matches) * 100
    end

    return {
        score = player.score,
        kills = player.kills,
        deaths = player.deaths,
        kd_ratio = kd_ratio,
        wins = player.wins,
        losses = player.losses,
        win_rate = win_rate,
        total_matches = player.total_matches
    }
end

--- Get leaderboard statistics
function Leaderboard.GetGlobalStats()
    local stats = leaderboards:Aggregate({
        {
            ["$group"] = {
                _id = nil,
                total_players = { ["$sum"] = 1 },
                avg_score = { ["$avg"] = "$score" },
                max_score = { ["$max"] = "$score" },
                total_kills = { ["$sum"] = "$kills" },
                total_matches = { ["$sum"] = "$total_matches" }
            }
        }
    })

    return stats[1]
end

--- Get score distribution
function Leaderboard.GetScoreDistribution()
    local distribution = leaderboards:Aggregate({
        {
            ["$bucket"] = {
                groupBy = "$score",
                boundaries = { 0, 1000, 5000, 10000, 50000, 100000 },
                default = "100000+",
                output = {
                    count = { ["$sum"] = 1 },
                    avg_score = { ["$avg"] = "$score" }
                }
            }
        }
    })

    return distribution
end

-- Demo: Leaderboard System
print("=== Leaderboard System Demo ===\n")

-- Add sample players
print("1. Adding players to leaderboard")
Leaderboard.UpdateScore("STEAM_0:1:111", "ProGamer", 15000, { kills = 150, deaths = 45, wins = 30, losses = 15 })
Leaderboard.UpdateScore("STEAM_0:1:222", "CasualPlayer", 5000, { kills = 80, deaths = 70, wins = 15, losses = 30 })
Leaderboard.UpdateScore("STEAM_0:1:333", "Newbie", 1200, { kills = 25, deaths = 60, wins = 5, losses = 40 })
Leaderboard.UpdateScore("STEAM_0:1:444", "VeteranPlayer", 25000, { kills = 300, deaths = 80, wins = 50, losses = 10 })
Leaderboard.UpdateScore("STEAM_0:1:555", "EliteGamer", 50000, { kills = 500, deaths = 100, wins = 80, losses = 20 })

-- Show top players
print("\n2. Top 5 Players:")
local topPlayers = Leaderboard.GetTop(5)
for i, player in ipairs(topPlayers) do
    print(string.format("   %d. %s - %d points (K/D: %.2f)",
        i, player.username, player.score,
        player.deaths > 0 and (player.kills / player.deaths) or player.kills))
end

-- Show player rank and stats
print("\n3. Player Profile - ProGamer:")
local steamid = "STEAM_0:1:111"
local rank = Leaderboard.GetRank(steamid)
local stats = Leaderboard.GetStats(steamid)

if rank and stats then
    print(string.format("   Rank: #%d", rank))
    print(string.format("   Score: %d", stats.score))
    print(string.format("   K/D Ratio: %.2f", stats.kd_ratio))
    print(string.format("   Win Rate: %.1f%%", stats.win_rate))
    print(string.format("   Total Matches: %d", stats.total_matches))
end

-- Global statistics
print("\n4. Global Leaderboard Statistics:")
local globalStats = Leaderboard.GetGlobalStats()
if globalStats then
    print(string.format("   Total Players: %d", globalStats.total_players))
    print(string.format("   Average Score: %.0f", globalStats.avg_score))
    print(string.format("   Highest Score: %d", globalStats.max_score))
    print(string.format("   Total Kills: %d", globalStats.total_kills))
    print(string.format("   Total Matches: %d", globalStats.total_matches))
end

-- Score distribution
print("\n5. Score Distribution:")
local distribution = Leaderboard.GetScoreDistribution()
for _, bucket in ipairs(distribution) do
    local range = tostring(bucket._id)
    print(string.format("   %s: %d players (avg: %.0f)",
        range, bucket.count, bucket.avg_score))
end

print("\n=== Demo Complete ===")
