---
title: Connection
description: Manage MongoDB connections with gmsv_mongo
navigation:
  title: Connection
  order: 1
---

# Connection Management

Create a client to access databases and collections.

## Overview

gmsv_mongo provides two methods for creating connections:

- `MongoDB.Client()` - Simple connection with default settings
- `MongoDB.ClientWithOptions()` - Advanced connection with custom options

## Connection Lifecycle

```lua
-- 1. Create connection
local client = MongoDB.Client("mongodb://localhost:27017")

-- 2. Use connection for operations
local db = client:Database("mydb")
local collection = db:Collection("mycollection")

-- 3. Connection is managed automatically
-- No explicit close/disconnect needed
```

## Quick Reference

| Method                                    | Description                    | Returns              |
|-------------------------------------------|--------------------------------|----------------------|
| `MongoDB.Client(uri)`                     | Create basic connection        | MongoDBClient or nil |
| `MongoDB.ClientWithOptions(uri, options)` | Create connection with options | MongoDBClient or nil |
| `client:Database(name)`                   | Get database reference         | MongoDBDatabase      |
| `client:ListDatabases()`                  | List all databases             | table of strings     |

## Topics

::see-also
---
items:
- title: "Basic Connection"
  to: "/connection/basic"
  description: "Simple connection setup and usage."
- title: "Connection Options"
  to: "/connection/options"
  description: "Configure your MongoDB connection with custom options."
- title: "Database Operations"
  to: "/connection/database"
  description: "Select databases, manage collections, and inspect statistics."
---
::
