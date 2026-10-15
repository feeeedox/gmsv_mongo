---
title: Aggregation Pipelines
description: Analyze and transform data with MongoDB aggregation
navigation:
  title: Aggregation Pipelines
  order: 2
---

# Aggregation Pipelines

Aggregation pipelines are MongoDB's framework for data aggregation and transformation.

## What is Aggregation?

Aggregation operations process data records and return computed results. Documents pass through each stage in order; the final stage produces the results.

```
Documents → [Stage 1] → [Stage 2] → [Stage 3] → Results
```

## Basic Syntax

```lua
local results = collection:Aggregate(pipeline)
```

### Parameters

- `pipeline` (table): Array of aggregation stages

### Returns

- `table`: Array of result documents
- `nil`: On failure

## Aggregation Stages

For filter syntax used by `$match`, see [Query Operators](/operators/query).

### $match - Filter Documents

Filter documents early in the pipeline for efficiency:

```lua
local results = collection:Aggregate({
    {
        ["$match"] = {
            level = { ["$gte"] = 10 },
            banned = { ["$ne"] = true }
        }
    }
})
```

### $group - Group Documents

Group documents by a field and perform calculations:

```lua
-- Group by class
local byClass = collection:Aggregate({
    {
        ["$group"] = {
            _id = "$class",
            count = { ["$sum"] = 1 },
            avgLevel = { ["$avg"] = "$level" },
            totalCredits = { ["$sum"] = "$credits" },
            maxLevel = { ["$max"] = "$level" },
            minLevel = { ["$min"] = "$level" }
        }
    }
})

for _, group in ipairs(byClass) do
    print(string.format("%s: %d players, avg level %.1f",
        group._id, group.count, group.avgLevel))
end
```

::info-box{type="info"}
Use `_id = nil` (or omit) to aggregate across all documents without grouping.
::

### $sort - Sort Results

```lua
local sorted = collection:Aggregate({
    { ["$match"] = { banned = false } },
    { ["$sort"] = { score = -1, level = -1 } },  -- -1 = descending
    { ["$limit"] = 10 }
})
```

### $limit - Limit Results

```lua
local top10 = collection:Aggregate({
    { ["$sort"] = { score = -1 } },
    { ["$limit"] = 10 }
})
```

### $skip - Skip Results

```lua
-- Pagination: page 2 with 10 per page
local page2 = collection:Aggregate({
    { ["$sort"] = { score = -1 } },
    { ["$skip"] = 10 },
    { ["$limit"] = 10 }
})
```

### $project - Reshape Documents

Control which fields appear in output:

```lua
local projected = collection:Aggregate({
    {
        ["$project"] = {
            username = 1,        -- Include
            level = 1,
            score = 1,
            _id = 0,            -- Exclude
            -- Computed field
            scorePerLevel = {
                ["$divide"] = { "$score", "$level" }
            }
        }
    }
})
```

### $unwind - Deconstruct Arrays

Flatten array fields:

```lua
-- If players have inventory arrays
local items = collection:Aggregate({
    { ["$unwind"] = "$inventory" },
    {
        ["$group"] = {
            _id = "$inventory.type",
            count = { ["$sum"] = 1 }
        }
    }
})
```

### $lookup - Join Collections

Join data from another collection:

```lua
local withGuild = collection:Aggregate({
    {
        ["$lookup"] = {
            from = "guilds",
            localField = "guild_id",
            foreignField = "_id",
            as = "guild"
        }
    },
    { ["$unwind"] = "$guild" }  -- Flatten the array
})
```

### $bucket - Group into Ranges

```lua
local levelDist = collection:Aggregate({
    {
        ["$bucket"] = {
            groupBy = "$level",
            boundaries = { 0, 10, 20, 30, 40, 50 },
            default = "50+",
            output = {
                count = { ["$sum"] = 1 },
                avgCredits = { ["$avg"] = "$credits" }
            }
        }
    }
})

for _, bucket in ipairs(levelDist) do
    print(string.format("Level %s: %d players",
        tostring(bucket._id), bucket.count))
end
```

## Accumulator Operators

Use these in `$group` stages:

| Operator    | Description     | Example                        |
|-------------|-----------------|--------------------------------|
| `$sum`      | Sum values      | `{ ["$sum"] = "$credits" }`    |
| `$avg`      | Average         | `{ ["$avg"] = "$level" }`      |
| `$min`      | Minimum         | `{ ["$min"] = "$score" }`      |
| `$max`      | Maximum         | `{ ["$max"] = "$score" }`      |
| `$first`    | First value     | `{ ["$first"] = "$name" }`     |
| `$last`     | Last value      | `{ ["$last"] = "$name" }`      |
| `$push`     | Array of values | `{ ["$push"] = "$name" }`      |
| `$addToSet` | Unique array    | `{ ["$addToSet"] = "$class" }` |

## Async Aggregation

```lua
collection:AggregateAsync({
    { ["$group"] = { _id = "$class", count = { ["$sum"] = 1 } } }
}, function(err, results)
    if err then
        print("Aggregation error:", err)
        return
    end

    for _, result in ipairs(results) do
        print(result._id, ":", result.count)
    end
end)
```
### Additional Callback Example

```lua
collection:AggregateAsync({
    {
        ["$group"] = {
            _id = "$class",
            count = { ["$sum"] = 1 }
        }
    }
}, function(err, results)
    if err then
        print("Aggregation error:", err)
        return
    end

    for _, result in ipairs(results) do
        print(result._id, ":", result.count)
    end
end)
```

::see-also
---
items:
  - title: "Operators"
    to: "/operators"
  - title: "Async Operations"
    to: "/crud-operations/async"
---
::
