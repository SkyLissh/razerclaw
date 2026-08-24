# Razerclaw 🦞

A **Linux Razer peripheral manager** built with **Rust + Tauri 2** and a **SvelteKit** frontend. It reads and writes device settings — brightness, DPI, polling rate, gamemode, power — over **D-Bus** (OpenRazer) with a clean feature-first architecture on both the Rust and Svelte side.

> **Stack:** Rust · Tauri 2 · zbus (D-Bus) · tokio · SvelteKit · Svelte 5 · TypeScript

## Why this project

This is my deep-dive into the systems + desktop stack. I wanted to build a native-feeling Linux GUI — not Electron junk — that talks directly to hardware over D-Bus. It forced me to reason about async, IPC boundaries, typed proxy layers, and a UX that feels at home on the Linux desktop. And it runs on my own machine (Fedora + Wayland), so it's a working tool, not a demo.

## Highlights

- **Rust + Tauri 2 backend** — native, small-binary, secure-by-default; no Node in the bundle.
- **Feature-first, mirrored across layers** — each device capability (`brightness`, `dpi`, `gamemode`, `power`, `misc`) is its own module on **both** sides:
  - Rust: `feature.rs` (domain), `service.rs` (logic), `proxy.rs` (D-Bus), `commands.rs` (Tauri IPC), `model.rs`
  - Svelte: `service.ts`, `stores.ts`, `queries.ts`, `schemas.ts`, `components/`
- **zbus over D-Bus** — real IPC to the OpenRazer daemon (async, `tokio`), with a typed `zbus-xml` interface.
- **Svelte 5 + runes** — modern reactivity; a full accessible UI kit (`button`, `slider`, `toggle`, `toggle-group`, `tooltip`).
- **Feature-aware UI** — capability cards per device (`brightness-card`, `dpi-card`, `poll-rate-card`), gated by a device-capability schema so the UI only shows what hardware supports.
- **Native desktop shell** — custom `titlebar` + `sidebar` blocks; `tauri-plugin-clipboard-manager` and `tauri-plugin-opener`.

## Architecture

```
src/                          # SvelteKit UI
├── features/                 # per-feature modules (mirrors the Rust side)
│   ├── brightness/           #   service, stores, queries, schemas, components
│   ├── dpi/ , gamemode/ , misc/ , power/
└── lib/components/           # ui kit + titlebar/sidebar blocks

src-tauri/src/                # Rust backend
├── features/                 # per-feature modules (mirrors the Svelte side)
│   ├── brightness/           #   feature, service, proxy (D-Bus), commands, model
│   ├── dpi/ , gamemode/ , misc/ , power/
├── lib.rs / main.rs
```

The symmetry is deliberate: a feature is the same bounded context on both sides, so adding capabilities is predictable.

## Getting started

```bash
# needs OpenRazer daemon (openrazer-daemon) running on the host
pnpm install
cargo tauri dev
```

Requires the Rust toolchain, OpenRazer/OpenRGB drivers, and a Razer device.

## What it demonstrates

- Systems programming in Rust: async D-Bus, IPC, hardware control
- Tauri 2 desktop apps with a native shell
- Structured feature architecture spanning a Rust backend + Svelte frontend
- Building real Linux desktop software (runs on Fedora + Wayland)
