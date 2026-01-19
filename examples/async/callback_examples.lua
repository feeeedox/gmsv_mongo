-- MongoDB Async Operations with Callbacks Example
-- This demonstrates how to use callbacks with all async operations

local MongoDB = MongoDB or require("gmsv_mongo")

-- Connect to MongoDB
local client = MongoDB.Connect("mongodb://localhost:27017")
local db = client:GetDatabase("test_db")
local collection = db:GetCollection("users")

print("=== MongoDB Async Callbacks Example ===\n")

-- 1. InsertOneAsync with callback
print("1. InsertOneAsync with callback")
collection:InsertOneAsync(
    { name = "John", age = 30 },
    function(err, result)
        if err then
            print("Error inserting document: " .. err)
        else
            print("Document inserted with ID: " .. result)
        end
    end
)

-- 2. InsertManyAsync with callback
print("\n2. InsertManyAsync with callback")
collection:InsertManyAsync(
    {
        { name = "Alice", age = 25 },
        { name = "Bob", age = 35 }
    },
    function(err, result)
        if err then
            print("Error inserting documents: " .. err)
        else
            print("Documents inserted with IDs:")
            for i, id in ipairs(result) do
                print("  " .. i .. ": " .. id)
            end
        end
    end
)

-- 3. FindAsync with callback
print("\n3. FindAsync with callback")
collection:FindAsync(
    { age = { ["$gte"] = 25 } },
    10, -- limit
    function(err, result)
        if err then
            print("Error finding documents: " .. err)
        else
            print("Found " .. #result .. " documents:")
            for i, doc in ipairs(result) do
                print("  " .. doc.name .. " (age: " .. doc.age .. ")")
            end
        end
    end
)

-- 4. FindOneAsync with callback
print("\n4. FindOneAsync with callback")
collection:FindOneAsync(
    { name = "John" },
    function(err, result)
        if err then
            print("Error finding document: " .. err)
        elseif result then
            print("Found document: " .. result.name .. " (age: " .. result.age .. ")")
        else
            print("No document found")
        end
    end
)

-- 5. UpdateOneAsync with callback
print("\n5. UpdateOneAsync with callback")
collection:UpdateOneAsync(
    { name = "John" },
    { ["$set"] = { age = 31 } },
    false, -- upsert
    function(err, result)
        if err then
            print("Error updating document: " .. err)
        else
            print("Updated " .. result .. " document(s)")
        end
    end
)

-- 6. UpdateManyAsync with callback
print("\n6. UpdateManyAsync with callback")
collection:UpdateManyAsync(
    { age = { ["$lt"] = 30 } },
    { ["$inc"] = { age = 1 } },
    false, -- upsert
    function(err, result)
        if err then
            print("Error updating documents: " .. err)
        else
            print("Updated " .. result .. " document(s)")
        end
    end
)

-- 7. DeleteOneAsync with callback
print("\n7. DeleteOneAsync with callback")
collection:DeleteOneAsync(
    { name = "Bob" },
    function(err, result)
        if err then
            print("Error deleting document: " .. err)
        else
            print("Deleted " .. result .. " document(s)")
        end
    end
)

-- 8. DeleteManyAsync with callback
print("\n8. DeleteManyAsync with callback")
collection:DeleteManyAsync(
    { age = { ["$gt"] = 30 } },
    function(err, result)
        if err then
            print("Error deleting documents: " .. err)
        else
            print("Deleted " .. result .. " document(s)")
        end
    end
)

-- 9. CountAsync with callback
print("\n9. CountAsync with callback")
collection:CountAsync(
    { age = { ["$gte"] = 25 } },
    function(err, result)
        if err then
            print("Error counting documents: " .. err)
        else
            print("Found " .. result .. " matching document(s)")
        end
    end
)

-- 10. AggregateAsync with callback
print("\n10. AggregateAsync with callback")
collection:AggregateAsync(
    {
        { ["$match"] = { age = { ["$gte"] = 25 } } },
        { ["$group"] = {
            _id = nil,
            avgAge = { ["$avg"] = "$age" },
            count = { ["$sum"] = 1 }
        } }
    },
    function(err, result)
        if err then
            print("Error aggregating: " .. err)
        else
            print("Aggregation results:")
            for i, doc in ipairs(result) do
                print("  Average age: " .. (doc.avgAge or "N/A"))
                print("  Count: " .. (doc.count or 0))
            end
        end
    end
)

-- 11. Chaining async operations with callbacks
print("\n11. Chaining async operations")
collection:InsertOneAsync(
    { name = "Charlie", age = 28 },
    function(err, insertedId)
        if err then
            print("Error in insert: " .. err)
            return
        end

        print("Inserted Charlie with ID: " .. insertedId)

        -- After insert, find the document
        collection:FindOneAsync(
            { name = "Charlie" },
            function(err, doc)
                if err then
                    print("Error in find: " .. err)
                    return
                end

                if doc then
                    print("Found Charlie: age " .. doc.age)

                    -- After find, update the document
                    collection:UpdateOneAsync(
                        { name = "Charlie" },
                        { ["$set"] = { age = 29 } },
                        false,
                        function(err, count)
                            if err then
                                print("Error in update: " .. err)
                                return
                            end

                            print("Updated " .. count .. " document(s)")
                            print("Callback chain completed successfully!")
                        end
                    )
                end
            end
        )
    end
)

print("\n=== All async operations submitted ===")
print("Note: Callbacks will be executed when operations complete")
print("Make sure you have the Think hook registered to process callbacks")
