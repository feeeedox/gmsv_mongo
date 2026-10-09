---
seo:
  title: gmsv_mongo Documentation
  description: MongoDB driver for Garry's Mod with sync and async operations, connection pooling, aggregation, and indexes.
---

::u-page-hero{class="dark:bg-gradient-to-b from-neutral-900 to-neutral-950"}
---
orientation: horizontal
---
#top
:hero-background

#title
[MongoDB]{.text-primary} for Garry's Mod

#description
gmsv_mongo is a Rust module that exposes MongoDB operations to Lua. Use synchronous calls or asynchronous calls with callbacks.

#links
  :::u-button
  ---
  to: /getting-started
  size: xl
  trailing-icon: i-lucide-arrow-right
  ---
  Get started
  :::

  :::u-button
  ---
  icon: i-simple-icons-github
  color: neutral
  variant: outline
  size: xl
  to: https://github.com/feeeedox/gmsv_mongo
  target: _blank
  ---
  View on GitHub
  :::

#default
  :::prose-pre
  ---
  code: |
    local client = MongoDB.Client("mongodb://localhost:27017")
    local db = client:Database("gameserver")
    local players = db:Collection("players")

    -- Insert a player
    local id = players:InsertOne({
        steamid = "STEAM_0:1:12345",
        username = "Player1",
        level = 5
    })

    -- Find players
    local results = players:Find({ level = { ["$gte"] = 5 } })
  filename: example.lua
  ---

  ```lua [example.lua]
  local client = MongoDB.Client("mongodb://localhost:27017")
  local db = client:Database("gameserver")
  local players = db:Collection("players")

  -- Insert a player
  local id = players:InsertOne({
      steamid = "STEAM_0:1:12345",
      username = "Player1",
      level = 5
  })

  -- Find players
  local results = players:Find({ level = { ["$gte"] = 5 } })
  ```
  :::
::

::u-page-section{class="dark:bg-neutral-950"}
#title
Supported Operations

#links
  :::u-button
  ---
  color: neutral
  size: lg
  to: /crud-operations
  trailingIcon: i-lucide-arrow-right
  variant: subtle
  ---
  Learn CRUD Operations
  :::

#features
  :::u-page-feature
  ---
  icon: i-lucide-plus-circle
  ---
  #title
  CRUD Operations

  #description
  Insert, find, update, and delete documents. Sync and async variants are available for individual documents and batches.
  :::

  :::u-page-feature
  ---
  icon: i-lucide-bar-chart-3
  ---
  #title
  Aggregation Pipelines

  #description
  Run MongoDB pipelines to filter, group, sort, and transform documents.
  :::

  :::u-page-feature
  ---
  icon: i-lucide-search
  ---
  #title
  Index Management

  #description
  Create, list, and drop indexes, including unique and compound indexes.
  :::

  :::u-page-feature
  ---
  icon: i-lucide-server
  ---
  #title
  Connection Management

  #description
  Connect using MongoDB connection strings and configure the application name, connection pool size, and write retries.
  :::

  :::u-page-feature
  ---
  icon: i-lucide-file-text
  ---
  #title
  Lua and BSON

  #description
  Convert Lua tables to BSON documents and query results back to Lua tables. ObjectIds and dates have dedicated representations.
  :::

  :::u-page-feature
  ---
  icon: i-lucide-alert-triangle
  ---
  #title
  Error Handling

  #description
  Async callbacks receive an error string as their first argument on failure, or nil on success.
  :::
::
