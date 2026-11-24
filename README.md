![Imagem do OnionPort](./Group12.png)
# OnionPort

OnionPort is the OnionLeaks project for running the Xbox Series X|S build of Grand Theft Auto VI on Windows using game files you supply. It is based on **VIctory**, which loads the title’s x86-64 executable and implements the Xbox services and graphics interfaces it calls.

Our local integration adds an Agility SDK option for Windows 10, a launcher with profile and display settings, and graphics diagnostics for investigating crashes during story loading.

## Current status

**Experimental.** The Windows diagnostics build starts the title and reaches the world-rendering stage. The NVIDIA run can still hit a Direct3D device hang during story loading. The first mission and sustained gameplay remain unverified.

| Stage | Observed in local testing |
| --- | --- |
| Build and launch | The Rust build starts the title, loads its game DLLs, and resolves their imports. |
| Graphics on Windows 10 | The Agility SDK and a local WARP software adapter produced a responsive window with draw calls. One startup sample showed about 8 FPS. |
| Story loading | The NVIDIA run reaches world-rendering commands, including compute dispatches. The latest inspected crash log records a GPU hang during this phase. |
| First mission | The opening title card and stable gameplay still need local verification. |

The hardware test used an **NVIDIA GeForce RTX 4060 Ti**. Upstream VIctory’s AMD compatibility results apply to its own build and test machine.

## Runtime capabilities

| Subsystem | Current coverage |
| --- | --- |
| Loader | VIctory loads the game executable and its DLLs in-process and resolves imports through Windows and its Xbox compatibility layer. |
| Profiles and saves | VIctory provides an offline profile and local save handling. The launcher can reuse a separate save directory. Full first-run setup still needs a complete local test. |
| Input | VIctory provides XInput controller support and a keyboard controller fallback. |
| Files and streaming | Archive reading, zlib and BCPack decoding, and tiled texture conversion come from VIctory. Story streaming still needs end-to-end verification. |
| Audio | VIctory provides XMA and Opus decoding. Audio coverage through the first mission remains unverified locally. |
| Graphics and shaders | The runtime translates console shaders to DXIL and caches shaders and pipelines. The local launcher supports a 720p test mode and targeted pipeline skips. Skipping a pipeline can leave rendering or GPU work incomplete. |
| Diagnostics | Diagnostics builds can log GPU failures, capture presented frames, and identify shader hashes associated with rendering commands. |

## Getting started

Use a source checkout containing the VIctory-based Cargo workspace. You’ll need Windows x64, the Rust version pinned in `rust-toolchain.toml`, and the Windows MSVC build tools. Hardware rendering requires Direct3D 12 with Shader Model 6.6. World rendering also uses mesh shaders.

```powershell
cargo build --locked --profile diag -p victory --features diagnostics
.\target\diag\victory.exe config
.\target\diag\victory.exe run 'D:\Games\GTA-VI\game_scarlett_master.exe' --execute
```

On a build with the local Agility integration, set `VICTORY_D3D12_SDK_PATH` to the SDK DLL directory and `VICTORY_D3D12_SDK_VERSION` to its SDK version before running. For software rendering, set `VICTORY_D3D12_WARP=1` in a diagnostics build. Our Windows 10 test used a compatible Microsoft WARP DLL alongside the executable.

The window title shows frame timing. By default, saves, settings, and `crashes.log` are kept in the title’s subdirectory under `%LOCALAPPDATA%\victory\data`. Set `VICTORY_DATA_DIR` to use a different data directory.

## Controls

With the game window focused and no controller connected, the keyboard acts as an Xbox controller.

| Keyboard | Xbox controller |
| --- | --- |
| W A S D / I J K L | Left / right stick |
| Arrow keys | D-pad |
| Enter, Backspace, X, Y | A, B, X, Y |
| Q / E, Z / C | Shoulder buttons, triggers |
| F / H | Stick clicks |
| Esc, Tab | Menu, View |

## Troubleshooting

| Setting | Effect |
| --- | --- |
| `VICTORY_LOG=info` | Enables runtime and renderer logs. |
| `VICTORY_D3D12_DRED=1` | Requests GPU breadcrumbs and fault details where supported. |
| `VICTORY_CAPTURE` | Sets the directory for captured frames in a diagnostics build. |
| `VICTORY_D3D12_WARP=1` | Uses the software adapter for diagnostic runs. |
| `VICTORY_SKIP_PIPELINES` | Takes a comma-separated list of 16-character shader hashes for targeted diagnosis. |

When reporting a GPU hang, include the GPU model, the build, the stage reached, and the line containing the failing command and its `[compute …]` or `[mesh …]` hash, if available.

## Credits and license

OnionPort is based on **VIctory**. The local integration builds on the [fxceless VIctory fork](https://github.com/fxceless/victory-gta6). Credit for the loader, Xbox compatibility layer, and shader translation goes to the VIctory contributors and the contributors to that fork.

The VIctory base is licensed under **GPL-3.0-or-later**. See `LICENSE` in the source checkout.
