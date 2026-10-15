---
title: Update Operations
description: Update existing documents in MongoDB collections
navigation:
  title: Update Operations
  order: 4
---

# Update Operations

Update matching documents using MongoDB update operators.

::hatnote
For callback conventions and error handling, see [Async Operations](/crud-operations/async).
::

::link-card{to="/operators/update" title="Update Operators" description="Find field and array update syntax."}
::

## UpdateOne

Update the first document matching a filter.

### Syntax

```lua
local modifiedCount = collection:UpdateOne(filter, update [, upsert])
```

### Parameters

::field-table
---
items:
  "filter": "`table`. Query filter to match documents"
  "update": "`table`. Update operations to apply"
  "upsert": "`boolean` (optional). Create document if none match (default: false)"
---
::

### Returns

::field-table
---
items:
  "number": "Count of modified documents (0 or 1)"
---
::

### Examples

```lua
local players = db:Collection("players")

-- Basic update
local modified = players:UpdateOne(
    { steamid = "STEAM_0:1:12345" },
    { ["$set"] = { level = 10 } }
)
print("Modified:", modified, "document(s)")

-- Update multiple fields
players:UpdateOne(
    { steamid = "STEAM_0:1:12345" },
    {
        ["$set"] = {
            level = 15,
            last_login = os.time(),
            username = "NewName"
        }
    }
)

-- Increment a value
players:UpdateOne(
    { steamid = "STEAM_0:1:12345" },
    { ["$inc"] = { credits = 100 } }
)

-- Upsert (insert if not exists)
players:UpdateOne(
    { steamid = "STEAM_0:1:99999" },
    {
        ["$set"] = {
            username = "NewPlayer",
            level = 1
        }
    },
    true  -- upsert = true
)
```

### UpdateOneAsync

```lua
collection:UpdateOneAsync(filter, update, callback)
collection:UpdateOneAsync(filter, update, upsert, callback)
```

`upsert` defaults to `false`. Pass `true` to insert a document when none match, or `nil` to use the default. The callback receives `(err, modifiedCount)`; inserting through upsert returns `0` modified documents.

```lua
collection:UpdateOneAsync(
    { steamid = "STEAM_0:1:12345" },
    { ["$inc"] = { credits = 100 } },
    function(err, modifiedCount)
        if err then
            print("Update error:", err)
        else
            print("Modified:", modifiedCount)
        end
    end
)
```

::collapsible{title="More callback examples"}

```lua
collection:UpdateOneAsync(
    { steamid = steamid },
    { ["$inc"] = { credits = 100 } },
    function(err, modifiedCount)
        if err then
            print("Update error:", err)
            return
        end
        print("Modified:", modifiedCount)
    end
)
```

```lua
-- Save or create a player with upsert enabled
collection:UpdateOneAsync(
    { steamid = steamid },
    { ["$set"] = { level = 2, credits = 1500 } },
    true,
    function(err, modifiedCount)
        if err then
            print("Update error:", err)
            return
        end
        print("Modified:", modifiedCount)
    end
)
```

::

## UpdateMany

Update all documents matching a filter.

### Syntax

```lua
local modifiedCount = collection:UpdateMany(filter, update [, upsert])
```

### Parameters

::field-table
---
items:
  "filter": "`table`. Query filter to match documents"
  "update": "`table`. Update operations to apply"
  "upsert": "`boolean` (optional). Create document if none match (default: false)"
---
::

### Returns

::field-table
---
items:
  "number": "Count of modified documents"
---
::

### Examples

```lua
local players = db:Collection("players")

-- Update all players of a class
local modified = players:UpdateMany(
    { class = "Warrior" },
    { ["$inc"] = { strength = 5 } }
)
print("Modified:", modified, "warriors")

-- Reset all inactive players
local reset = players:UpdateMany(
    { last_login = { ["$lt"] = os.time() - (30 * 24 * 60 * 60) } },
    {
        ["$set"] = {
            active = false,
            vacation_mode = true
        }
    }
)
print("Marked", reset, "players as inactive")

-- Give bonus to all VIP players
players:UpdateMany(
    { vip = true },
    { ["$inc"] = { credits = 1000 } }
)
```

### UpdateManyAsync

```lua
collection:UpdateManyAsync(filter, update, callback)
collection:UpdateManyAsync(filter, update, upsert, callback)
```

`upsert` defaults to `false`. Pass `true` to insert a document when none match, or `nil` to use the default. The callback receives `(err, modifiedCount)`; inserting through upsert returns `0` modified documents.

```lua
collection:UpdateManyAsync(
    { active = true },
    { ["$inc"] = { daily_bonus = 50 } },
    function(err, modifiedCount)
        if err then
            print("Update error:", err)
        else
            print("Gave bonus to", modifiedCount, "players")
        end
    end
)
```

::collapsible{title="More callback examples"}

```lua
collection:UpdateManyAsync(
    { vip = true },
    { ["$inc"] = { bonus = 50 } },
    function(err, modifiedCount)
        if err then
            print("Update error:", err)
            return
        end
        print("Updated", modifiedCount, "VIP players")
    end
)
```

::

## Update Operators

See the [Update Operators reference](/operators/update) for all existing operator examples.


<span id="set-set-field-values"></span>

[$set - Set Field Values](/operators/update#set-set-field-values)

<span id="inc-increment-values"></span>

[$inc - Increment Values](/operators/update#inc-increment-values)

<span id="unset-remove-fields"></span>

[$unset - Remove Fields](/operators/update#unset-remove-fields)

<span id="push-add-to-array"></span>

[$push - Add to Array](/operators/update#push-add-to-array)

<span id="pull-remove-from-array"></span>

[$pull - Remove from Array](/operators/update#pull-remove-from-array)

<span id="addtoset-add-unique-to-array"></span>

[$addToSet - Add Unique to Array](/operators/update#addtoset-add-unique-to-array)

<span id="combined-operators"></span>

[Combined Operators](/operators/update#combined-operators)

## Upsert Pattern

Create or update in a single operation:

```lua
function SavePlayerStats(steamid, stats)
    local players = db:Collection("players")

    players:UpdateOne(
        { steamid = steamid },
        {
            ["$set"] = {
                stats = stats,
                updated_at = os.time()
            },
            ["$setOnInsert"] = {
                created_at = os.time(),
                steamid = steamid
            }
        },
        true  -- upsert
    )
end
```

## Async Update Operations

Async variants are documented beside each synchronous method: [UpdateOneAsync](#updateoneasync), [UpdateManyAsync](#updatemanyasync).
