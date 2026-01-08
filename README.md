FerrisCodec
===========

Goal
----
Build a Rust-native media stack inspired by FFmpeg. The focus is a clean,
modular architecture that can grow into a large codebase while keeping
compile times and dependencies under control.

Workspace layout
----------------
- crates/ferris-core    Core media types, timebase, and shared errors.
- crates/ferris-io      IO traits (read, write, seek) and adapters.
- crates/ferris-codec   Encoder/decoder traits and codec registry.
- crates/ferris-format  Demux/mux traits and container probing.
- crates/ferris-filter  Filter graph abstractions.
- crates/ferris-cli     Command-line tool (placeholder for now).
- docs/                 Design notes and roadmap.

Notes
-----
- This is a scaffold. Each crate contains minimal types to wire the workspace.
- Add new crates under crates/ and list them in the workspace Cargo.toml.
