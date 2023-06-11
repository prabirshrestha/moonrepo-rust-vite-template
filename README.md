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
