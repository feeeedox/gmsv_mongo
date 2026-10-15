---
title: Achievement System
description: Lua example for achievement system with gmsv_mongo
navigation:
  title: Achievement System
---

# Achievement System

::info-box{type="important"}
These examples were AI-generated and have not been reviewed for accuracy. Use them as a starting point and verify correctness before deploying in production.
::

## Implementation

```lua
--[[
    Achievement System
    Track and award player achievements
]]

local Achievements = {}

-- Define achievements
Achievements.Definitions = {
    first_kill = {
        name = "First Blood",
        description = "Get your first kill",
        points = 10
    },
    kill_streak_5 = {
        name = "On Fire",
        description = "Get a 5 kill streak",
        points = 25
    },
    level_10 = {
        name = "Rising Star",
        description = "Reach level 10",
        points = 50
    },
    level_50 = {
        name = "Veteran",
        description = "Reach level 50",
        points = 100
    },
    playtime_10h = {
        name = "Dedicated",
        description = "Play for 10 hours",
        points = 75
    },
    credits_100k = {
        name = "Rich",
        description = "Accumulate 100,000 credits",
        points = 100
    }
}

function Achievements.Initialize(db)
    Achievements.db = db
    Achievements.col = db:Collection("player_achievements")

    Achievements.col:CreateIndex({ steamid = 1 }, false, "steamid_index")
end

-- Check if player has achievement
function Achievements.Has(steamid, achievementId, callback)
    Achievements.col:FindOneAsync(
        { steamid = steamid },
        function(err, doc)
            if doc and doc.achievements then
                for _, ach in ipairs(doc.achievements) do
                    if ach.id == achievementId then
                        callback(true, ach)
                        return
                    end
                end
            end
            callback(false, nil)
        end
    )
end

-- Award achievement
function Achievements.Award(steamid, achievementId, callback)
    local def = Achievements.Definitions[achievementId]
    if not def then
        if callback then callback(false, "Invalid achievement") end
        return
    end

    -- Check if already has it
    Achievements.Has(steamid, achievementId, function(has)
        if has then
            if callback then callback(false, "Already earned") end
            return
        end

        local achievement = {
            id = achievementId,
            name = def.name,
            points = def.points,
            earned_at = os.time()
        }

        Achievements.col:UpdateOneAsync(
            { steamid = steamid },
            {
                ["$push"] = { achievements = achievement },
                ["$inc"] = { total_points = def.points }
            },
            function(err)
                if callback then callback(not err, achievement) end
            end
        )
    end)
end

-- Get player achievements
function Achievements.GetAll(steamid, callback)
    Achievements.col:FindOneAsync(
        { steamid = steamid },
        function(err, doc)
            if doc then
                callback(doc.achievements or {}, doc.total_points or 0)
            else
                callback({}, 0)
            end
        end
    )
end

-- Get achievement progress/stats
function Achievements.GetProgress(steamid, callback)
    Achievements.GetAll(steamid, function(earned, points)
        local total = 0
        local totalPoints = 0

        for _, def in pairs(Achievements.Definitions) do
            total = total + 1
            totalPoints = totalPoints + def.points
        end

        callback({
            earned = #earned,
            total = total,
            points = points,
            max_points = totalPoints,
            percentage = (#earned / total) * 100
        })
    end)
end

-- Check achievements based on player stats
function Achievements.CheckAndAward(steamid, playerData, callback)
    local awarded = {}

    local function checkNext(checks, index)
        if index > #checks then
            callback(awarded)
            return
        end

        local check = checks[index]
        if check.condition then
            Achievements.Award(steamid, check.id, function(success, ach)
                if success then
                    table.insert(awarded, ach)
                end
                checkNext(checks, index + 1)
            end)
        else
            checkNext(checks, index + 1)
        end
    end

    local checks = {
        { id = "first_kill", condition = playerData.stats.kills >= 1 },
        { id = "level_10", condition = playerData.level >= 10 },
        { id = "level_50", condition = playerData.level >= 50 },
        { id = "playtime_10h", condition = playerData.playtime >= 36000 },
        { id = "credits_100k", condition = playerData.credits >= 100000 }
    }

    checkNext(checks, 1)
end

return Achievements
```
