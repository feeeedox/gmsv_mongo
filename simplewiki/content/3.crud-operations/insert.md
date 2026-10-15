---
title: Insert Operations
description: Create new documents in MongoDB collections
navigation:
  title: Insert Operations
  order: 2
---

# Insert Operations

Insert single documents or batches into a collection.

::hatnote
For callback conventions and error handling, see [Async Operations](/crud-operations/async).
::

## InsertOne

Insert a single document into a collection.

### Syntax

```lua
local insertedId = collection:InsertOne(document)
```

### Parameters

::field-table
---
items:
  "document": "`table`. The document to insert"
---
::

### Returns

::field-table
---
items:
  "string": "The inserted document's ObjectId"
  "nil": "On failure"
---
::

### Example

```lua
local players = db:Collection("players")

local id = players:InsertOne({
    steamid = "STEAM_0:1:12345678",
    username = "NewPlayer",
    level = 1,
    credits = 1000,
    joined_at = os.time(),
    inventory = {},
    stats = {
        kills = 0,
        deaths = 0,
        playtime = 0
    }
})

if id then
    print("Inserted player with ID:", id)
else
    print("Insert failed!")
end
```

### Document Structure

Documents are Lua tables that convert to BSON:

```lua
-- Simple document
{
    name = "Test",
    value = 42
}

-- Nested document
{
    player = {
        name = "Test",
        level = 10
    },
    items = { "sword", "shield", "potion" }
}

-- With various types
{
    string_field = "hello",
    number_field = 123,
    float_field = 3.14,
    boolean_field = true,
    array_field = { 1, 2, 3 },
    nested_field = { key = "value" },
    timestamp = os.time()
}
```

### InsertOneAsync

```lua
collection:InsertOneAsync(document, function(err, result)
    if err then
        print("Error:", err)
    else
        print("Inserted ID:", result)
    end
end)
```

::collapsible{title="More callback examples"}

```lua
collection:InsertOneAsync(document, function(err, insertedId)
    if err then
        print("Insert error:", err)
        return
    end
    print("Inserted with ID:", insertedId)
end)
```

::

## InsertMany

Insert multiple documents in a single operation.

### Syntax

```lua
local insertedIds = collection:InsertMany(documents)
```

### Parameters

::field-table
---
items:
  "documents": "`table`. Array of documents to insert"
---
::

### Returns

::field-table
---
items:
  "table": "Array of inserted ObjectIds"
  "nil": "On failure"
---
::

### Example

```lua
local players = db:Collection("players")

local ids = players:InsertMany({
    {
        steamid = "STEAM_0:1:11111111",
        username = "Player1",
        level = 5,
        credits = 5000
    },
    {
        steamid = "STEAM_0:1:22222222",
        username = "Player2",
        level = 10,
        credits = 10000
    },
    {
        steamid = "STEAM_0:1:33333333",
        username = "Player3",
        level = 15,
        credits = 15000
    }
})

if ids then
    print("Inserted", #ids, "players:")
    for i, id in ipairs(ids) do
        print("  " .. i .. ": " .. id)
    end
else
    print("Batch insert failed!")
end
```

### Performance Benefits

`InsertMany` is more efficient than multiple `InsertOne` calls:

```lua
-- Inefficient (multiple round trips)
for i = 1, 100 do
    collection:InsertOne({ index = i })
end

-- Efficient (single round trip)
local docs = {}
for i = 1, 100 do
    table.insert(docs, { index = i })
end
collection:InsertMany(docs)
```

### InsertManyAsync

```lua
collection:InsertManyAsync(documents, function(err, results)
    if err then
        print("Error:", err)
    else
        print("Inserted", #results, "documents")
        for i, id in ipairs(results) do
            print("  " .. id)
        end
    end
end)
```

::collapsible{title="More callback examples"}

```lua
collection:InsertManyAsync(documents, function(err, insertedIds)
    if err then
        print("Insert error:", err)
        return
    end
    print("Inserted", #insertedIds, "documents")
end)
```

::

## Async Insert Operations

Async variants are documented beside each synchronous method: [InsertOneAsync](#insertoneasync), [InsertManyAsync](#insertmanyasync).
