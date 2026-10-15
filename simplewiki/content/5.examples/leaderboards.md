---
title: Leaderboard System
description: Lua example for leaderboard system with gmsv_mongo
navigation:
  title: Leaderboard System
---

# Leaderboard System

::info-box{type="important"}
These examples were AI-generated and have not been reviewed for accuracy. Use them as a starting point and verify correctness before deploying in production.
::

## Implementation

```lua
--[[
    Leaderboard System
    Score tracking and rankings
]]

local Leaderboard = {}

function Leaderboard.Initialize(db)
    Leaderboard.db = db
    Leaderboard.scores = db:Collection("leaderboard")

    -- Create indexes
    Leaderboard.scores:CreateIndex({ category = 1, score = -1 }, false, "category_score")
    Leaderboard.scores:CreateIndex({ steamid = 1, category = 1 }, true, "player_category")
end

-- Submit a score
function Leaderboard.SubmitScore(steamid, username, score, category, callback)
    category = category or "default"

    -- Upsert - update if higher, insert if new
    Leaderboard.scores:FindOneAsync(
        { steamid = steamid, category = category },
        function(err, existing)
            if existing and existing.score >= score then
                -- Existing score is higher
                if callback then callback(false, "Existing score is higher") end
                return
            end

            local doc = {
                steamid = steamid,
                username = username,
                category = category,
                score = score,
                submitted_at = os.time()
            }

            if existing then
                -- Update existing
                Leaderboard.scores:UpdateOneAsync(
                    { steamid = steamid, category = category },
                    { ["$set"] = doc },
                    function(err)
                        if callback then callback(not err, err) end
                    end
                )
            else
                -- Insert new
                Leaderboard.scores:InsertOneAsync(doc, function(err)
                    if callback then callback(not err, err) end
                end)
            end
        end
    )
end

-- Get top scores
function Leaderboard.GetTop(category, limit, callback)
    category = category or "default"
    limit = limit or 10

    Leaderboard.scores:AggregateAsync({
        { ["$match"] = { category = category } },
        { ["$sort"] = { score = -1 } },
        { ["$limit"] = limit },
        {
            ["$project"] = {
                steamid = 1,
                username = 1,
                score = 1,
                submitted_at = 1,
                _id = 0
            }
        }
    }, function(err, results)
        callback(results or {})
    end)
end

-- Get player rank
function Leaderboard.GetRank(steamid, category, callback)
    category = category or "default"

    Leaderboard.scores:FindOneAsync(
        { steamid = steamid, category = category },
        function(err, player)
            if not player then
                callback(nil, 0)
                return
            end

            -- Count players with higher scores
            Leaderboard.scores:CountAsync(
                { category = category, score = { ["$gt"] = player.score } },
                function(err, higher)
                    callback(higher + 1, player.score)
                end
            )
        end
    )
end

-- Get player's scores in all categories
function Leaderboard.GetPlayerScores(steamid, callback)
    Leaderboard.scores:FindAsync(
        { steamid = steamid },
        100,
        function(err, results)
            callback(results or {})
        end
    )
end

-- Get leaderboard with ranks around player
function Leaderboard.GetAroundPlayer(steamid, category, range, callback)
    category = category or "default"
    range = range or 5

    Leaderboard.GetRank(steamid, category, function(rank, score)
        if not rank then
            callback({}, nil)
            return
        end

        local skip = math.max(0, rank - range - 1)

        Leaderboard.scores:AggregateAsync({
            { ["$match"] = { category = category } },
            { ["$sort"] = { score = -1 } },
            { ["$skip"] = skip },
            { ["$limit"] = range * 2 + 1 },
            {
                ["$project"] = {
                    steamid = 1,
                    username = 1,
                    score = 1,
                    _id = 0
                }
            }
        }, function(err, results)
            -- Add rank numbers
            for i, entry in ipairs(results) do
                entry.rank = skip + i
            end
            callback(results, rank)
        end)
    end)
end

return Leaderboard
```