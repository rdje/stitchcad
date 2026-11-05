# Host RAM: observe the trend, flag only a sustained rise above 90%

- **Type:** `decision`
- **Date:** `2026-11-06` (absolute)
- **Status:** `active`
- **Owner / source:** director instruction, 2026-11-06, recorded by leaf `G1-SLICE.5f.2.prepush-400.ram-observe`

answers: "how much RAM may a job use?" · "when do I stop or defer work for memory?" · "what do I do when the host is above 86%?"

## The fact / decision

Watch the host RAM trend. No percentage stops, defers or admits work. Only when usage has crossed 90% and
keeps increasing, tell the director that there may be a situation. Otherwise carry on without commentary.

## Why

The director's words: "Just observe the trend, it shouldn't cross 90% and still increasing. don't stress
out." Earlier thresholds (the 2026-11-05 88% ceiling and the session prompt's 86%/90% reduce-and-stop rules)
stalled compilation at 88.8% over a reading that fell to 58% within minutes, because other projects' jobs
come and go. This supersedes those thresholds; D380's cache-adjusted metric still defines "used".

## How to apply

- Start Rust and probe jobs without an admission check; choose ordinary, conservative worker counts.
- Sample with `scripts/macos_memory.py` while they run. Judge successive readings, not one.
- Above 90% and rising: say so, with the readings. Never touch other projects' processes.
