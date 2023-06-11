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

Install [rustup](https://rustup.rs/).

Modify `.moon/toolchain.yml` to support rust.

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
