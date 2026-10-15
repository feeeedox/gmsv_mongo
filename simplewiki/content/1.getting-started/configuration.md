---
title: Configuration
description: Find connection settings and module utility functions
navigation:
  title: Configuration
  order: 3
---

# Configuration

Choose a connection guide below, or use the utility functions to check the module version and control console messages.

## Connection String Format

::link-card{to="/connection/basic#connection-string-formats" title="Connection Strings" description="Local servers, authentication, remote hosts, Atlas, and replica sets."}
::

<span id="examples"></span>

[Examples](/connection/basic#connection-string-examples)

## Connection Options

::link-card{to="/connection/options#available-options" title="Connection Options" description="Option types, defaults, and examples."}
::

<span id="available-options"></span>

[Available Options](/connection/options#available-options)

<span id="default-configuration-values"></span>

[Default Configuration Values](/connection/options#default-configuration-values)

## Connection Pooling

::link-card{to="/connection/options#connection-pool-behavior" title="Connection Pooling" description="Connection reuse, pool limits, and idle cleanup."}
::

## Environment-Specific Configuration

::link-card{to="/connection/options#environment-specific-examples" title="Environment Examples" description="Development, production, and Docker settings."}
::

<span id="development"></span>

[Development](/connection/options#development)

<span id="production"></span>

[Production](/connection/options#production)

<span id="dockercontainerized"></span>

[Docker/Containerized](/connection/options#docker-connection-strings)

## Utility Functions

### Version Check

```lua
print("MongoDB Module Version:", MongoDB.Version())
```

### Suppress Messages

Control console output from the module:

```lua
-- Suppress informational messages
MongoDB.SuppressMessages(true)

-- Re-enable messages
MongoDB.SuppressMessages(false)
```
