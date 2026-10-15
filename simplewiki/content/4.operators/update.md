---
title: Update Operators
description: Reference for field and array updates
navigation:
  title: Update Operators
  order: 3
---

# Update Operators

Use these in the update table passed to [UpdateOne or UpdateMany](/crud-operations/update). Query operators belong in the filter table.

## Quick Reference

| Operator    | Description                  |
|-------------|------------------------------|
| [`$set`](#set-set-field-values) | Set field value              |
| [`$unset`](#unset-remove-fields) | Remove field                 |
| [`$inc`](#inc-increment-values) | Increment by value           |
| `$mul`      | Multiply by value            |
| `$min`      | Update if less than          |
| `$max`      | Update if greater than       |
| [`$push`](#push-add-to-array) | Add to array                 |
| [`$pull`](#pull-remove-from-array) | Remove from array            |
| [`$addToSet`](#addtoset-add-unique-to-array) | Add unique to array          |
| `$pop`      | Remove first/last from array |
| [`$each`](#push-add-to-array) | Add multiple values in the `$push` and `$addToSet` examples |
| [`$setOnInsert`](/crud-operations/update#upsert-pattern) | Set fields in the upsert example |

`$each` is used in the array examples below. The [upsert pattern](/crud-operations/update#upsert-pattern) shows `$setOnInsert`.

## Field Operators

### $set - Set Field Values

```lua
-- Set single field
{ ["$set"] = { level = 10 } }

-- Set multiple fields
{ ["$set"] = {
    level = 10,
    experience = 5000,
    last_updated = os.time()
}}

-- Set nested field
{ ["$set"] = { ["stats.kills"] = 100 } }
```

### $inc - Increment Values

```lua
-- Increment by positive value
{ ["$inc"] = { credits = 100 } }

-- Decrement (negative value)
{ ["$inc"] = { credits = -50 } }

-- Multiple increments
{ ["$inc"] = {
    kills = 1,
    experience = 100,
    credits = 50
}}
```

### $unset - Remove Fields

```lua
-- Remove a field
{ ["$unset"] = { temporary_buff = "" } }

-- Remove multiple fields
{ ["$unset"] = {
    old_field = "",
    deprecated_data = ""
}}
```

## Array Operators

### $push - Add to Array

```lua
-- Add item to array
{ ["$push"] = { inventory = "sword" } }

-- Add multiple items
{ ["$push"] = {
    inventory = { ["$each"] = { "sword", "shield", "potion" } }
}}
```

### $pull - Remove from Array

```lua
-- Remove specific value
{ ["$pull"] = { inventory = "broken_sword" } }

-- Remove matching condition
{ ["$pull"] = { inventory = { rarity = "common" } } }
```

### $addToSet - Add Unique to Array

```lua
-- Add only if not exists
{ ["$addToSet"] = { achievements = "first_kill" } }

-- Multiple unique values
{ ["$addToSet"] = {
    tags = { ["$each"] = { "vip", "beta_tester" } }
}}
```

## Combined Operators

```lua
-- Multiple operations in one update
collection:UpdateOne(
    { steamid = "STEAM_0:1:12345" },
    {
        ["$set"] = {
            last_login = os.time(),
            online = true
        },
        ["$inc"] = {
            login_count = 1,
            credits = 10
        },
        ["$push"] = {
            login_history = os.time()
        }
    }
)
```

::link-card{to="/crud-operations/update#upsert-pattern" title="Upsert Pattern" description="Create or update a document in one operation."}
::
