---
title: Player Data System
description: Lua example for player data system with gmsv_mongo
navigation:
  title: Player Data System
---

# Player Data System

::info-box{type="important"}
These examples were AI-generated and have not been reviewed for accuracy. Use them as a starting point and verify correctness before deploying in production.
::

## Implementation

A complete player data management system:

```lua
--[[
    Player Data System
    Handles player registration, loading, and saving
]]

local PlayerData = {}
PlayerData.client = nil
PlayerData.db = nil

-- Initialize the system
function PlayerData.Initialize(connectionString, dbName)
    PlayerData.client = MongoDB.Client(connectionString)
    if not PlayerData.client then
        print("[PlayerData] Failed to connect to MongoDB!")
        return false
    end

    PlayerData.db = PlayerData.client:Database(dbName)

    -- Setup indexes
    local players = PlayerData.db:Collection("players")
    players:CreateIndex({ steamid = 1 }, true, "steamid_unique")
    players:CreateIndex({ last_login = -1 }, false, "recent_login")

    print("[PlayerData] System initialized")
    return true
end

-- Get player collection
function PlayerData.GetCollection()
    return PlayerData.db:Collection("players")
end

-- Load player data
function PlayerData.Load(steamid, callback)
    local col = PlayerData.GetCollection()

    col:FindOneAsync({ steamid = steamid }, function(err, data)
        if err then
            print("[PlayerData] Load error:", err)
            callback(nil, err)
            return
        end

        if data then
            -- Update last login
            col:UpdateOneAsync(
                { steamid = steamid },
                { ["$set"] = { last_login = os.time() } },
                function() end
            )
            callback(data, nil)
        else
            -- Create new player
            local newPlayer = {
                steamid = steamid,
                level = 1,
                experience = 0,
                credits = 1000,
                playtime = 0,
                created_at = os.time(),
                last_login = os.time(),
                stats = {
                    kills = 0,
                    deaths = 0,
                    assists = 0
                },
                settings = {
                    notifications = true,
                    music = true
                },
                inventory = {}
            }

            col:InsertOneAsync(newPlayer, function(err, id)
                if err then
                    callback(nil, err)
                else
                    newPlayer._id = id
                    callback(newPlayer, nil)
                end
            end)
        end
    end)
end

-- Save player data
function PlayerData.Save(steamid, data, callback)
    local col = PlayerData.GetCollection()

    col:UpdateOneAsync(
        { steamid = steamid },
        { ["$set"] = data },
        function(err, modified)
            if callback then
                callback(err, modified > 0)
            end
        end
    )
end

-- Update specific fields
function PlayerData.Update(steamid, updates, callback)
    local col = PlayerData.GetCollection()

    col:UpdateOneAsync(
        { steamid = steamid },
        updates,
        function(err, modified)
            if callback then
                callback(err, modified > 0)
            end
        end
    )
end

-- Add experience and handle level up
function PlayerData.AddExperience(steamid, amount, callback)
    local col = PlayerData.GetCollection()

    -- Get current data
    col:FindOneAsync({ steamid = steamid }, function(err, player)
        if err or not player then
            if callback then callback(false, 0) end
            return
        end

        local newExp = player.experience + amount
        local newLevel = player.level
        local expRequired = newLevel * 1000

        -- Level up logic
        while newExp >= expRequired do
            newExp = newExp - expRequired
            newLevel = newLevel + 1
            expRequired = newLevel * 1000
        end

        local leveledUp = newLevel > player.level

        col:UpdateOneAsync(
            { steamid = steamid },
            {
                ["$set"] = {
                    experience = newExp,
                    level = newLevel
                }
            },
            function(err)
                if callback then
                    callback(leveledUp, newLevel)
                end
            end
        )
    end)
end

-- Get player stats
function PlayerData.GetStats(steamid, callback)
    local col = PlayerData.GetCollection()

    col:FindOneAsync(
        { steamid = steamid },
        function(err, player)
            if err or not player then
                callback(nil)
            else
                callback(player.stats)
            end
        end
    )
end

-- Increment stat
function PlayerData.IncrementStat(steamid, stat, amount)
    local col = PlayerData.GetCollection()

    col:UpdateOneAsync(
        { steamid = steamid },
        { ["$inc"] = { ["stats." .. stat] = amount or 1 } },
        function() end
    )
end

return PlayerData
```