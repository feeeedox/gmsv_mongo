---
title: Ban System
description: Lua example for ban system with gmsv_mongo
navigation:
  title: Ban System
---

# Ban System

::info-box{type="important"}
These examples were AI-generated and have not been reviewed for accuracy. Use them as a starting point and verify correctness before deploying in production.
::

## Implementation

```lua
--[[
    Ban System
    Manage player bans with expiration and history
]]

local BanSystem = {}

function BanSystem.Initialize(db)
    BanSystem.db = db
    BanSystem.bans = db:Collection("bans")
    BanSystem.banHistory = db:Collection("ban_history")

    -- Indexes
    BanSystem.bans:CreateIndex({ steamid = 1 }, true, "steamid_unique")
    BanSystem.bans:CreateIndex({ expires_at = 1 }, false, "expiry_index")
    BanSystem.banHistory:CreateIndex({ steamid = 1, banned_at = -1 }, false, "player_history")
end

-- Ban a player
function BanSystem.Ban(steamid, reason, duration, adminSteamid, callback)
    local ban = {
        steamid = steamid,
        reason = reason,
        banned_at = os.time(),
        expires_at = duration and (os.time() + duration) or nil,
        permanent = duration == nil,
        admin_steamid = adminSteamid
    }

    -- Add to active bans
    BanSystem.bans:UpdateOneAsync(
        { steamid = steamid },
        { ["$set"] = ban },
        function(err)
            if err then
                if callback then callback(false, err) end
                return
            end

            -- Add to history
            BanSystem.banHistory:InsertOneAsync({
                steamid = steamid,
                reason = reason,
                duration = duration,
                permanent = duration == nil,
                banned_at = os.time(),
                admin_steamid = adminSteamid
            }, function()
                if callback then callback(true) end
            end)
        end
    )
end

-- Unban a player
function BanSystem.Unban(steamid, adminSteamid, callback)
    BanSystem.bans:DeleteOneAsync(
        { steamid = steamid },
        function(err, deleted)
            if deleted > 0 then
                -- Log the unban
                BanSystem.banHistory:InsertOneAsync({
                    steamid = steamid,
                    action = "unban",
                    unbanned_at = os.time(),
                    admin_steamid = adminSteamid
                })
            end
            if callback then callback(deleted > 0) end
        end
    )
end

-- Check if player is banned
function BanSystem.IsBanned(steamid, callback)
    BanSystem.bans:FindOneAsync({ steamid = steamid }, function(err, ban)
        if not ban then
            callback(false, nil)
            return
        end

        -- Check if expired
        if ban.expires_at and ban.expires_at < os.time() then
            -- Remove expired ban
            BanSystem.bans:DeleteOneAsync({ steamid = steamid })
            callback(false, nil)
            return
        end

        callback(true, ban)
    end)
end

-- Get player's ban history
function BanSystem.GetHistory(steamid, callback)
    BanSystem.banHistory:FindAsync(
        { steamid = steamid },
        100,
        function(err, history)
            callback(history or {})
        end
    )
end

-- Get all active bans
function BanSystem.GetAllBans(callback)
    BanSystem.bans:FindAsync({}, nil, function(err, bans)
        callback(bans or {})
    end)
end

-- Cleanup expired bans
function BanSystem.CleanupExpired(callback)
    BanSystem.bans:DeleteManyAsync(
        {
            permanent = false,
            expires_at = { ["$lt"] = os.time() }
        },
        function(err, deleted)
            if callback then callback(deleted) end
        end
    )
end

-- Get ban statistics
function BanSystem.GetStats(callback)
    BanSystem.bans:AggregateAsync({
        {
            ["$group"] = {
                _id = nil,
                total = { ["$sum"] = 1 },
                permanent = {
                    ["$sum"] = {
                        ["$cond"] = { ["if"] = "$permanent", ["then"] = 1, ["else"] = 0 }
                    }
                },
                temporary = {
                    ["$sum"] = {
                        ["$cond"] = { ["if"] = "$permanent", ["then"] = 0, ["else"] = 1 }
                    }
                }
            }
        }
    }, function(err, results)
        if results and #results > 0 then
            callback(results[1])
        else
            callback({ total = 0, permanent = 0, temporary = 0 })
        end
    end)
end

return BanSystem
```
