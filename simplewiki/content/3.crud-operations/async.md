---
title: Async Operations
description: Perform asynchronous operations on MongoDB collections
navigation:
  title: Async Operations
  order: 6
---

# Async Operations

Async operations allow your server to continue processing while database operations complete in the background.

## Why Use Async?

### Synchronous (Blocking)

```lua
-- This blocks the server until complete
local result = collection:Find({})  -- Server pauses here
-- Lua resumes after the query finishes
```

### Asynchronous (Non-Blocking)

```lua
-- This returns immediately
collection:FindAsync({}, nil, function(err, result)
    -- Called when complete
end)
-- Server continues running
```

## Callback Pattern

All async operations use callbacks with this signature:

```lua
function(err, result)
    if err then
        -- Handle error
    else
        -- Process result
    end
end
```

## Available Async Methods

Each method page includes its callback examples, optional arguments, and result details.

### Insert Operations

#### InsertOneAsync

See [InsertOneAsync](/crud-operations/insert#insertoneasync) for syntax and callback examples.

#### InsertManyAsync

See [InsertManyAsync](/crud-operations/insert#insertmanyasync) for syntax and callback examples.


### Query Operations

#### FindAsync

See [FindAsync](/crud-operations/query#findasync) for syntax and callback examples.

#### FindOneAsync

See [FindOneAsync](/crud-operations/query#findoneasync) for syntax and callback examples.

#### CountAsync

See [CountAsync](/crud-operations/query#countasync) for syntax and callback examples.


### Update Operations

#### UpdateOneAsync

See [UpdateOneAsync](/crud-operations/update#updateoneasync) for syntax and callback examples.

#### UpdateManyAsync

See [UpdateManyAsync](/crud-operations/update#updatemanyasync) for syntax and callback examples.


### Delete Operations

#### DeleteOneAsync

See [DeleteOneAsync](/crud-operations/delete#deleteoneasync) for syntax and callback examples.

#### DeleteManyAsync

See [DeleteManyAsync](/crud-operations/delete#deletemanyasync) for syntax and callback examples.


### Aggregation

#### AggregateAsync

See [AggregateAsync](/advanced/aggregation#async-aggregation) for syntax and callback examples.
