Roadmap
=======

Phase 1 - Core and IO
---------------------
- Stabilize core types: Packet, Frame, TimeBase.
- Define IO traits and basic in-memory/file adapters.

Phase 2 - Format layer
----------------------
- Container probe API.
- Basic demux/mux traits.
- Minimal test container (e.g., raw or elementary stream).

Phase 3 - Codec layer
---------------------
- Encoder/decoder traits and registry.
- Reference codecs for testing (toy or raw formats).

Phase 4 - Filter graph
----------------------
- Graph API and scheduling model.
- A few built-in filters for validation.

Phase 5 - CLI
-------------
- Basic probe/inspect commands.
- Simple transcode pipeline.
