---
title: Server Monitor
description: Lua example for server monitor with gmsv_mongo
navigation:
  title: Server Monitor
---

# Server Monitor

::info-box{type="important"}
These examples were AI-generated and have not been reviewed for accuracy. Use them as a starting point and verify correctness before deploying in production.
::

## Implementation

```lua
--[[
    Server Monitor
    Track server performance and player activity
]]

local Monitor = {}

function Monitor.Initialize(db)
    Monitor.db = db
    Monitor.snapshots = db:Collection("server_snapshots")
    Monitor.playerSessions = db:Collection("player_sessions")

    -- Indexes
    Monitor.snapshots:CreateIndex({ timestamp = -1 }, false, "time_desc")
    Monitor.playerSessions:CreateIndex({ steamid = 1, joined_at = -1 }, false, "player_sessions")
end

-- Record server snapshot
function Monitor.RecordSnapshot()
    local snapshot = {
        timestamp = os.time(),
        player_count = player and #player.GetAll() or 0,
        map = game and game.GetMap() or "unknown",
        hostname = GetHostName and GetHostName() or "unknown",
        -- Add more metrics as needed
    }

    Monitor.snapshots:InsertOneAsync(snapshot, function() end)
end

-- Get server statistics
function Monitor.GetStats(hours, callback)
    hours = hours or 24
    local since = os.time() - (hours * 3600)

    Monitor.snapshots:AggregateAsync({
        {
            ["$match"] = { timestamp = { ["$gte"] = since } }
        },
        {
            ["$group"] = {
                _id = nil,
                avg_players = { ["$avg"] = "$player_count" },
                max_players = { ["$max"] = "$player_count" },
                min_players = { ["$min"] = "$player_count" },
                snapshots = { ["$sum"] = 1 }
            }
        }
    }, function(err, results)
        if results and #results > 0 then
            callback(results[1])
        else
            callback({ avg_players = 0, max_players = 0, min_players = 0, snapshots = 0 })
        end
    end)
end

-- Track player session
function Monitor.PlayerJoined(steamid, username)
    Monitor.playerSessions:InsertOneAsync({
        steamid = steamid,
        username = username,
        joined_at = os.time(),
        left_at = nil,
        duration = nil
    })
end

function Monitor.PlayerLeft(steamid)
    Monitor.playerSessions:UpdateOneAsync(
        { steamid = steamid, left_at = nil },
        {
            ["$set"] = { left_at = os.time() }
        }
    )
end

-- Get player activity
function Monitor.GetPlayerActivity(steamid, days, callback)
    days = days or 30
    local since = os.time() - (days * 24 * 3600)

    Monitor.playerSessions:AggregateAsync({
        {
            ["$match"] = {
                steamid = steamid,
                joined_at = { ["$gte"] = since }
            }
        },
        {
            ["$group"] = {
                _id = nil,
                sessions = { ["$sum"] = 1 },
                total_time = {
                    ["$sum"] = {
                        ["$subtract"] = {
                            { ["$ifNull"] = { "$left_at", os.time() } },
                            "$joined_at"
                        }
                    }
                },
                first_seen = { ["$min"] = "$joined_at" },
                last_seen = { ["$max"] = "$joined_at" }
            }
        }
    }, function(err, results)
        if results and #results > 0 then
            callback(results[1])
        else
            callback({ sessions = 0, total_time = 0 })
        end
    end)
end

-- Get peak hours
function Monitor.GetPeakHours(days, callback)
    days = days or 7
    local since = os.time() - (days * 24 * 3600)

    Monitor.snapshots:AggregateAsync({
        {
            ["$match"] = { timestamp = { ["$gte"] = since } }
        },
        {
            ["$project"] = {
                hour = { ["$mod"] = {
                    { ["$floor"] = { ["$divide"] = { "$timestamp", 3600 } } },
                    24
                }},
                player_count = 1
            }
        },
        {
            ["$group"] = {
                _id = "$hour",
                avg_players = { ["$avg"] = "$player_count" }
            }
        },
        { ["$sort"] = { avg_players = -1 } }
    }, function(err, results)
        callback(results or {})
    end)
end

return Monitor
```
