---
title: Getting Started
description: Install and configure the MongoDB module for Garry's Mod
navigation:
  title: Getting Started
  order: 1
---

# Getting Started

Install the module, connect to MongoDB, and run Lua queries.

## What is gmsv_mongo?

gmsv_mongo is a MongoDB driver for Garry's Mod written in Rust. It provides:

- **Full CRUD Operations**: Create, Read, Update, Delete with sync and async variants
- **Aggregation Pipelines**: Filter, group, and transform documents
- **Index Management**: Optimize your queries with custom indexes
- **Connection Pooling**: Reuse connections across operations
- **Async Support**: Non-blocking operations with callback-based API

## Quick Start

```lua
-- Load the module
require("mongo")

-- Create a connection
local client = MongoDB.Client("mongodb://localhost:27017")

-- Get database and collection
local db = client:Database("gameserver")
local players = db:Collection("players")

-- Insert a document
local id = players:InsertOne({
    steamid = "STEAM_0:1:12345",
    username = "Player1",
    level = 1
})

print("Inserted player with ID:", id)
```

## Next Steps

::link-card{to="/getting-started/installation" title="Installation" description="Choose the binary for your server and install the module."}
::

::link-card{to="/getting-started/first-steps" title="First Steps" description="Connect and run your first document operations."}
::

::link-card{to="/crud-operations" title="CRUD Operations" description="Find method signatures and examples."}
::

::link-card{to="/operators" title="Operators" description="Look up filter and update syntax."}
::
