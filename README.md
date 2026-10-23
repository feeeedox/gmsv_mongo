<h1 align="center">
   <img src="./.github/assets/logo.png" width="100px" /> 
   <br>
      gmsv_mongo
   <br>
   <img src="./.github/assets/bar.png" width="600px" /> 
   <br>   
   <div align="center">
      <p></p>
      <div align="center">
         <img src="https://shieldcn.dev/group/github/stars/feeeedox/gmsv_mongo+github/forks/feeeedox/gmsv_mongo+github/license/feeeedox/gmsv_mongo.svg?variant=ghost&theme=zinc">
         <p></p>
         <img src="https://shieldcn.dev/flag/de.svg?theme=zinc"> <img src="https://www.shieldcn.dev/github/release/feeeedox/gmsv_mongo.svg?size=sm&theme=zinc">
      </div>
      <br>
   </div>
</h1>


<h2>Installation & Usage</h2>

For a tutorial how to setup and use the module, check out the [installation page on the wiki](https://feeeedox.github.io/gmsv_mongo/getting-started/installation)

<h2>Examples</h2>

For some examples on how to use the module, check out the [wiki](https://feeeedox.github.io/gmsv_mongo)

<h2>Build project</h2>

To build the sample project in debug mode, you need to specify the target architecture for your build.

| Platform  |                     Command                     |                                                          Description                                                           |
|:---------:|:-----------------------------------------------:|:------------------------------------------------------------------------------------------------------------------------------:|
|  `win32`  |   `cargo build --target i686-pc-windows-msvc`   | Windows 32-bit<br>Use this if your server is running Windows on the `main` branch of Garry's Mod (this is the default branch). |
|  `win64`  |  `cargo build --target x86_64-pc-windows-msvc`  |              Windows 64-bit<br>Use this if your server is running Windows on the `x86-64` branch of Garry's Mod.               |
|  `linux`  |  `cargo build --target i686-unknown-linux-gnu`  |   Linux 32-bit<br>Use this if your server is running Linux on the `main` branch of Garry's Mod (this is the default branch).   |
| `linux64` | `cargo build --target x86_64-unknown-linux-gnu` |                Linux 64-bit<br>Use this if your server is running Linux on the `x86-64` branch of Garry's Mod.                 |

If Rust reports it cannot find the target/toolchain, you may need to install it. By default, Rust installs the native
toolchain for your system, which is likely Windows 64-bit (`x86_64-pc-windows-msvc`).

Cross-compiling Linux binaries on Windows is not recommended. For compiling Linux binaries on Windows, use WSL.