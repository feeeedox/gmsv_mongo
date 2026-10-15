---
title: Matchmaking System
description: Lua example for matchmaking system with gmsv_mongo
navigation:
  title: Matchmaking System
---

# Matchmaking System

::info-box{type="important"}
These examples were AI-generated and have not been reviewed for accuracy. Use them as a starting point and verify correctness before deploying in production.
::

## Implementation

```lua
--[[
    Simple Matchmaking System
    Queue players and create balanced matches
]]

local Matchmaking = {}

function Matchmaking.Initialize(db)
    Matchmaking.db = db
    Matchmaking.queue = db:Collection("matchmaking_queue")
    Matchmaking.matches = db:Collection("matches")

    -- Index for efficient queries
    Matchmaking.queue:CreateIndex({ skill_rating = 1 }, false, "skill_index")
    Matchmaking.queue:CreateIndex({ queued_at = 1 }, false, "queue_time")
end

-- Add player to queue
function Matchmaking.JoinQueue(steamid, username, skillRating, callback)
    -- Remove from queue if already in it
    Matchmaking.queue:DeleteOneAsync({ steamid = steamid }, function()
        -- Add to queue
        Matchmaking.queue:InsertOneAsync({
            steamid = steamid,
            username = username,
            skill_rating = skillRating,
            queued_at = os.time()
        }, function(err)
            if callback then callback(not err) end
        end)
    end)
end

-- Remove player from queue
function Matchmaking.LeaveQueue(steamid, callback)
    Matchmaking.queue:DeleteOneAsync(
        { steamid = steamid },
        function(err, deleted)
            if callback then callback(deleted > 0) end
        end
    )
end

-- Find match for player
function Matchmaking.FindMatch(steamid, skillRating, skillRange, callback)
    skillRange = skillRange or 100

    Matchmaking.queue:FindAsync({
        steamid = { ["$ne"] = steamid },
        skill_rating = {
            ["$gte"] = skillRating - skillRange,
            ["$lte"] = skillRating + skillRange
        }
    }, 100, function(err, candidates)
        if #candidates == 0 then
            callback(nil, "No players in range")
            return
        end

        -- Sort by closest skill rating
        table.sort(candidates, function(a, b)
            return math.abs(a.skill_rating - skillRating) <
                   math.abs(b.skill_rating - skillRating)
        end)

        callback(candidates[1])
    end)
end

-- Create a match
function Matchmaking.CreateMatch(player1, player2, callback)
    local match = {
        players = {
            {
                steamid = player1.steamid,
                username = player1.username,
                skill_rating = player1.skill_rating
            },
            {
                steamid = player2.steamid,
                username = player2.username,
                skill_rating = player2.skill_rating
            }
        },
        status = "pending",
        created_at = os.time(),
        skill_diff = math.abs(player1.skill_rating - player2.skill_rating)
    }

    -- Remove players from queue
    Matchmaking.queue:DeleteManyAsync({
        steamid = { ["$in"] = { player1.steamid, player2.steamid } }
    }, function()
        -- Create match record
        Matchmaking.matches:InsertOneAsync(match, function(err, matchId)
            if err then
                callback(nil, err)
            else
                match._id = matchId
                callback(match)
            end
        end)
    end)
end

-- Get queue statistics
function Matchmaking.GetQueueStats(callback)
    Matchmaking.queue:AggregateAsync({
        {
            ["$group"] = {
                _id = nil,
                count = { ["$sum"] = 1 },
                avg_skill = { ["$avg"] = "$skill_rating" },
                min_skill = { ["$min"] = "$skill_rating" },
                max_skill = { ["$max"] = "$skill_rating" },
                avg_wait = { ["$avg"] = {
                    ["$subtract"] = { os.time(), "$queued_at" }
                }}
            }
        }
    }, function(err, results)
        if results and #results > 0 then
            callback(results[1])
        else
            callback({
                count = 0,
                avg_skill = 0,
                min_skill = 0,
                max_skill = 0,
                avg_wait = 0
            })
        end
    end)
end

-- Process queue and create matches
function Matchmaking.ProcessQueue(callback)
    local created = 0

    Matchmaking.queue:FindAsync({}, 1000, function(err, players)
        if #players < 2 then
            callback(0)
            return
        end

        -- Sort by queue time
        table.sort(players, function(a, b)
            return a.queued_at < b.queued_at
        end)

        local function processNext(index)
            if index > #players then
                callback(created)
                return
            end

            local player = players[index]

            -- Check if still in queue
            Matchmaking.queue:FindOneAsync({ steamid = player.steamid }, function(err, stillQueued)
                if not stillQueued then
                    processNext(index + 1)
                    return
                end

                -- Find match
                Matchmaking.FindMatch(player.steamid, player.skill_rating, 200, function(opponent)
                    if opponent then
                        Matchmaking.CreateMatch(player, opponent, function(match)
                            if match then
                                created = created + 1
                                print("Created match:", player.username, "vs", opponent.username)
                            end
                            processNext(index + 1)
                        end)
                    else
                        processNext(index + 1)
                    end
                end)
            end)
        end

        processNext(1)
    end)
end

return Matchmaking
```
