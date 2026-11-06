# Runtime profiles

StitchCAD builds three ways, as roadmap §7.3 lays out. Each has one command, and this table says what it holds and
what it leaves out.

| Profile | Command | Holds today | Leaves out, and why |
| --- | --- | --- | --- |
| `native-full` | `make profile-native-full` | Every crate that exists: units, the garment model, measurements, the constraint kernel, the project store, the command line and the viewer's entry | SQLite, the wgpu viewport and Tauri do not exist yet. They belong to G4, leaf `.13` and G5 |
| `native-headless` | `make profile-native-headless` | The `stitchcad` command line, for agents and CI | Any viewport, GUI or database, because a headless machine has no display |
| `wasm-viewer` | `make profile-wasm-viewer` | The garment model, measurements, the constraint kernel, the project store and session replay, as `sc_viewer.wasm` | Geometry, artifacts and the viewport do not exist yet, and direct file access, because a browser has none. Its project lives in IndexedDB as one image |

`bash scripts/check_profiles.sh` builds all three and checks those limits: the headless program pulls in no viewport,
GUI or database, and the browser artifact asks its host for nothing, so it needs no JavaScript glue beyond loading it.

```text
$ bash scripts/check_profiles.sh
native-full: every crate that exists today builds natively
native-headless: target/debug/stitchcad builds, with no viewport, GUI or database dependency
wasm-viewer: target/wasm32-unknown-unknown/debug/sc_viewer.wasm builds, with no host imports and the five exports
profiles: 3 built
```

**Compiling is not running.** [The browser capabilities chapter](browser-capabilities.md) says what actually ran, in
which browser, and what has not.

## How the browser side talks to Rust

`sc_viewer.wasm` exports five functions, and bytes cross one at a time, so no raw pointers are involved. That is slow
for a large project, and a later front end will replace it. It is the one place in the workspace allowed the
`no_mangle` attribute, and it holds no unsafe block.

| Name | What it does |
| --- | --- |
| `sc_viewer` | The crate. It runs a session against a project image |
| `sc_viewer.wasm` | The built WebAssembly module |
| `sc_input_clear` | Empties an input: 0 is the session, anything else is the image |
| `sc_input_push` | Appends one byte to an input |
| `sc_run` | Runs the session, leaving the result as the output |
| `sc_output_len` | The output's length in bytes |
| `sc_output_byte` | One byte of the output |
| `no_mangle` | The attribute that keeps a Rust function's name for WebAssembly to export |
