--[[
    Basic Connection Example - gmsv_mongo V2

    This example demonstrates how to create a basic MongoDB connection
    and verify that it's working.
]]

-- Load the MongoDB module
require("mongo")

-- Print version
print("MongoDB Module Version:", MongoDB.Version())

-- Method 1: Simple connection
local client = MongoDB.Client("mongodb://admin:password@localhost:27017")
if not client then
    print("Failed to connect to MongoDB!")
    return
end

print("✓ Successfully connected to MongoDB!")

-- List all databases
local databases = client:ListDatabases()
if databases then
    print("\nAvailable databases:")
    for i, dbName in ipairs(databases) do
        print("  " .. i .. ". " .. dbName)
    end
else
    print("Failed to list databases")
end

-- Method 2: Connection with options
local clientWithOpts = MongoDB.ClientWithOptions("mongodb://localhost:27017", {
    app_name = "GModServer",
    max_pool_size = 50,
    retry_writes = true
})

if clientWithOpts then
    print("\n✓ Successfully connected with custom options!")
end
