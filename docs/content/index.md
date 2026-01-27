---
seo:
  title: gmsv_mongo Documentation
  description: Comprehensive MongoDB driver for Garry's Mod with full async support, connection pooling, and advanced database operations.
---

::u-page-hero{class="dark:bg-gradient-to-b from-neutral-900 to-neutral-950"}
---
orientation: horizontal
---
#top
:hero-background

#title
Power Your Garry's Mod Server with [MongoDB]{.text-primary}.

#description
A high-performance MongoDB driver for Garry's Mod featuring full async support, connection pooling, aggregation pipelines, index management, and comprehensive CRUD operations. Built for modern game servers.

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
Why Choose gmsv_mongo?

#links
:::u-button
---
color: neutral
size: lg
to: /getting-started/installation
trailingIcon: i-lucide-arrow-right
variant: subtle
---
Installation Guide
:::

#features
:::u-page-feature
---
icon: i-lucide-zap
---
#title
High Performance

#description
Built with Rust for maximum performance. Features connection pooling, async operations, and optimized BSON conversion for lightning-fast database interactions.
:::

:::u-page-feature
---
icon: i-lucide-database
---
#title
Full MongoDB Support

#description
Complete MongoDB feature set including aggregation pipelines, index management, bulk operations, and all query/update operators.
:::

:::u-page-feature
---
icon: i-lucide-shield
---
#title
Production Ready

#description
Robust error handling, detailed logging, configurable connection options, and backward compatibility ensure reliable operation in production environments.
:::

:::u-page-feature
---
icon: i-lucide-code
---
#title
Easy Integration

#description
Simple Lua API that feels natural in Garry's Mod. Automatic type conversion between Lua and BSON. Comprehensive examples and migration guides.
:::

:::u-page-feature
---
icon: i-lucide-cpu
---
#title
Async Operations

#description
Non-blocking database operations prevent server lag. Connection pooling and retry mechanisms handle high-load scenarios gracefully.
:::

:::u-page-feature
---
icon: i-lucide-git-branch
---
#title
Migration Friendly

#description
Easy migration from older versions with detailed guides. Maintain data integrity and minimize downtime during upgrades.
:::
::

::u-page-section{class="dark:bg-neutral-950"}
#title
Key Features

#links
:::u-button
---
color: neutral
size: lg
target: _blank
to: /crud-operations/overview
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
Complete Create, Read, Update, Delete operations with support for single and bulk operations. FindOne, Count, and advanced query operators.
:::

:::u-page-feature
---
icon: i-lucide-bar-chart-3
---
#title
Aggregation Pipelines

#description
Powerful data analysis with MongoDB's aggregation framework. Group, sort, filter, and transform data with complex pipelines.
:::

:::u-page-feature
---
icon: i-lucide-search
---
#title
Index Management

#description
Create, list, and drop indexes for query optimization. Support for unique, compound, and custom indexes to improve performance.
:::

:::u-page-feature
---
icon: i-lucide-server
---
#title
Connection Management

#description
Flexible connection options with authentication, TLS, connection pooling, and retry policies. Multiple database and collection operations.
:::

:::u-page-feature
---
icon: i-lucide-file-text
---
#title
Type Safety

#description
Automatic conversion between Lua tables and BSON documents. Support for ObjectIds, dates, and all MongoDB data types.
:::

:::u-page-feature
---
icon: i-lucide-alert-triangle
---
#title
Error Handling

#description
Comprehensive error reporting with detailed messages. Configurable logging levels and graceful failure handling.
:::
::

::u-page-section{class="dark:bg-gradient-to-b from-neutral-950 to-neutral-900"}
:::u-page-c-t-a
---
links:
    - label: Start Building
      to: '/getting-started'
      trailingIcon: i-lucide-arrow-right
    - label: View Examples
      to: '/examples'
      trailingIcon: i-lucide-code
    - label: API Reference
      to: '/api-reference'
      trailingIcon: i-lucide-book-open
    title: Ready to enhance your Garry's Mod server?
    description: Join other server owners using gmsv_mongo for reliable, high-performance database operations. Get started with comprehensive documentation and examples.
    class: dark:bg-neutral-950
---

:stars-bg
:::
::