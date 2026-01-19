--[[
    Index Management Example - gmsv_mongo V2

    Demonstrates creating and managing indexes for better query performance
]]

require("mongo")

local client = MongoDB.Client("mongodb://admin:password@localhost:27017")
if not client then
    print("Connection failed!")
    return
end

local db = client:Database("gameserver")
local players = db:Collection("players")

print("=== Index Management Example ===\n")

-- Create a simple index
print("1. Creating index on 'steamid' (unique)...")
local indexName = players:CreateIndex(
    { steamid = 1 },  -- 1 = ascending, -1 = descending
    true,             -- unique
    "steamid_unique"  -- name
)
print("   Created index:", indexName)

-- Create a compound index
print("\n2. Creating compound index on 'level' and 'score'...")
local compoundIndex = players:CreateIndex(
    { level = -1, score = -1 },  -- descending order for both
    false,  -- not unique
    "level_score_desc"
)
print("   Created index:", compoundIndex)

-- Create a text index for search
print("\n3. Creating text index on 'username'...")
local textIndex = players:CreateIndex(
    { username = "text" },
    false,
    "username_text"
)
print("   Created index:", textIndex)

-- List all indexes
print("\n4. Listing all indexes:")
local indexes = players:ListIndexes()
for i, index in ipairs(indexes) do
    print(string.format("   %d. %s", i, index.name or "unnamed"))
    if index.key then
        print("      Keys:", table.concat(table.GetKeys(index.key), ", "))
    end
    if index.unique then
        print("      Unique: true")
    end
end

-- Drop a specific index
print("\n5. Dropping 'username_text' index...")
local dropped = players:DropIndex("username_text")
if dropped then
    print("   ✓ Index dropped successfully")
else
    print("   ✗ Failed to drop index")
end

-- Verify deletion
print("\n6. Verifying - Current indexes:")
indexes = players:ListIndexes()
for i, index in ipairs(indexes) do
    print("   " .. i .. ". " .. (index.name or "unnamed"))
end

print("\n=== Index Management Complete ===")
