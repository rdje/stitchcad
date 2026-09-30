# A derived map's entries are orientation-sized, and its ceiling is a function of the record count

- **Type:** `decision`
- **Date:** `2026-09-30` (absolute)
- **Status:** `active`
- **Owner / source:** leaf `G0-CONTRACT.10`, which grew `KNOWLEDGE_MAP.md` to 8 128 bytes against an
  8 192-byte ceiling and had to answer for it in the same commit; the surface rows are
  `knowledge_map` and `subsystems_input` in `.doctrine/live_document_size/surfaces.tsv`

answers: "why is a Knowledge Map entry one line and not a paragraph?" · "how large may the derived map become?" · "can I compact the map generator?" · "what happens when the map reaches its ceiling?"

## The decision

**An entry in `knowledge-map/subsystems.md` is orientation-sized: a path, one clause saying what it is,
where to enter, and the owner.** Detail lives in the file the entry names, never in the entry, because
the map is a `generated_projection` that holds no unique facts and its byte budget is shared with a
line per decision record and per task tree — content the map must carry and cannot trim.

The convention is enforced by arithmetic rather than by taste: `subsystems_input` keeps its own health
target (40 lines / 3 072 bytes), and the map's size is then bounded by

```
map ≈ 380 B header + ≤3 072 B subsystems + ~55 B × trees + ~128 B × records
```

which at this commit's counts (16 trees, 22 records) is the measured 6 733 bytes — 82 % of the ceiling,
with roughly nine further records of headroom.

## Why the input is the only lever

`knowledge-map/scripts/gen_knowledge_map.sh` is classified **NEUTRAL** by `scripts/update_scaffold.sh`
(line 108 of its registry), so it is re-synced from the template and must not be patched locally — the
house rule for a neutral file is to report the divergence upstream and never to fork it. Its record lines
carry each filename twice (once as the link text, once as the target), which is why a record costs ~128
bytes instead of ~64. That is an upstream observation, recorded here for the report and not fixed here.

So the local levers are exactly two: the curated input, and the row's targets.

## The trigger, and what happens at it

- **Trigger:** the map passing **90 % of its byte ceiling**, or `subsystems_input` passing its own health.
  Derive both rather than remembering them: `wc -lc KNOWLEDGE_MAP.md knowledge-map/subsystems.md`
  against the two rows.
- **At the trigger:** re-derive the `knowledge_map` row's health from the formula above at that day's
  counts, set the ceiling at 1.6 × health (the ratio every other row uses), cite this record in the row's
  notes, and say in the commit which of the two levers moved. A ceiling rises by that derivation, never to
  land a slice.
- **Owned by:** `SPINE.5`, which owns `subsystems_input`; defect **D50** carries the state until then.

## What was measured to reach it

- `wc -lc KNOWLEDGE_MAP.md` → `99 8128` before the trim, i.e. 99 % of the ceiling, with the two subsystem
  entries added by `G0-CONTRACT.9` and `.10` accounting for ~1 350 bytes of prose that duplicated the
  chapters they point at.
- After the trim: `knowledge-map/subsystems.md` → `37` lines / `3 255` bytes (inside its line health, 106 %
  of its byte health) and `KNOWLEDGE_MAP.md` → `84` lines / `6 733` bytes. Both re-derived by
  `knowledge-map/scripts/gen_knowledge_map.sh`, whose freshness the `KNOWLEDGE-MAP` doctrine checks.
- The alternative rejected: raising the ceiling in the same commit that filled it. That is precisely the
  move `LIVE_DOCUMENT_SIZE_CONTAINMENT.md` forbids, and it would have hidden the only real finding here —
  that two verbose entries had eaten a ninth of a generated surface's budget.
