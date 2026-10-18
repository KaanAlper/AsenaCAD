# Never crash

People trust AsenaCAD with their drawings; a crash loses their work. This outranks feature work.

1. **No panics in non-test code.** No `unwrap()`, `expect()`, `panic!`, `unreachable!`, `todo!`,
   `unimplemented!`. Every production crate root carries
   `#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::unimplemented, clippy::todo, clippy::unreachable)]`.
2. **No `unsafe`.** The workspace sets `unsafe_code = "forbid"`.
3. **Errors are values.** `Result<T, E>` with the crate's error type and `?`. An unfinished feature
   returns an error or says "not available yet".
4. **Input is hostile.** DXF/DWG files, command-line text and MCP/control parameters: `get()` not
   `[i]`, checked arithmetic, no NaN-to-int casts, caps on input-sized allocations and loops,
   bounded recursion (`MAX_BLOCK_DEPTH`).
5. **Last-resort guard.** `Session::execute` and every step of an interactive command run under
   `catch_unwind` (`crates/engine/src/guard.rs`); an escaped panic restores the drawing and
   reports an error.
6. **Prove it.** Every crash fix lands with a regression test (pattern: `hostile_params_never_panic`,
   `hostile_dxf_does_not_panic`).
