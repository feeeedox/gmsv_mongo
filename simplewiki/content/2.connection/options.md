---
title: Connection Options
description: Configure your MongoDB connection with custom options
navigation:
  title: Connection Options
  order: 3
---

# Connection Options

Use `MongoDB.ClientWithOptions()` for advanced connection configuration.

## Basic Usage

```lua
local client = MongoDB.ClientWithOptions("mongodb://localhost:27017", {
    app_name = "MyGModServer",
    max_pool_size = 50,
    retry_writes = true
})
```

## Available Options

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `app_name` | string | `"gmsv_mongo_v2"` | Application name for server logs |
| `max_pool_size` | number | `100` | Maximum connections in pool |
| `retry_writes` | boolean | `true` | Retry failed write operations |


### app_name

Application name for server monitoring and logging.

```lua
local client = MongoDB.ClientWithOptions("mongodb://localhost:27017", {
    app_name = "MyGameServer"
})
```

- **Type**: string
- **Default**: `"gmsv_mongo_v2"`
- **Purpose**: Appears in MongoDB server logs and monitoring tools

### max_pool_size

Maximum number of connections in the connection pool.

```lua
local client = MongoDB.ClientWithOptions("mongodb://localhost:27017", {
    max_pool_size = 100
})
```

- **Type**: number
- **Default**: `100`
- **Purpose**: Controls concurrent connection capacity

### retry_writes

Automatically retry failed write operations.

```lua
local client = MongoDB.ClientWithOptions("mongodb://localhost:27017", {
    retry_writes = true
})
```

- **Type**: boolean
- **Default**: `true`
- **Purpose**: Retries eligible writes after transient network failures

## Complete Options Example

```lua
local client = MongoDB.ClientWithOptions("mongodb://localhost:27017", {
    -- Application identification
    app_name = "MyGModServer",

    -- Connection pool configuration
    max_pool_size = 50,

    -- Retry behavior
    retry_writes = true
})

if client then
    print("✓ Connected with custom options!")
end
```

## Internal Configuration

The library uses these internal defaults (from Rust configuration):

| Setting | Default Value | Description |
|---------|---------------|-------------|
| `min_pool_size` | 10 | Minimum connections maintained |
| `max_pool_size` | 100 | Maximum connections allowed |
| `server_selection_timeout` | 30s | Time to select a server |
| `connect_timeout` | 10s | Connection establishment timeout |
| `max_idle_time` | 600s | Close idle connections after |
| `retry_writes` | true | Retry failed writes |
| `retry_reads` | true | Retry failed reads |

### Default Configuration Values

The same defaults as Lua values:

```lua
-- Default connection pool settings
max_pool_size = 100
min_pool_size = 10

-- Default timeouts
server_selection_timeout = 30  -- seconds
connect_timeout = 10           -- seconds
max_idle_time = 600            -- seconds (10 minutes)

-- Default retry behavior
retry_writes = true
retry_reads = true
```

## Connection Pool Behavior

Each client manages a connection pool and reuses connections across operations:

```lua
-- Configure pool size based on your server's needs
local client = MongoDB.ClientWithOptions("mongodb://localhost:27017", {
    max_pool_size = 50,
})

-- Connections are automatically:
-- - Reused across operations
-- - Managed for health and timeouts
-- - Cleaned up when idle
```


### How Pooling Works

1. **Initial Connection**: First operation creates connection(s)
2. **Reuse**: Subsequent operations reuse existing connections
3. **Scaling**: Pool grows based on demand (up to `max_pool_size`)
4. **Cleanup**: Idle connections are closed after `max_idle_time`

## Environment-Specific Examples

### Development

```lua
local client = MongoDB.ClientWithOptions("mongodb://localhost:27017", {
    app_name = "GModDev",
    max_pool_size = 10
})
```

### Production

```lua
local client = MongoDB.ClientWithOptions("mongodb://prod-user:secure-pass@mongo.example.com:27017", {
    app_name = "GModProd_Server1",
    max_pool_size = 75,
    retry_writes = true
})
```

### Another Production Configuration

```lua
local client = MongoDB.ClientWithOptions("mongodb://user:pass@prod-server:27017", {
    app_name = "GModProd",
    max_pool_size = 100,
    retry_writes = true
})
```

### Docker Environment

```lua
-- When MongoDB is in the same Docker network
local client = MongoDB.ClientWithOptions("mongodb://mongo:27017", {
    app_name = "GModDocker",
    max_pool_size = 50
})
```

### Docker Connection Strings

```lua
-- Use service name when MongoDB is in same Docker network
local client = MongoDB.Client("mongodb://mongo:27017")

-- Or with authentication
local client = MongoDB.Client("mongodb://root:example@mongo:27017")
```

## Monitoring Connections

### Check Connection Health

```lua
local function checkConnection(client)
    if not client then
        return false, "No client"
    end

    local databases = client:ListDatabases()
    if databases then
        return true, "Connected (" .. #databases .. " databases)"
    else
        return false, "Connection test failed"
    end
end

-- Usage
local ok, message = checkConnection(client)
print(ok and "✓" or "✗", message)
```

### Connection in Server Logs

With `app_name` set, you'll see entries in MongoDB logs:

```json
{
  "t": {
    "$date": "2024-01-15T10:30:00.000+00:00"
  },
  "s": "I",
  "c": "NETWORK",
  "msg": "Connection accepted",
  "attr": {
    "appName": "MyGModServer"
  }
}
```
