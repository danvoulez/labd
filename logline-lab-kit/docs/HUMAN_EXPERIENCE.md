# Human Experience

"The surface is flexible. The experience grammar is not" (FINAL §11). The nine
surfaces are **headless-first**: library functions on the Lab host, each returning
a stable versioned JSON read-model, wrapped by the `labkit` CLI and exposed over
MCP. A human (CLI) and an LLM (MCP) open the same Lab directory and see the same
reality. Full contract: [`SURFACES.md`](SURFACES.md).

| Surface | Function | CLI | JSON `kind` |
|---|---|---|---|
| Start | `Lab::start` | `labkit start` | `logline.view.start.v0` |
| Today | `Lab::today` | `labkit today` | `logline.view.today.v0` |
| Timeline | `Lab::timeline` | `labkit timeline` | `logline.view.timeline.v0` |
| Write | `Lab::write` | `labkit write` | `WriteOutcome` |
| Schedule | `Lab::schedule` / `schedule_view` | `labkit schedule` | `logline.view.schedule.v0` |
| Workbench | `Lab::workbench` | `labkit workbench` | `logline.view.workbench.v0` |
| Proof | `Lab::proof` | `labkit proof` | `logline.view.proof.v0` |
| Learn | `Lab::learn` | `labkit learn` | `logline.learning_report.v0` |
| Settings | `Lab::settings` | `labkit settings` | `logline.view.settings.v0` |

Runnable flows for a human and an LLM over the same Lab:
[`examples/human-and-llm/`](../examples/human-and-llm/).

Write allows ugly capture; promotion stays strict. Settings never bypass Act
discipline, proof discipline, or gate policy (A40). The Lab is a directory on disk
so retry is natural, mistakes are visible, uncertainty is explicit, experiments
are repeatable, proof is inspectable, and learning is cumulative.
