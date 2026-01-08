Architecture
===========

Goals
-----
- Keep crates small and focused to reduce compile times.
- Avoid cyclic dependencies between crates.
- Make IO, codecs, formats, and filters pluggable.

Dependency flow
---------------
- ferris-core is the foundation (no internal deps).
- ferris-io depends on ferris-core.
- ferris-codec depends on ferris-core and ferris-io.
- ferris-format depends on ferris-core and ferris-io (and may depend on codec).
- ferris-filter depends on ferris-core and ferris-codec.
- ferris-cli depends on the public APIs of other crates.

Crate responsibilities
----------------------
- ferris-core
  - Base media types (Frame, Packet)
  - Timebase and rational math
  - Common error and result types

- ferris-io
  - Read/write/seek traits
  - Buffering, adapters, and wrappers

- ferris-codec
  - Encoder/decoder traits
  - Codec registry and capability descriptors

- ferris-format
  - Demux/mux traits
  - Container probing and stream discovery

- ferris-filter
  - Filter graph abstraction
  - Node scheduling and media routing

- ferris-cli
  - End-user tool for probing, decoding, encoding, and filtering
