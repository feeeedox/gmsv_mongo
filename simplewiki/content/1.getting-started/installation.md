---
title: Installation
description: Download and install gmsv_mongo on your Garry's Mod server
navigation:
  title: Installation
  order: 2
---

# Installation

Download the binary for your platform and install it on your Garry's Mod server.

## Prerequisites

Before installing gmsv_mongo, ensure you have:

- A Garry's Mod dedicated server
- MongoDB server (local or remote, version 4.0+)
- Basic knowledge of Lua scripting

## Download

### From GitHub Releases

1. Visit the [GitHub releases page](https://github.com/feeeedox/gmsv_mongo/releases)
2. Download the appropriate binary for your platform:

|         Platform         |                                                          Description                                                           |
|:------------------------:|:------------------------------------------------------------------------------------------------------------------------------:|
|  `gmsv_mongo_win32.dll`  | Windows 32-bit<br>Use this if your server is running Windows on the `main` branch of Garry's Mod (this is the default branch). |
|  `gmsv_mongo_win64.dll`  |              Windows 64-bit<br>Use this if your server is running Windows on the `x86-64` branch of Garry's Mod.               |
|  `gmsv_mongo_linux.dll`  |   Linux 32-bit<br>Use this if your server is running Linux on the `main` branch of Garry's Mod (this is the default branch).   |
| `gmsv_mongo_linux64.dll` |                Linux 64-bit<br>Use this if your server is running Linux on the `x86-64` branch of Garry's Mod.                 |

## Installation Steps

::::steps

:::div{title="Locate Your Server Directory"}

Find your Garry's Mod server installation:
- **Linux**: Usually `/home/steam/gmodserver/garrysmod/`
- **Windows**: Usually `C:\gmodserver\garrysmod\`

:::

:::div{title="Create the Modules Directory"}

Create the `lua/bin` directory if it doesn't exist:

```bash
mkdir -p garrysmod/lua/bin
```

:::

:::div{title="Copy the Binary"}

Copy the downloaded binary to the `lua/bin` directory:

```bash
# Linux
cp gmsv_mongo_linux64.dll garrysmod/lua/bin/

# Windows
copy gmsv_mongo_win64.dll garrysmod\lua\bin\
```

::info-box{type="important"}
The binary name must start with `gmsv_` and match your platform architecture.
::


:::

::::

## Troubleshooting

### Library Dependencies (Linux)

On Linux, you may need additional libraries:

```bash
# Debian/Ubuntu
sudo apt-get install libssl-dev

# CentOS/RHEL
sudo yum install openssl-devel
```

::link-card{to="/getting-started/first-steps" title="First Steps" description="Load the module, connect, and run your first operations."}
::
