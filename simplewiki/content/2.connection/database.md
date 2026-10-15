---
title: Database Operations
description: Working with MongoDB databases
navigation:
  title: Database Operations
  order: 4
---

# Database Operations

Learn how to work with MongoDB databases using gmsv_mongo.

## Getting a Database

```lua
local client = MongoDB.Client("mongodb://localhost:27017")
local db = client:Database("gameserver")
```

::info-box{type="info"}
MongoDB creates databases automatically when you first write data to them.
::

## Listing Databases

List all databases on the MongoDB server:

```lua
local databases = client:ListDatabases()

if databases then
    print("Available databases:")
    for i, dbName in ipairs(databases) do
        print(string.format("  %d. %s", i, dbName))
    end
else
    print("Failed to list databases")
end
```

**Output:**
```
Available databases:
  1. admin
  2. config
  3. local
  4. gameserver
```

## Collection Operations

### Getting a Collection

```lua
local players = db:Collection("players")
```

### Listing Collections

List all collections in a database:

```lua
local collections = db:ListCollections()

if collections then
    print("Collections in database:")
    for i, colName in ipairs(collections) do
        print(string.format("  %d. %s", i, colName))
    end
end
```

### Creating a Collection

Explicitly create a collection:

```lua
local success = db:CreateCollection("new_collection")

if success then
    print("✓ Collection created")
else
    print("✗ Failed to create collection")
end
```

::info-box{type="info"}
Collections are also created automatically when you first insert data.
::

### Dropping a Collection

Remove a collection and all its data:

```lua
local success = db:DropCollection("old_collection")

if success then
    print("✓ Collection dropped")
else
    print("✗ Failed to drop collection")
end
```

::info-box{type="important"}
Dropping a collection permanently deletes all documents in it. This cannot be undone!
::

## Database Statistics

Get statistics for a collection:

```lua
local stats = db:Stats("players")

if stats then
    print("Collection Statistics:")
    print("  Count:", stats.count)
    print("  Size:", stats.size, "bytes")
    print("  Storage Size:", stats.storageSize, "bytes")
    print("  Avg Object Size:", stats.avgObjSize, "bytes")
end
```

## Dropping a Database

Remove an entire database:

```lua
local success = db:Drop()

if success then
    print("✓ Database dropped")
end
```

::info-box{type="alert"}
**WARNING**: This permanently deletes the entire database and all its collections. Use with extreme caution!
::


## Database Method Reference

| Method                      | Description               | Returns           |
|-----------------------------|---------------------------|-------------------|
| `client:Database(name)`     | Get database reference    | MongoDBDatabase   |
| `client:ListDatabases()`    | List all databases        | table or nil      |
| `db:Collection(name)`       | Get collection reference  | MongoDBCollection |
| `db:ListCollections()`      | List all collections      | table or nil      |
| `db:CreateCollection(name)` | Create new collection     | boolean           |
| `db:DropCollection(name)`   | Delete a collection       | boolean           |
| `db:Stats(collection)`      | Get collection statistics | table or nil      |
| `db:Drop()`                 | Delete entire database    | boolean           |

::see-also
---
items:
  - title: "CRUD Operations"
    to: "/crud-operations"
  - title: "Index Management"
    to: "/advanced"
  - title: "Aggregation Pipelines"
    to: "/advanced/aggregation"
---
::
