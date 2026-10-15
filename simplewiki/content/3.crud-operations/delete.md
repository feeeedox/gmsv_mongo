---
title: Delete Operations
description: Delete existing documents from MongoDB collections
navigation:
  title: Delete Operations
  order: 5
---

# Delete Operations

Delete individual documents or all documents matching a filter.

::hatnote
For callback conventions and error handling, see [Async Operations](/crud-operations/async).
::

## DeleteOne

Delete the first document matching a filter.

### Syntax

```lua
local deletedCount = collection:DeleteOne(filter)
```

### Parameters

::field-table
---
items:
  "filter": "`table`. Query filter to match documents"
---
::

### Returns

::field-table
---
items:
  "number": "Count of deleted documents (0 or 1)"
---
::

### Examples

```lua
local players = db:Collection("players")

-- Delete by unique identifier
local deleted = players:DeleteOne({
    steamid = "STEAM_0:1:12345678"
})

if deleted > 0 then
    print("Player deleted")
else
    print("Player not found")
end

-- Delete first matching document
players:DeleteOne({ banned = true })

-- Delete by _id
players:DeleteOne({ _id = "ObjectId(...)" })
```

### DeleteOneAsync

```lua
collection:DeleteOneAsync(
    { steamid = "STEAM_0:1:12345" },
    function(err, deletedCount)
        if err then
            print("Delete error:", err)
        elseif deletedCount > 0 then
            print("Player deleted")
        else
            print("Player not found")
        end
    end
)
```

::collapsible{title="More callback examples"}

```lua
collection:DeleteOneAsync(
    { steamid = steamid },
    function(err, deletedCount)
        if err then
            print("Delete error:", err)
            return
        end
        if deletedCount > 0 then
            print("Deleted successfully")
        end
    end
)
```

::

## DeleteMany

Delete all documents matching a filter.

### Syntax

```lua
local deletedCount = collection:DeleteMany(filter)
```

### Parameters

::field-table
---
items:
  "filter": "`table`. Query filter to match documents"
---
::

### Returns

::field-table
---
items:
  "number": "Count of deleted documents"
---
::

### Examples

```lua
local players = db:Collection("players")

-- Delete all banned players
local deleted = players:DeleteMany({ banned = true })
print("Removed", deleted, "banned players")

-- Delete inactive players (30 days)
local thirtyDaysAgo = os.time() - (30 * 24 * 60 * 60)
local cleaned = players:DeleteMany({
    last_login = { ["$lt"] = thirtyDaysAgo }
})
print("Cleaned up", cleaned, "inactive accounts")

-- Delete all documents (BE CAREFUL!)
local all = players:DeleteMany({})
print("Deleted all", all, "players")
```

::info-box{type="alert"}
An empty filter in `DeleteMany({})` deletes every document in the collection.
::

### DeleteManyAsync

```lua
collection:DeleteManyAsync(
    { banned = true },
    function(err, deletedCount)
        if err then
            print("Delete error:", err)
        else
            print("Removed", deletedCount, "banned players")
        end
    end
)
```

::collapsible{title="More callback examples"}

```lua
collection:DeleteManyAsync(
    { expired = true },
    function(err, deletedCount)
        if err then
            print("Delete error:", err)
            return
        end
        print("Deleted", deletedCount, "expired records")
    end
)
```

::

## Async Delete Operations

Async variants are documented beside each synchronous method: [DeleteOneAsync](#deleteoneasync), [DeleteManyAsync](#deletemanyasync).
