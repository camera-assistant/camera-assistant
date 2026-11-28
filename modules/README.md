# Detection modules

Each folder here holds one open detection module, named `ca-detect-<target>`: one binary that runs object detection on one kind of hardware, such as the CPU or Intel graphics.

A module runs as a separate process next to the server and passes the conformance kit in `conformance/`. Modules that need a vendor's closed runtime live in a repository of their own, not here.

Every folder here must hold a `Cargo.toml`, because the workspace includes `modules/*`.
