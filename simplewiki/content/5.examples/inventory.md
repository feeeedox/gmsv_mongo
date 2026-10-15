---
title: Inventory System
description: Lua example for inventory system with gmsv_mongo
navigation:
  title: Inventory System
---

# Inventory System

::info-box{type="important"}
These examples were AI-generated and have not been reviewed for accuracy. Use them as a starting point and verify correctness before deploying in production.
::

## Implementation

```lua
--[[
    Player Inventory System
    Manages player items with stacking and limits
]]

local Inventory = {}
Inventory.MAX_SLOTS = 50
Inventory.MAX_STACK = 999

function Inventory.Initialize(db)
    Inventory.db = db
    Inventory.players = db:Collection("players")
end

-- Get player inventory
function Inventory.Get(steamid, callback)
    Inventory.players:FindOneAsync(
        { steamid = steamid },
        function(err, player)
            if err or not player then
                callback({})
            else
                callback(player.inventory or {})
            end
        end
    )
end

-- Add item to inventory
function Inventory.AddItem(steamid, itemId, itemName, quantity, callback)
    quantity = quantity or 1

    Inventory.players:FindOneAsync({ steamid = steamid }, function(err, player)
        if err or not player then
            if callback then callback(false, "Player not found") end
            return
        end

        local inventory = player.inventory or {}
        local added = false

        -- Try to stack with existing item
        for _, item in ipairs(inventory) do
            if item.item_id == itemId then
                local newQty = math.min(item.quantity + quantity, Inventory.MAX_STACK)
                local actualAdded = newQty - item.quantity
                item.quantity = newQty
                added = true
                quantity = quantity - actualAdded
                break
            end
        end

        -- Add as new item if not stacked (and we have remaining)
        if not added and quantity > 0 then
            if #inventory >= Inventory.MAX_SLOTS then
                if callback then callback(false, "Inventory full") end
                return
            end

            table.insert(inventory, {
                item_id = itemId,
                item_name = itemName,
                quantity = math.min(quantity, Inventory.MAX_STACK),
                added_at = os.time()
            })
        end

        -- Save inventory
        Inventory.players:UpdateOneAsync(
            { steamid = steamid },
            { ["$set"] = { inventory = inventory } },
            function(err)
                if callback then
                    callback(not err, err and "Save failed" or nil)
                end
            end
        )
    end)
end

-- Remove item from inventory
function Inventory.RemoveItem(steamid, itemId, quantity, callback)
    quantity = quantity or 1

    Inventory.players:FindOneAsync({ steamid = steamid }, function(err, player)
        if err or not player then
            if callback then callback(false, "Player not found") end
            return
        end

        local inventory = player.inventory or {}
        local removed = false

        for i, item in ipairs(inventory) do
            if item.item_id == itemId then
                if item.quantity <= quantity then
                    table.remove(inventory, i)
                else
                    item.quantity = item.quantity - quantity
                end
                removed = true
                break
            end
        end

        if not removed then
            if callback then callback(false, "Item not found") end
            return
        end

        Inventory.players:UpdateOneAsync(
            { steamid = steamid },
            { ["$set"] = { inventory = inventory } },
            function(err)
                if callback then
                    callback(not err, err and "Save failed" or nil)
                end
            end
        )
    end)
end

-- Check if player has item
function Inventory.HasItem(steamid, itemId, quantity, callback)
    quantity = quantity or 1

    Inventory.Get(steamid, function(inventory)
        for _, item in ipairs(inventory) do
            if item.item_id == itemId and item.quantity >= quantity then
                callback(true, item.quantity)
                return
            end
        end
        callback(false, 0)
    end)
end

-- Transfer item between players
function Inventory.Transfer(fromSteamid, toSteamid, itemId, quantity, callback)
    quantity = quantity or 1

    Inventory.HasItem(fromSteamid, itemId, quantity, function(has, currentQty)
        if not has then
            callback(false, "Insufficient items")
            return
        end

        -- Get item name
        Inventory.Get(fromSteamid, function(inv)
            local itemName = ""
            for _, item in ipairs(inv) do
                if item.item_id == itemId then
                    itemName = item.item_name
                    break
                end
            end

            -- Remove from source
            Inventory.RemoveItem(fromSteamid, itemId, quantity, function(success)
                if not success then
                    callback(false, "Failed to remove item")
                    return
                end

                -- Add to destination
                Inventory.AddItem(toSteamid, itemId, itemName, quantity, function(success, err)
                    if not success then
                        -- Rollback
                        Inventory.AddItem(fromSteamid, itemId, itemName, quantity)
                        callback(false, "Failed to add item: " .. (err or ""))
                    else
                        callback(true)
                    end
                end)
            end)
        end)
    end)
end

-- Get inventory statistics
function Inventory.GetStats(steamid, callback)
    Inventory.Get(steamid, function(inventory)
        local stats = {
            total_items = 0,
            unique_items = #inventory,
            slots_used = #inventory,
            slots_free = Inventory.MAX_SLOTS - #inventory
        }

        for _, item in ipairs(inventory) do
            stats.total_items = stats.total_items + item.quantity
        end

        callback(stats)
    end)
end

return Inventory
```