# Time and Ruler

Time is infrastructure for study, not the product identity (FINAL §10). The Lab
must confront time: ticks, due work, overdue work, reschedules, capacity.

- Clock: `crates/logline-lab-clock` (tick / due / reschedule).
- Ruler: `crates/logline-lab-ruler` (due/overdue/blocked, capacity band,
  next-study).
- **No due Act is skipped silently** (A25): every due Act receives an explicit
  disposition (executable / blocked-with-reason / reschedule).
- Capacity surfaces harmful idleness with a real next-study proposal, never fake
  busywork (A26).
