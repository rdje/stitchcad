# Browser capabilities

This is what actually ran in a browser, and what has not. It is one recorded run on one machine, so a number here is a
fact about that run rather than a promise.

The run was in **Google Chrome 156.0.8078.80** on **Darwin arm64**, headless, serving from `127.0.0.1` with no special
headers. Reproduce it with:

```text
$ bash docs/tasks/artifacts/browser_run/run_browser_run.sh
load: real Chrome compiled and instantiated the module, with no imports
evaluate: the recipe's values equal the native program's
constraint: the constraint kernel solved in the browser, equal to native
save and reopen: a second browser process reopened the edit from IndexedDB
render: 4585 dark pixels on a real canvas, from the 5 points of the inline SVG
export: the Blob's bytes equal the native program's exported file
control: a fresh profile has nothing to reopen, so IndexedDB carried the project
browser run: 6 steps passed in Google Chrome 156.0.8078.80
```

## What the run does

Two separate Chrome processes share one profile. The first evaluates [the reference skirt's](reference-skirt.md) recipe,
solves one constraint with [the kernel](constraints.md), edits the waist, and saves to IndexedDB. Then that browser
ends, and a second one reopens the project from IndexedDB, draws it and exports a file. Every transcript, the saved project
and the exported bytes must equal what the native `stitchcad` program produces from the same two sessions.

A third browser on a fresh profile must then fail its reopen. Without that failure, the run would not show that
IndexedDB carried the project.

**The drawing is a schematic.** It joins points named by evaluated values with straight lines, because drafting is G2's
job. Export writes SVG, because DXF and PDF are G2's too. The run shows the path from a recipe to a picture and a file
works in a browser, not that the picture is a pattern. Headless Chrome paints offscreen, so a test counted dark pixels
and nobody looked at it.

## The matrix

| Capability | State | What the run shows, or why not | Owner of what remains |
| --- | --- | --- | --- |
| Chrome 156 on Darwin arm64 | tested | All six steps and the control | G5-SHELLS.11 for the supported list |
| Firefox | not tested | Not run | G5-SHELLS.11 |
| Safari | not tested | Not run | G5-SHELLS.11 |
| Edge | not tested | Not run | G5-SHELLS.11 |
| WebAssembly with no host imports | tested | The module asks its host for nothing | - |
| Startup size | tested | 2,093,428 bytes uncompressed, in a release build. Not optimized for size, and not served compressed | G5-SHELLS.11 |
| Startup time | tested | 5.2 ms to compile and instantiate. One sample on one fast machine | G5-SHELLS.11 |
| COOP/COEP headers | not used | Nothing needs cross-origin isolation, so the run works without these headers. That changes if threads arrive | G5-SHELLS.11 |
| Threads and SharedArrayBuffer | not used | The page is not cross-origin isolated, so `SharedArrayBuffer` is absent. Everything runs on the main thread | G5-SHELLS.11 |
| WebGPU | not used | This Chrome offers it, but the drawing uses a 2D canvas. The baseline is undecided | G5-SHELLS.11 |
| IndexedDB | tested | One image saved in one transaction, 1,864 bytes, and reopened by a second browser process | G5-SHELLS.11 for quota and eviction |
| Canvas 2D | tested | 4,585 dark pixels read back from a real canvas | - |
| Inline SVG | tested | The drawing parsed by the browser and its points read from the DOM | - |
| Export through a Blob | tested | The Blob's bytes equal the native file. The link was built and not clicked, since headless Chrome has no download prompt | G5-SHELLS.11 |
| Cancellation | not implemented | A session runs to its end on the main thread. These take 15.5 ms and 9.6 ms, too short to need it yet | G5-SHELLS.11 |

## Limitations to carry into G5

- **Bytes cross one at a time.** That is fine for a small project and slow for a real one. [The runtime profiles chapter](runtime-profiles.md)
  explains why, and a real front end will replace it.
- **The size is unoptimized.** A release build keeps its names and panic messages, and nothing compresses it yet.
- **One tab at a time.** Nothing stops two tabs writing one project, as with two processes on [the native store](project-store.md).
- **A saved image is whole, by IndexedDB's own transaction.** The browser side relies on that rather than the store's
  journal, and the run cannot crash a browser mid-transaction to test it.
- **Eviction is untested.** A browser may clear this storage under pressure, and nothing warns the user yet.

## Names used in this chapter

| Name | Meaning |
| --- | --- |
| `SharedArrayBuffer` | The browser memory type threads need, which requires cross-origin isolation |
| `127.0.0.1` | This machine's own address, so the page has an ordinary web origin |
