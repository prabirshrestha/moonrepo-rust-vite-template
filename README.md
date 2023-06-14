# moonrepo-rust-vite-template

This repo demonstrates example of a mono repo using rust and vite via moonrepo.

# Instructions for starting from scratch

## Initialize empty repo

```bash
mkdir moonrepo-rust-vite-template
cd moonrepo-rust-vite-template
git init
touch README.md
git commit -m "Initial commit"
```

Going forward we will call `moonrepo-rust-vite-template` as `ROOT` folder.

## Install moon cli

Refer to the official instructions at https://moonrepo.dev/docs/install.

## Setup moon workspace

Run the following command to initialize moon workspace.

```
moon init
```

## Enable Rust Toolchain

### Install [rustup](https://rustup.rs/).

Rustup is required by moon.

### Modify `.moon/toolchain.yml` to support rust.

```yaml
# Configures Rust within the toolchain.
rust:
  # The Rust toolchain to use. Must be a semantic version that includes major, minor, and patch.
  version: '1.70.0'

  # List of Cargo binaries to install globally and make available to tasks.
  bins:
    - systemfd
    - watchexec-cli

  # Sync the configured version above as a channel to the root `rust-toolchain.toml` config.
  syncToolchainConfig: false
```

Run `moon setup` to install the tools.

### Edit .gitignore for rust

Edit `.gitignore` to exclude rust output from git.

```gitignore
# rust
/target/
```

### Configure Cargo.toml for workspace

Create `Cargo.toml` in the root for `crates/*` folder.

```toml
[workspace]
resolver = "2"
members = [ "crates/*" ]

[workspace.package]
version = "0.1.0"
authors = ["Prabir Shrestha <mail@prabir.me>"]
edition = "2021"
description = """
moonrepo rust vite project
"""
repository = "https://github.com/prabirshrestha/moonrepo-rust-vite-template"
readme = "./README.md"
license = "MIT OR Apache-2.0"
```

### Create the first crate

```bash
cargo new --vcs=none crates/cli
```

Verify build works by running `cargo build` from root. This should generate the initial `Cargo.lock` in the root.

### Configure moon for rust projects

#### Add moon.yml for rust

Create `moon.yml` for rust in the root.

```yaml
language: 'rust'
type: 'application'

env:
  CARGO_TERM_COLOR: 'always'

fileGroups:
  sources:
    - 'crates/*/src/**/*'
    - 'crates/*/Cargo.toml'
    - 'Cargo.toml'
  tests:
    - 'crates/*/benches/**/*'
    - 'crates/*/tests/**/*'

tasks:
  server-dev:
    command: 'cargo run --package cli'
    inputs:
      - '@globs(sources)'
    local: true
  server-build:
    command: 'cargo build --release'
    inputs:
      - '@globs(sources)'
  server-check:
    command: 'cargo check --workspace'
    inputs:
      - '@globs(sources)'
  server-format:
    command: 'cargo fmt --all --check'
    inputs:
      - '@globs(sources)'
      - '@globs(tests)'
  server-lint:
    command: 'cargo clippy --workspace'
    inputs:
      - '@globs(sources)'
      - '@globs(tests)'
  server-test:
    command: 'cargo test --workspace'
    inputs:
      - '@globs(sources)'
      - '@globs(tests)'
```

### Configure rust workspace for moon

Update `.moon/workspace.yml` to include rust workspace.

```yaml
projects:
  - '.'
```

### Verify moon for rust

```bash
moon run :server-build
moon run :server-dev
moon run :server-check
moon run :server-format
moon run :server-lint
moon run :server-test
moon ci
```

## Initialize client

### Configure nodejs and pnpm via moon toolchain

Edit `.moon/workspace.yml` to include nodejs and pnpm toolchain.

```yaml
# Configures Node.js within the toolchain. moon manages its own version of Node.js
# instead of relying on a version found on the host machine. This ensures deterministic
# and reproducible builds across any machine.
node:
  # The version to use. Must be a semantic version that includes major, minor, and patch.
  # We suggest using the latest active LTS version: https://nodejs.org/en/about/releases
  version: '18.16.0'

  # The package manager to use when managing dependencies.
  # Accepts "npm" (default), "pnpm", or "yarn".
  packageManager: 'pnpm'

  # The version of the package manager (above) to use.
  pnpm:
    version: '8.6.2'

  # Version format to use when syncing dependencies within the project's `package.json`.
  dependencyVersionFormat: 'workspace'

# Configures how moon integrates with TypeScript.
typescript:
  # Update a project's `tsconfig.json` to route the `outDir` compiler option
  # to moon's `.moon/cache` directory.
  routeOutDirToCache: true
```

Run `moon setup` to download and install the toolchains.

### Initialize pnpm

```bash
pnpm init
```

### Initial pnpm workspace

Create a `pnpm-workspace.yaml` file in the root.

```yaml
packages:
  - apps/*
  - packages/*

```

### Add node_modules to gitignore

Update `node_modules/` to `.gitignore`

```gitignore
# node
node_modules/
```

### Initialize client app

Initialize client.

```bash
pnpm create vite apps/client
```

Package name: ...
Framework: React
Variant: TypeScript + SWC

### Update package name for client app

Update `name` in `apps/client/package.json` to `@moonrepo-rust-vite-template/client`.

### Create shared tsconfig

Create `tsconfig.options.json` in root based of `apps/client/tsconfig.json`.

```json
{
  "compilerOptions": {
    "target": "ES2020",
    "useDefineForClassFields": true,
    "lib": ["ES2020", "DOM", "DOM.Iterable"],
    "module": "ESNext",
    "skipLibCheck": true,

    /* Bundler mode */
    "moduleResolution": "bundler",
    "allowImportingTsExtensions": true,
    "resolveJsonModule": true,
    "isolatedModules": true,
    "noEmit": true,
    "jsx": "react-jsx",

    /* Linting */
    "strict": true,
    "noUnusedLocals": true,
    "noUnusedParameters": true,
    "noFallthroughCasesInSwitch": true
  }
}
```

Update `app/client/tsconfig.json` to reference the root tsconfig.

### Install node_modules and initialize pnpm-lock.yaml

```bash
pnpm install
```

### Run vite app directly without moon to verify

```bash
cd apps/client
pnpm dev
```

Open [http://localhost:5173/](http://localhost:5173/).

### Initialize moon for vite client app.

Create `apps/client/moon.yml`. `moon.yml` needs to be per project.

```yaml
fileGroups:
  vite:
    - 'src/**/*'
    - 'index.html'
    - 'vite.config.*'
    - 'tsconfig.json'
    - '/tsconfig.options.json'
    - 'eslintrc.*'

tasks:
  # Development server
  dev:
    command: 'pnpm dev'
    local: true
    options:
      persistent: true

  # Production build
  build:
    command: 'pnpm build'
    inputs:
      - '@group(vite)'
    outputs:
      - 'dist'

  # Preview production build locally
  preview:
    command: 'pnpm preview'
    deps:
      - '~:build'
    local: true
    options:
      persistent: true

  # Lint
  lint:
    command: 'pnpm lint'
    inputs:
      - '@group(vite)'
```

### Update moon workspace to include vite app

```yaml
projects:
  - '.'
  - 'apps/*'
```

Verify using `moon run client:dev`.

### Configure vite to ignore clearning screen

Update `apps/vite.config.ts` script to ignore clearing screen so we can see both server and client logs.

```typescript
// https://vitejs.dev/config/
export default defineConfig({
  plugins: [react()],
  clearScreen: false,
});
```

### Configure the webserver

We will install salvo and configure the webserver to server vite app when in dev mode and bundle it with the executable when in release mode.

```
cd crates/cli
cargo add tokio --features=macros
cargo add anyhow
cargo add salvo --features=compression,caching-headers,proxy,serve-static
cargo add rust-embed
cargo add listenfd
```

Update `crates/cli/src/main.rs` to add webserver.


```rust
use std::net::SocketAddr;

use salvo::prelude::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().init();

    let mut listenfd = listenfd::ListenFd::from_env();
    let (addr, listener) = if let Some(listener) = listenfd.take_tcp_listener(0)? {
        (
            listener.local_addr()?,
            tokio::net::TcpListener::from_std(listener.into()).unwrap(),
        )
    } else {
        let addr: SocketAddr = format!(
            "{}:{}",
            std::env::var("HOST").unwrap_or("127.0.0.1".into()),
            std::env::var("PORT").unwrap_or("8080".into())
        )
        .parse()
        .unwrap();
        (addr, tokio::net::TcpListener::bind(addr).await.unwrap())
    };

    tracing::info!("Listening on {}", addr);
    let acceptor = salvo::conn::tcp::TcpAcceptor::try_from(listener).unwrap();

    let router = Router::new()
        .hoop(salvo::caching_headers::CachingHeaders::default()) // CachingHeader must be before Compression.
        .hoop(salvo::compression::Compression::default().force_priority(true));

    Server::new(acceptor).serve(with_spa_router(router)).await;

    Ok(())
}

#[cfg(debug_assertions)]
fn with_spa_router(router: Router) -> Router {
    router.push(
        Router::with_path("<**rest>").handle(salvo::proxy::Proxy::new("http://localhost:5173")),
    )
}

#[cfg(not(debug_assertions))]
fn with_spa_router(router: Router) -> Router {
    #[derive(rust_embed::RustEmbed)]
    #[folder = "../../apps/client/dist"]
    struct Assets;

    router.push(
        Router::with_path("<**rest>")
            .handle(salvo::serve_static::static_embed::<Assets>().fallback("index.html")),
    )
}
```

### Configure dev server via moon

Update moon.yml to run both client and server via `moon run dev`.

```yaml
tasks:
  dev:
    command: noop
    deps:
      - client:dev
      - ~:server-dev
    local: true
    options:
      persistent: true
  server-dev:
    command: systemfd --no-pid -s http::8080 -- watchexec -r -w crates cargo run --package cli
    # command: systemfd --no-pid -s http::8080 -- cargo watch -x run --package cli
    inputs:
      - '@globs(sources)'
    local: true
    options:
      persistent: true
```

### Configure release build for client and server via moon

Update moon.yml to build both client and server via `moon run build`.

```yaml
tasks:
  build:
    command: noop
    deps:
      - client:build
      - ~:server-build
  server-build:
    command: 'cargo build --release'
    inputs:
      - '@globs(sources)'
    deps:
      - client:build
```

### Configure preview to run release build of client and server

Update moon.yml to build release mode for client and embed in server and run via `moon run preview`.

```yaml
fileGroups:
  client-dist:
    - 'apps/client/dist/**/*'

tasks:
  preview:
    command: noop
    deps:
      - ~:server-preview
    local: true
    options:
      persistent: true
  server-preview:
    command: 'cargo run --release'
    inputs:
      - '@globs(sources)'
    deps:
      - client:build
    local: true
    options:
      persistent: true
```
