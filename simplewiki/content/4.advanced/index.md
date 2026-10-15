---
title: Manage Indexes
description: Create and manage indexes for optimal query performance
navigation:
  title: Manage Indexes
  order: 1
---

# Index Management

Indexes are essential for database performance. They allow MongoDB to efficiently locate documents without scanning every document in a collection.

## Why Use Indexes?

Without an index:
```
Query: Find player by steamid
Process: Scan ALL documents → Find match → Return
```

With an index:
```
Query: Find player by steamid
Process: Look up in index → Jump to document → Return
```

## Creating Indexes

### Syntax

```lua
local indexName = collection:CreateIndex(keys, unique, name)
```

### Parameters

- `keys` (table): Fields to index with sort direction (1 = ascending, -1 = descending)
- `unique` (boolean): Enforce unique values
- `name` (string): Custom index name

### Returns

- `string`: Name of created index
- `nil`: On failure

## Index Types

### Single Field Index

Index on one field:

```lua
-- Ascending index on steamid (unique)
local name = players:CreateIndex(
    { steamid = 1 },
    true,               -- unique
    "steamid_unique"
)
print("Created index:", name)

-- Descending index on level
players:CreateIndex(
    { level = -1 },
    false,
    "level_desc"
)
```

### Compound Index

Index on multiple fields:

```lua
-- Index on class and level
players:CreateIndex(
    { class = 1, level = -1 },
    false,
    "class_level"
)

-- Useful for queries like:
players:Find({ class = "Warrior", level = { ["$gte"] = 10 } })
```

### Text Index

For text search:

```lua
-- Create text index on username
players:CreateIndex(
    { username = "text" },
    false,
    "username_text"
)

-- Now you can search:
players:Find({
    ["$text"] = { ["$search"] = "player" }
})
```

## Listing Indexes

Get all indexes on a collection:

```lua
local indexes = collection:ListIndexes()

if indexes then
    print("Indexes on collection:")
    for i, index in ipairs(indexes) do
        print(string.format("%d. %s", i, index.name or "unnamed"))

        -- Print keys
        if index.key then
            local keys = {}
            for field, direction in pairs(index.key) do
                table.insert(keys, field .. ":" .. tostring(direction))
            end
            print("   Keys: " .. table.concat(keys, ", "))
        end

        -- Show if unique
        if index.unique then
            print("   Unique: true")
        end
    end
end
```

## Dropping Indexes

### Drop Specific Index

```lua
local success = collection:DropIndex("index_name")

if success then
    print("✓ Index dropped")
else
    print("✗ Failed to drop index")
end
```

### Drop All Indexes

```lua
-- Drop all indexes except _id
collection:DropIndex("*")
```

::info-box{type="important"}
The `_id` index cannot be dropped and is always present.
::

## Index Strategies

### For Player Lookups

```lua
-- Most common query: find by SteamID
players:CreateIndex(
    { steamid = 1 },
    true,  -- Unique!
    "steamid_unique"
)
```

### For Leaderboards

```lua
-- Sort by score descending
players:CreateIndex(
    { score = -1 },
    false,
    "score_desc"
)

-- Or compound for filtered leaderboards
players:CreateIndex(
    { banned = 1, score = -1 },
    false,
    "active_score"
)
```

### For Filtering

```lua
-- Common filter fields
players:CreateIndex({ class = 1 }, false, "class_index")
players:CreateIndex({ level = 1 }, false, "level_index")
players:CreateIndex({ vip = 1 }, false, "vip_index")

-- Or compound for common query patterns
players:CreateIndex(
    { class = 1, level = 1, vip = 1 },
    false,
    "filter_compound"
)
```

### For Time-Based Queries

```lua
-- Index on timestamps
players:CreateIndex(
    { last_login = -1 },
    false,
    "recent_login"
)

-- TTL index for auto-expiring documents
-- (Note: TTL indexes are managed by MongoDB server)
```

::see-also
---
items:
  - title: "Query Operators"
    to: "/operators/query"
  - title: "Query Operations"
    to: "/crud-operations/query"
---
::
