# Greeter

A Paper plugin backed entirely by Rust. Java is a thin JNI bridge with no game logic, database access, or async handling.

## Table of Contents

- [Features](#features)
- [Platform Support](#platform-support)
- [Download](#download)
- [Requirements](#requirements)
- [Installation](#installation)
- [Configuration](#configuration)
- [Architecture](#architecture)
- [Building from Source](#building-from-source)
- [Contributing](#contributing)
- [License](#license)

## Features

- `/greet <message>`: set a personal join message
- The message is shown to the player each time they join the server

## Platform Support

| OS      | x86\_64 | arm64 |
|---------|---------|-------|
| Linux   | ✓       |       |
| Windows | ✓       |       |
| macOS   | ✓       | ✓     |
| FreeBSD | ✓       |       |

The correct native library is selected and loaded automatically at runtime.

## Download

Download the latest JAR from the [Releases](https://github.com/FortiBrine/greeter/releases) page.

## Requirements

- Paper 1.16+
- Java 8+

No additional dependencies. The native library is bundled inside the JAR.

## Installation

Drop the JAR into your server's `plugins/` folder.

## Configuration

On first run, a `config.yml` is created in the plugin's data folder:

```yaml
database:
  driver: sqlite
  path: database.db
```

To use MySQL instead:

```yaml
database:
  driver: mysql
  url: mysql://user:password@localhost:3306/greeter
```

## Architecture

```
Java (thin bridge)               Rust (core)
────────────────────             ──────────────────────────────────────
GreeterPlugin.onEnable()    →    on_enable()      – config, DB, Tokio runtime init
PlayerJoinListener          →    onPlayerJoin()   – fetch stored greet message
GreetCommand                →    onGreetCommand() – persist new greet message
RustBridge (JNI glue)
```

`RustBridge` extracts the platform-specific native library from the JAR at startup and loads it via `System.load()`. From that point, all logic (async I/O, database access, configuration) runs in Rust.

The Rust side holds a `static OnceLock<GreeterPlugin>` that owns a Tokio `Runtime`, an `sqlx::AnyPool`, and the parsed config for the plugin's lifetime. `onGreetCommand` fires DB writes asynchronously so the Paper main thread is never blocked. `onPlayerJoin` uses `block_on` and keeps queries short.

## Building from Source

### Local (current OS only)

```bash
./gradlew build
```

The resulting JAR contains only the native library for your current OS.

### All platforms via GitHub Actions

Push to the repository. CI cross-compiles Rust for all platforms in parallel, then assembles a single JAR with all native libraries. The artifact is uploaded to the Actions run.

### All platforms locally

> Coming soon. Will use `cargo-zigbuild` to cross-compile all targets from a single machine, matching what CI does, without needing a push.

## Contributing

Follow [Conventional Commits](https://www.conventionalcommits.org/) for commit messages:

```
feat: add permission node for /greet
fix: handle null UUID on player join
chore: bump sqlx to 0.9
```

Common types: `feat`, `fix`, `chore`, `refactor`, `docs`, `ci`.

Open a PR against `main`. Describe what changed and why. If it touches the Rust/JNI boundary, note both sides.

## License

Licensed under either of [MIT](LICENSE-MIT) or [Apache 2.0](LICENSE-APACHE) at your option.
