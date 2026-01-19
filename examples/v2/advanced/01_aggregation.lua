--[[
    Aggregation Pipeline Example - gmsv_mongo V2

    Demonstrates advanced data aggregation and analysis
]]

require("mongo")

local client = MongoDB.Client("mongodb://admin:password@localhost:27017")
if not client then
    print("Connection failed!")
    return
end

local db = client:Database("gameserver")
local players = db:Collection("players")

-- Insert sample data
print("Inserting sample data...")
players:InsertMany({
    { username = "Alice", level = 10, class = "Warrior", score = 1500 },
    { username = "Bob", level = 15, class = "Mage", score = 2000 },
    { username = "Charlie", level = 8, class = "Warrior", score = 1200 },
    { username = "Diana", level = 20, class = "Rogue", score = 2500 },
    { username = "Eve", level = 12, class = "Mage", score = 1800 },
    { username = "Frank", level = 18, class = "Warrior", score = 2200 }
})

print("\n=== Aggregation Examples ===\n")

-- Example 1: Group by class and count
print("1. Players by class:")
local byClass = players:Aggregate({
    {
        ["$group"] = {
            _id = "$class",
            count = { ["$sum"] = 1 },
            avgLevel = { ["$avg"] = "$level" },
            totalScore = { ["$sum"] = "$score" }
        }
    },
    {
        ["$sort"] = { count = -1 }
    }
})

for _, group in ipairs(byClass) do
    print(string.format("   %s: %d players, Avg Level: %.1f, Total Score: %d",
        group._id, group.count, group.avgLevel, group.totalScore))
end

-- Example 2: Top players by score
print("\n2. Top 3 players by score:")
local topPlayers = players:Aggregate({
    {
        ["$sort"] = { score = -1 }
    },
    {
        ["$limit"] = 3
    },
    {
        ["$project"] = {
            username = 1,
            score = 1,
            level = 1,
            _id = 0
        }
    }
})

for i, player in ipairs(topPlayers) do
    print(string.format("   %d. %s - Score: %d (Level %d)",
        i, player.username, player.score, player.level))
end

-- Example 3: Level distribution
print("\n3. Level distribution:")
local levelDist = players:Aggregate({
    {
        ["$bucket"] = {
            groupBy = "$level",
            boundaries = { 0, 10, 15, 20, 25 },
            default = "25+",
            output = {
                count = { ["$sum"] = 1 },
                players = { ["$push"] = "$username" }
            }
        }
    }
})

for _, bucket in ipairs(levelDist) do
    print(string.format("   Level %s: %d players", tostring(bucket._id), bucket.count))
end

-- Example 4: Advanced statistics
print("\n4. Overall statistics:")
local stats = players:Aggregate({
    {
        ["$group"] = {
            _id = '',
            totalPlayers = { ["$sum"] = 1 },
            avgLevel = { ["$avg"] = "$level" },
            avgScore = { ["$avg"] = "$score" },
            minScore = { ["$min"] = "$score" },
            maxScore = { ["$max"] = "$score" }
        }
    }
})

if stats[1] then
    local s = stats[1]
    print(string.format("   Total Players: %d", s.totalPlayers))
    print(string.format("   Average Level: %.2f", s.avgLevel))
    print(string.format("   Average Score: %.2f", s.avgScore))
    print(string.format("   Score Range: %d - %d", s.minScore, s.maxScore))
end

print("\n=== Aggregation Complete ===")
