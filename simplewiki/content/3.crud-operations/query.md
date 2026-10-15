---
title: Query Operations
description: Find and retrieve documents from MongoDB collections
navigation:
  title: Query Operations
  order: 3
---

# Query Operations

Query documents using filters and optional limits.

::hatnote
For callback conventions and error handling, see [Async Operations](/crud-operations/async).
::

::link-card{to="/operators/query" title="Query Operators" description="Find filter syntax and operator examples."}
::

## Find

Retrieve multiple documents matching a filter.

### Syntax

```lua
local documents = collection:Find(filter [, limit])
```

### Parameters

::field-table
---
items:
  "filter": "`table`. Query filter (empty `{}` for all documents)"
  "limit": "`number` (optional). Maximum number of documents to return"
---
::

### Returns

::field-table
---
items:
  "table": "Array of matching documents"
  "nil": "On failure"
---
::

### Examples

```lua
local players = db:Collection("players")

-- Find all documents
local allPlayers = players:Find({})
print("Total players:", #allPlayers)

-- Find with filter
local highLevel = players:Find({
    level = { ["$gte"] = 10 }
})

-- Find with limit
local topPlayers = players:Find(
    { level = { ["$gte"] = 5 } },
    10  -- Return max 10 documents
)

-- Iterate results
for i, player in ipairs(allPlayers) do
    print(player.username, "- Level", player.level)
end
```

### FindAsync

```lua
collection:FindAsync(filter, callback)
collection:FindAsync(filter, limit, callback)
```

The `limit` argument is optional. Omit it or pass `nil` to return all matching documents; pass a positive number to limit the results.

```lua
collection:FindAsync({ level = { ["$gte"] = 10 } }, 100, function(err, results)
    if err then
        print("Query error:", err)
        return
    end

    print("Found", #results, "players")
    for _, player in ipairs(results) do
        print("  " .. player.username)
    end
end)
```

::collapsible{title="More callback examples"}

```lua
-- Find up to 100 documents matching a filter
collection:FindAsync({ level = { ["$gte"] = 10 } }, 100, function(err, documents)
    if err then
        print("Find error:", err)
        return
    end

    print("Found", #documents, "documents")
    for _, doc in ipairs(documents) do
        print("  -", doc.username)
    end
end)

-- Find all players without a limit
collection:FindAsync({}, function(err, documents)
    if err then
        print("Find error:", err)
        return
    end

    print("Total players retrieved:", #documents)
end)
```

::

## FindOne

Retrieve the first document matching a filter.

### Syntax

```lua
local document = collection:FindOne(filter)
```

### Parameters

::field-table
---
items:
  "filter": "`table`. Query filter"
---
::

### Returns

::field-table
---
items:
  "table": "First matching document"
  "nil": "If not found or on failure"
---
::

### Examples

```lua
local players = db:Collection("players")

-- Find by unique field
local player = players:FindOne({
    steamid = "STEAM_0:1:12345678"
})

if player then
    print("Found:", player.username)
    print("Level:", player.level)
    print("Credits:", player.credits)
else
    print("Player not found")
end

-- Find by multiple criteria
local vipPlayer = players:FindOne({
    level = { ["$gte"] = 50 },
    vip = true
})
```

### FindOneAsync

```lua
collection:FindOneAsync({ steamid = "STEAM_0:1:12345" }, function(err, player)
    if err then
        print("Query error:", err)
        return
    end

    if player then
        print("Found:", player.username)
    else
        print("Player not found")
    end
end)
```

::collapsible{title="More callback examples"}

```lua
collection:FindOneAsync({ steamid = steamid }, function(err, document)
    if err then
        print("Find error:", err)
        return
    end

    if document then
        print("Found:", document.username)
    else
        print("Not found")
    end
end)
```

::

## Count

Count documents matching a filter.

### Syntax

```lua
local count = collection:Count(filter)
```

### Parameters

::field-table
---
items:
  "filter": "`table`. Query filter (empty `{}` for all documents)"
---
::

### Returns

::field-table
---
items:
  "number": "Count of matching documents"
---
::

### Examples

```lua
local players = db:Collection("players")

-- Count all documents
local totalPlayers = players:Count({})
print("Total players:", totalPlayers)

-- Count with filter
local activePlayers = players:Count({
    last_login = { ["$gte"] = os.time() - (7 * 24 * 60 * 60) }
})
print("Active players (7 days):", activePlayers)

-- Count by criteria
local vipCount = players:Count({ vip = true })
local warriorCount = players:Count({ class = "Warrior" })
```

### CountAsync

```lua
collection:CountAsync({ active = true }, function(err, count)
    if err then
        print("Count error:", err)
        return
    end

    print("Active players:", count)
end)
```

::collapsible{title="More callback examples"}

```lua
collection:CountAsync({ active = true }, function(err, count)
    if err then
        print("Count error:", err)
        return
    end
    print("Active players:", count)
end)
```

::

## Query Filters

See [Filter Basics](/operators/query#filter-basics) for empty filters, equality, comparisons, arrays, logic, and field checks.


<span id="empty-filter-all-documents"></span>

[Empty Filter (All Documents)](/operators/query#empty-filter-all-documents)

<span id="equality"></span>

[Equality](/operators/query#equality)

<span id="comparison-operators"></span>

[Comparison Operators](/operators/query#comparison-operators)

<span id="array-operators"></span>

[Array Operators](/operators/query#array-operators)

<span id="logical-operators"></span>

[Logical Operators](/operators/query#logical-operators)

<span id="existence-check"></span>

[Existence Check](/operators/query#existence-check)

## Async Query Operations

Async variants are documented beside each synchronous method: [FindAsync](#findasync), [FindOneAsync](#findoneasync), [CountAsync](#countasync).
