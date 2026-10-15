---
title: Query Operators
description: Build query filters with comparison, logical, array, element, and evaluation operators
navigation:
  title: Query Operators
  order: 2
---

# Query Operators

Use these operators in filter tables. For method signatures and return values, see [Query Operations](/crud-operations/query).

## Quick Reference

| Group | Operators |
| --- | --- |
| [Comparison Operators](#comparison-operators) | [`$eq`](#eq-equal), [`$ne`](#ne-not-equal), [`$gt`](#gt-greater-than), [`$gte`](#gte-greater-than-or-equal), [`$lt`](#lt-less-than), [`$lte`](#lte-less-than-or-equal) |
| [Array Operators](#array-operators) | [`$in`](#in-match-any-value), [`$nin`](#nin-match-none), [`$all`](#all-match-all-values), [`$size`](#size-array-length), [`$elemMatch`](#elemmatch-element-match) |
| [Logical Operators](#logical-operators) | [`$and`](#and-all-conditions), [`$or`](#or-any-condition), [`$nor`](#nor-none-of-conditions), [`$not`](#not-negate-condition) |
| [Element Operators](#element-operators) | [`$exists`](#exists-field-exists), [`$type`](#type-field-type) |
| [Evaluation Operators](#evaluation-operators) | [`$regex`](#regex-regular-expression), [`$options`](#regex-regular-expression), [`$mod`](#mod-modulo) |

## Filter Basics

### Empty Filter (All Documents)

```lua
-- Get all documents
local all = collection:Find({})
```

### Equality

```lua
-- Exact match
local warriors = players:Find({ class = "Warrior" })

-- Match specific value
local level10 = players:Find({ level = 10 })
```

### Comparison Filters

```lua
-- Greater than
local high = players:Find({ level = { ["$gt"] = 10 } })

-- Greater than or equal
local medium = players:Find({ level = { ["$gte"] = 5 } })

-- Less than
local low = players:Find({ level = { ["$lt"] = 5 } })

-- Less than or equal
local starter = players:Find({ level = { ["$lte"] = 3 } })

-- Not equal
local notBanned = players:Find({ banned = { ["$ne"] = true } })

-- Range (combined)
local midRange = players:Find({
    level = { ["$gte"] = 10, ["$lte"] = 20 }
})
```

### Membership Filters

```lua
-- Value in array
local specific = players:Find({
    class = { ["$in"] = { "Warrior", "Mage", "Rogue" } }
})

-- Value not in array
local nonCombat = players:Find({
    class = { ["$nin"] = { "Warrior", "Mage", "Rogue" } }
})
```

### Combining Conditions

```lua
-- AND (implicit with multiple fields)
local vipWarriors = players:Find({
    class = "Warrior",
    vip = true
})

-- OR
local highRank = players:Find({
    ["$or"] = {
        { level = { ["$gte"] = 50 } },
        { vip = true }
    }
})

-- AND with OR
local complexQuery = players:Find({
    class = "Warrior",
    ["$or"] = {
        { level = { ["$gte"] = 20 } },
        { experience = { ["$gte"] = 10000 } }
    }
})
```

### Existence Check

```lua
-- Field exists
local hasInventory = players:Find({
    inventory = { ["$exists"] = true }
})

-- Field doesn't exist
local noInventory = players:Find({
    inventory = { ["$exists"] = false }
})
```

## Comparison Operators

### $eq - Equal

```lua
-- Implicit equality
collection:Find({ level = 10 })

-- Explicit $eq
collection:Find({ level = { ["$eq"] = 10 } })
```

### $ne - Not Equal

```lua
-- Find players who are not banned
collection:Find({ banned = { ["$ne"] = true } })

-- Find non-warriors
collection:Find({ class = { ["$ne"] = "Warrior" } })
```

### $gt - Greater Than

```lua
-- Players with level > 10
collection:Find({ level = { ["$gt"] = 10 } })

-- Recent logins (last 24 hours)
collection:Find({
    last_login = { ["$gt"] = os.time() - (24 * 60 * 60) }
})
```

### $gte - Greater Than or Equal

```lua
-- Players with level >= 10
collection:Find({ level = { ["$gte"] = 10 } })

-- Credits at least 1000
collection:Find({ credits = { ["$gte"] = 1000 } })
```

### $lt - Less Than

```lua
-- Players with level < 5
collection:Find({ level = { ["$lt"] = 5 } })

-- Old sessions (older than 1 hour)
collection:Find({
    created_at = { ["$lt"] = os.time() - 3600 }
})
```

### $lte - Less Than or Equal

```lua
-- Players with level <= 5
collection:Find({ level = { ["$lte"] = 5 } })
```

### Range Queries

Combine operators for ranges:

```lua
-- Level between 10 and 20 (inclusive)
collection:Find({
    level = { ["$gte"] = 10, ["$lte"] = 20 }
})

-- Credits between 1000 and 5000
collection:Find({
    credits = { ["$gte"] = 1000, ["$lt"] = 5000 }
})
```

## Array Operators

### $in - Match Any Value

```lua
-- Find specific classes
collection:Find({
    class = { ["$in"] = { "Warrior", "Mage", "Rogue" } }
})

-- Find specific players by SteamID
collection:Find({
    steamid = { ["$in"] = {
        "STEAM_0:1:11111",
        "STEAM_0:1:22222",
        "STEAM_0:1:33333"
    }}
})
```

### $nin - Match None

```lua
-- Exclude specific classes
collection:Find({
    class = { ["$nin"] = { "Warrior", "Paladin" } }
})

-- Find players not in list
collection:Find({
    steamid = { ["$nin"] = bannedSteamIds }
})
```

### $all - Match All Values

```lua
-- Players with all required achievements
collection:Find({
    achievements = { ["$all"] = { "first_kill", "level_10", "vip" } }
})
```

### $size - Array Length

```lua
-- Players with exactly 5 items
collection:Find({
    inventory = { ["$size"] = 5 }
})
```

### $elemMatch - Element Match

```lua
-- Players with high-damage weapons
collection:Find({
    inventory = {
        ["$elemMatch"] = {
            type = "weapon",
            damage = { ["$gte"] = 100 }
        }
    }
})
```

## Logical Operators

### $and - All Conditions

```lua
-- Implicit AND (multiple fields)
collection:Find({
    level = { ["$gte"] = 10 },
    vip = true,
    banned = false
})

-- Explicit $and
collection:Find({
    ["$and"] = {
        { level = { ["$gte"] = 10 } },
        { credits = { ["$gte"] = 1000 } }
    }
})

-- Useful when same field appears twice
collection:Find({
    ["$and"] = {
        { level = { ["$gte"] = 10 } },
        { level = { ["$lte"] = 20 } }
    }
})
```

### $or - Any Condition

```lua
-- VIP or high level
collection:Find({
    ["$or"] = {
        { vip = true },
        { level = { ["$gte"] = 50 } }
    }
})

-- Multiple class options
collection:Find({
    ["$or"] = {
        { class = "Warrior" },
        { class = "Paladin" },
        { class = "Knight" }
    }
})
```

### $nor - None of Conditions

```lua
-- Not banned AND not inactive
collection:Find({
    ["$nor"] = {
        { banned = true },
        { active = false }
    }
})
```

### $not - Negate Condition

```lua
-- Level not greater than 10 (i.e., <= 10)
collection:Find({
    level = { ["$not"] = { ["$gt"] = 10 } }
})
```

### Complex Combinations

```lua
-- Warriors with high level OR any VIP player
collection:Find({
    ["$or"] = {
        {
            class = "Warrior",
            level = { ["$gte"] = 20 }
        },
        { vip = true }
    }
})

-- (level >= 10 AND credits >= 1000) OR vip = true
collection:Find({
    ["$or"] = {
        {
            ["$and"] = {
                { level = { ["$gte"] = 10 } },
                { credits = { ["$gte"] = 1000 } }
            }
        },
        { vip = true }
    }
})
```

## Element Operators

### $exists - Field Exists

```lua
-- Players with inventory field
collection:Find({
    inventory = { ["$exists"] = true }
})

-- Players without email set
collection:Find({
    email = { ["$exists"] = false }
})
```

### $type - Field Type

```lua
-- Documents where level is a number
collection:Find({
    level = { ["$type"] = "number" }
})

-- Type codes: "string", "number", "object", "array", "bool", "null"
```

## Evaluation Operators

### $regex - Regular Expression

```lua
-- Names starting with "A"
collection:Find({
    username = { ["$regex"] = "^A" }
})

-- Names containing "player" (case insensitive)
collection:Find({
    username = { ["$regex"] = "player", ["$options"] = "i" }
})
```

### $mod - Modulo

```lua
-- Players with even levels
collection:Find({
    level = { ["$mod"] = { 2, 0 } }  -- level % 2 == 0
})
```

::link-card{to="/operators/update" title="Update Operators" description="Operators that change field values and arrays."}
::
