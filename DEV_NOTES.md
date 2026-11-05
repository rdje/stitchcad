# DEV_NOTES.md

Detailed technical notes — root cause, implementation, validation — per slice.

## _(2026-11-05 UTC)_ — generation2 byte boundary and exact closed Profile retention

0930's unpublished1086 draft hit generation2's decoded bound before compaction. It was16782B;
generation2 held3992084B with7916B room,with only2 of8 allowed groups in use. Candidate bytes were
copied/hashed into the protected proof store before removing that unpublished path. No live source
goal/journal was compacted until a valid retained reader result existed.
Published seal prepared generation3;one guarded install followed OS-visible idle admission. All
1539 original logical records match before/candidate/installed by name,size,lines and SHA-256.
All92 original physical files remain exact in backup;91 installed files are unchanged. Only root
index/new generation3 descriptor differ. No payload is repacked,old window rewritten or cap raised.
After installed proof,the draft restores byte-exact;three exact Git spans/11698 decodedB and public
reader bytes pass. All207 task IDs/statuses survive the two-node compaction. Whole committed
journals remain exact in1086. Source/candidate/installed proof and transaction stores are protected;
never rerun the old installer/recovery after restored drafts,later archive edits or commits.
README/LIVE unchanged,G1still6/18 In Progress. Combined Root/Profile input work is next;no new
Rust/product or power-loss proof follows from this archive transition.
Promotion declined:existing published generation/retention and book contracts own these facts.

[Whole earlier journal and navigation](docs/history/stitchcad-devnotes-part1086.md) retains exact committed source bytes from45f39bcf.
