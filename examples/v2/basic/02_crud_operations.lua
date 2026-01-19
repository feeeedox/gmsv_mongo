--[[
    CRUD Operations Example - gmsv_mongo V2

    Demonstrates Create, Read, Update, Delete operations
]]

require("mongo")

-- Connect to MongoDB
local client = MongoDB.Client("mongodb://admin:password@localhost:27017")
if not client then
    print("Connection failed!")
    return
end

-- Get database and collection
local db = client:Database("gameserver")
local players = db:Collection("players")

print("=== CRUD Operations Example ===\n")

-- CREATE: Insert a single document
print("1. INSERT ONE")
local playerId = players:InsertOne({
    steamid = "STEAM_0:1:12345678",
    username = "TestPlayer",
    level = 1,
    credits = 1000,
    joined_at = os.time()
})
print("   Inserted player with ID:", playerId)

-- CREATE: Insert multiple documents
print("\n2. INSERT MANY")
local ids = players:InsertMany({
    {
        steamid = "STEAM_0:0:87654321",
        username = "PlayerTwo",
        level = 5,
        credits = 5000
    },
    {
        steamid = "STEAM_0:1:11111111",
        username = "PlayerThree",
        level = 10,
        credits = 10000
    }
})
print("   Inserted", #ids, "players")

-- READ: Find all players
print("\n3. FIND ALL")
local allPlayers = players:Find({})
print("   Total players:", #allPlayers)
for i, player in ipairs(allPlayers) do
    print("   -", player.username, "Level:", player.level)
end

-- READ: Find one player
print("\n4. FIND ONE")
local player = players:FindOne({ steamid = "STEAM_0:1:12345678" })
if player then
    print("   Found:", player.username)
end

-- READ: Find with filter
print("\n5. FIND WITH FILTER")
local highLevelPlayers = players:Find({ level = { ["$gte"] = 5 } })
print("   Players level 5+:", #highLevelPlayers)

-- READ: Count documents
print("\n6. COUNT")
local count = players:Count({ level = { ["$gte"] = 5 } })
print("   Count of level 5+ players:", count)

-- UPDATE: Update one document
print("\n7. UPDATE ONE")
local updated = players:UpdateOne(
    { steamid = "STEAM_0:1:12345678" },
    { ["$set"] = { level = 2, credits = 1500 } }
)
print("   Updated", updated, "document(s)")

-- UPDATE: Update many documents
print("\n8. UPDATE MANY")
local updatedMany = players:UpdateMany(
    { level = { ["$lt"] = 5 } },
    { ["$inc"] = { credits = 100 } }
)
print("   Updated", updatedMany, "document(s)")

-- DELETE: Delete one document
print("\n9. DELETE ONE")
local deleted = players:DeleteOne({ steamid = "STEAM_0:1:11111111" })
print("   Deleted", deleted, "document(s)")

-- DELETE: Delete many documents
print("\n10. DELETE MANY")
local deletedMany = players:DeleteMany({ level = { ["$lt"] = 2 } })
print("   Deleted", deletedMany, "document(s)")

print("\n=== CRUD Operations Complete ===")

Crashes: InsertMany, Find with Filter, Count, UpdateOne, UpdateMany, DeleteMany