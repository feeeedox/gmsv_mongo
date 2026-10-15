---
title: Logging System
description: Lua example for logging system with gmsv_mongo
navigation:
  title: Logging System
---

# Logging System

::info-box{type="important"}
These examples were AI-generated and have not been reviewed for accuracy. Use them as a starting point and verify correctness before deploying in production.
::

## Implementation

```lua
--[[
    Logging System
    Server event logging and analytics
]]

local Logger = {}

Logger.Levels = {
    DEBUG = 0,
    INFO = 1,
    WARN = 2,
    ERROR = 3
}

Logger.Categories = {
    "player",
    "economy",
    "combat",
    "admin",
    "system",
    "chat"
}

function Logger.Initialize(db)
    Logger.db = db
    Logger.logs = db:Collection("logs")

    -- Indexes for efficient querying
    Logger.logs:CreateIndex({ timestamp = -1 }, false, "time_desc")
    Logger.logs:CreateIndex({ category = 1, timestamp = -1 }, false, "category_time")
    Logger.logs:CreateIndex({ steamid = 1, timestamp = -1 }, false, "player_time")
    Logger.logs:CreateIndex({ level = 1 }, false, "level_index")
end

-- Log an event
function Logger.Log(level, category, message, data)
    local entry = {
        level = level,
        category = category,
        message = message,
        data = data or {},
        timestamp = os.time(),
        server_id = GetHostName and GetHostName() or "unknown"
    }

    -- Fire and forget - don't block for logging
    Logger.logs:InsertOneAsync(entry, function(err)
        if err then
            print("[Logger] Failed to log:", err)
        end
    end)
end

-- Convenience methods
function Logger.Debug(category, message, data)
    Logger.Log(Logger.Levels.DEBUG, category, message, data)
end

function Logger.Info(category, message, data)
    Logger.Log(Logger.Levels.INFO, category, message, data)
end

function Logger.Warn(category, message, data)
    Logger.Log(Logger.Levels.WARN, category, message, data)
end

function Logger.Error(category, message, data)
    Logger.Log(Logger.Levels.ERROR, category, message, data)
end

-- Query logs
function Logger.Query(filter, limit, callback)
    limit = limit or 100

    Logger.logs:AggregateAsync({
        { ["$match"] = filter },
        { ["$sort"] = { timestamp = -1 } },
        { ["$limit"] = limit }
    }, function(err, results)
        callback(results or {})
    end)
end

-- Get logs by category
function Logger.GetByCategory(category, limit, callback)
    Logger.Query({ category = category }, limit, callback)
end

-- Get logs by player
function Logger.GetByPlayer(steamid, limit, callback)
    Logger.Query({ ["data.steamid"] = steamid }, limit, callback)
end

-- Get logs by time range
function Logger.GetByTimeRange(startTime, endTime, callback)
    Logger.Query({
        timestamp = {
            ["$gte"] = startTime,
            ["$lte"] = endTime
        }
    }, 1000, callback)
end

-- Get error logs
function Logger.GetErrors(limit, callback)
    Logger.Query({ level = Logger.Levels.ERROR }, limit, callback)
end

-- Get log statistics
function Logger.GetStats(hours, callback)
    hours = hours or 24
    local since = os.time() - (hours * 3600)

    Logger.logs:AggregateAsync({
        {
            ["$match"] = { timestamp = { ["$gte"] = since } }
        },
        {
            ["$group"] = {
                _id = "$category",
                count = { ["$sum"] = 1 },
                errors = {
                    ["$sum"] = {
                        ["$cond"] = {
                            ["if"] = { ["$eq"] = { "$level", Logger.Levels.ERROR } },
                            ["then"] = 1,
                            ["else"] = 0
                        }
                    }
                }
            }
        },
        { ["$sort"] = { count = -1 } }
    }, function(err, results)
        callback(results or {})
    end)
end

-- Cleanup old logs
function Logger.Cleanup(daysToKeep, callback)
    local cutoff = os.time() - (daysToKeep * 24 * 3600)

    Logger.logs:DeleteManyAsync(
        { timestamp = { ["$lt"] = cutoff } },
        function(err, deleted)
            if callback then callback(deleted) end
        end
    )
end

return Logger
```