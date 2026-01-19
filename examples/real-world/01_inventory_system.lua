--[[
print("\n=== Demo Complete ===")

end
    end
        print(string.format("   %d. %s x%d", i, item.item_name, item.quantity))
    for i, item in ipairs(inv.items) do
if inv then
inv = InventorySystem.GetInventory(testSteamId)
print("\n7. Final inventory:")
-- Final inventory

InventorySystem.RemoveItem(testSteamId, "ammo_9mm", 30)
InventorySystem.RemoveItem(testSteamId, "medkit", 1)
print("\n6. Removing items")
-- Remove items

end
    print(string.format("   Usage: %.1f%%", stats.usage_percent))
    print(string.format("   Unique items: %d/%d", stats.unique_items, stats.max_slots))
    print(string.format("   Total items: %d", stats.total_items))
if stats then
local stats = InventorySystem.GetStats(testSteamId)
print("\n5. Inventory statistics:")
-- Get statistics

end
    print(string.format("   - %s x%d", item.item_name, item.quantity))
for _, item in ipairs(searchResults) do
local searchResults = InventorySystem.SearchItems(testSteamId, "ammo")
print("\n4. Search for 'ammo':")
-- Search items

end
    end
        print(string.format("   %d. %s x%d", i, item.item_name, item.quantity))
    for i, item in ipairs(inv.items) do
if inv then
local inv = InventorySystem.GetInventory(testSteamId)
print("\n3. Current inventory:")
-- Show inventory

InventorySystem.AddItem(testSteamId, "ammo_9mm", "9mm Ammo", 25)  -- Stack
InventorySystem.AddItem(testSteamId, "medkit", "Medkit", 3)
InventorySystem.AddItem(testSteamId, "ammo_9mm", "9mm Ammo", 50)
InventorySystem.AddItem(testSteamId, "weapon_pistol", "Pistol", 1)
print("\n2. Adding items")
-- Add items

InventorySystem.Initialize(testSteamId)
print("1. Initialize inventory")
-- Initialize inventory

local testSteamId = "STEAM_0:1:12345678"

print("=== Player Inventory System Demo ===\n")
-- Demo: Using the inventory system

end
    }
        usage_percent = (uniqueItems / inventory.max_slots) * 100
        max_slots = inventory.max_slots,
        unique_items = uniqueItems,
        total_items = totalItems,
    return {

    end
        totalItems = totalItems + item.quantity
    for _, item in ipairs(inventory.items) do

    local uniqueItems = #inventory.items
    local totalItems = 0

    if not inventory then return nil end
    local inventory = InventorySystem.GetInventory(steamid)
function InventorySystem.GetStats(steamid)
--- Get inventory statistics

end
    return results

    end
        end
            table.insert(results, item)
        if string.find(string.lower(item.item_name), string.lower(query)) then
    for _, item in ipairs(inventory.items) do
    local results = {}

    if not inventory then return {} end
    local inventory = InventorySystem.GetInventory(steamid)
function InventorySystem.SearchItems(steamid, query)
--- Search items in inventory

end
    return inventory

    end
        return nil
        print("No inventory found for " .. steamid)
    if not inventory then
    local inventory = inventories:FindOne({ steamid = steamid })
function InventorySystem.GetInventory(steamid)
--- Get player inventory

end
    return true

    end
        print(string.format("Decreased item %s quantity by %d", itemId, quantity))
        )
            }
                ["$set"] = { updated_at = os.time() }
                ["$inc"] = { ["items.$.quantity"] = -quantity },
            {
            { steamid = steamid, ["items.item_id"] = itemId },
        inventories:UpdateOne(
        -- Decrease quantity
    else
        print(string.format("Removed item %s from inventory", itemId))
        )
            }
                ["$set"] = { updated_at = os.time() }
                ["$pull"] = { items = { item_id = itemId } },
            {
            { steamid = steamid },
        inventories:UpdateOne(
        -- Remove item completely
    if currentQty <= quantity then

    end
        end
            break
            currentQty = item.quantity
        if item.item_id == itemId then
    for _, item in ipairs(inventory.items) do
    local currentQty = 0
    -- Find current quantity

    end
        return false
        print("Item not found in inventory")
    if not inventory then

    })
        ["items.item_id"] = itemId
        steamid = steamid,
    local inventory = inventories:FindOne({

    quantity = quantity or 1
function InventorySystem.RemoveItem(steamid, itemId, quantity)
--- Remove an item from inventory

end
    end
        print(string.format("Added new item %s (%d) to %s's inventory", itemName, quantity, steamid))
        )
            }
                ["$set"] = { updated_at = os.time() }
                },
                    }
                        added_at = os.time()
                        quantity = quantity,
                        item_name = itemName,
                        item_id = itemId,
                    items = {
                ["$push"] = {
            {
            { steamid = steamid },
        local updated = inventories:UpdateOne(
        -- Add new item
    else
        print(string.format("Added %d x %s to %s's inventory", quantity, itemName, steamid))
        )
            { ["$inc"] = { ["items.$.quantity"] = quantity } }
            { steamid = steamid, ["items.item_id"] = itemId },
        local updated = inventories:UpdateOne(
        -- Update existing item quantity
    if inventory then

    })
        ["items.item_id"] = itemId
        steamid = steamid,
    local inventory = inventories:FindOne({
    -- Check if item already exists

    quantity = quantity or 1
function InventorySystem.AddItem(steamid, itemId, itemName, quantity)
--- Add an item to player inventory

end
    return inventoryId
    print("Created new inventory for " .. steamid)

    })
        updated_at = os.time()
        created_at = os.time(),
        max_slots = 20,
        items = {},
        steamid = steamid,
    local inventoryId = inventories:InsertOne({

    end
        return existing
        print("Inventory already exists for " .. steamid)
    if existing then
    local existing = inventories:FindOne({ steamid = steamid })
function InventorySystem.Initialize(steamid)
--- Initialize a new player inventory

local InventorySystem = {}
-- Inventory System API

inventories:CreateIndex({ ["items.item_id"] = 1 }, false, "items_search")
inventories:CreateIndex({ steamid = 1 }, true, "steamid_unique")
-- Create indexes for better performance

local inventories = db:Collection("inventories")
local db = client:Database("gameserver")

end
    error("Failed to connect to MongoDB")
if not client then
local client = MongoDB.Client("mongodb://admin:password@localhost:27017")
-- Initialize MongoDB connection

require("mongo")

    A complete player inventory system with MongoDB backend

    Player Inventory System - Real World Example
