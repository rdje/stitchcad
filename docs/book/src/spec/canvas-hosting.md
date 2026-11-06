# Canvas hosting

Which technology should draw a pattern on screen? [ADR-0002](../governance.md) refused to assume an answer, and wrote the
corpus, the gates and the decision rule down before anybody measured anything. This chapter is the measurement.

**The verdict, from the rule and not from anybody's preference:** WebGPU in the browser, and a native wgpu surface on the
desktop. **It holds for macOS only.** The gate asks for all three desktop systems, and no Windows or Linux machine with a
display exists in this setup. If either refuses a topology, that row's eligibility reopens.

```text
$ bash docs/tasks/artifacts/canvas_spike/run_spike_verdict.sh
-- profile `browser`
  R3 dom-canvas               frame_p95_ms = 1.8 ms
  R3 webgpu-webview           frame_p95_ms = 1 ms   (inside the 10% margin)
  → winner: webgpu-webview
-- profile `native`
  R3 dom-canvas               frame_p95_ms = 3 ms
  R3 native-wgpu-composited   frame_p95_ms = 1.51 ms   (inside the 10% margin)
  R3 webgpu-webview           frame_p95_ms = 6 ms
  → winner: native-wgpu-composited
spike verdict: 5 row(s) / 2 profile(s) / two renderers: browser=webgpu-webview, native=native-wgpu-composited / 0 refusal(s)
```

## What was measured

| Row | Picks | Snaps | Worst position error | Drift | Frame p95 | Peak memory |
| --- | --- | --- | --- | --- | --- | --- |
| Canvas, in Chrome | 200 of 200 | 200 of 200 | 0.1262 px | 0 | 1.8 ms | 618 MB |
| WebGPU, in Chrome | 200 of 200 | 200 of 200 | 0.1256 px | 0 | 1 ms | 653 MB |
| Canvas, in the system webview | 200 of 200 | 200 of 200 | 0.0069 px | 0 | 3 ms | 264 MB |
| WebGPU, in the system webview | 200 of 200 | 200 of 200 | 0.1256 px | 0 | 6 ms | 263 MB |
| Native wgpu, with an HTML toolbar over it | 200 of 200 | 200 of 200 | 0.1256 px | 0 | 1.51 ms | 272 MB |

Every row passes every gate: all picks and snaps right, under half a pixel of error, zero drift, under 16.7 ms, under 1 GB.
So **any of the five could draw this corpus on this machine**, and the rule picks by speed among candidates that all have
time to spare. The measured margin is real, and on a slower machine it may matter more or less.

The machine is a Mac mini with an Apple M4 Pro and 24 GB, on macOS 27.0.1. The browser rows ran in Google Chrome 156 and the
webview rows in the system WKWebView. `docs/tasks/artifacts/canvas_spike/results.tsv` holds the raw rows and how each was taken.

## What the numbers mean

- **Position error** is where a marker's pixels actually landed, read back from the renderer, against the exact position,
  over nine zooms from 1:100 to 40:1. It is measured from drawn pixels, because a number worked out beside the renderer would
  say nothing about it.
- **Picks, snaps and drift** come from [the viewport math](viewport.md), which every renderer shares.
- **Frame time** is how long until one frame's whole picture can be read back, over 600 frames of scripted zoom and pan.
  No real frame needs that read, so this overestimates all of them. Waiting for the display is left out.
- **Peak memory** is the system's physical footprint of every process involved, in a separate visit with no read-backs.

## What this costs

The rule's fourth step prefers one renderer for both profiles, and it did not fire, because WebGPU in the system webview was
not within 10% of native wgpu on the desktop. So the rule says two renderers and two conformance surfaces.

The real cost is smaller than that sounds. wgpu implements WebGPU, and the two renderers here share the same shader and the
same drawing design. They differ in how they reach the screen. That is an observation about these two experiments, and G5
has to confirm it in a real viewport.

## What this does not show

- **Windows and Linux.** Not run. Hosted CI has no display or GPU to run them on.
- **A slow machine.** Every row had time to spare on an M4 Pro, which says little about an old laptop. G5 sets the supported
  hardware.
- **WKWebView's timer.** Its clock moves in 1 ms steps, so its two rows moved between 3 and 6 ms across runs. Chrome's clock
  and the native timer are finer, and those rows held steady.
- **The composition seam under use.** The toolbar was composed over the surface and not interacted with.
- **A real product.** The renderers are experiments in `spikes/canvas-host` and the spike's page. The product has no viewport yet.

## Reproducing it

```text
$ bash docs/tasks/artifacts/canvas_spike/run_canvas_spike.sh browser
$ bash docs/tasks/artifacts/canvas_spike/run_canvas_spike.sh native
```

The second opens real windows briefly, because measuring a native topology needs them. Each writes its rows under
`target/canvas_spike`, and `results.tsv` is the recording the verdict reads.

## Names used in this chapter

| Name | Meaning |
| --- | --- |
| `browser` | The profile that runs in a web browser |
| `native` | The profile that runs as a desktop program |
