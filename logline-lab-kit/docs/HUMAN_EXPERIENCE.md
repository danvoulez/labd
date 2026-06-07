# Human Experience

"The surface is flexible. The experience grammar is not" (FINAL §11). The nine
surfaces are implemented as library functions on the Lab host and wrapped by the
`labkit` CLI; MCP/web/TUI can wrap the same grammar.

| Surface | Function | CLI |
|---|---|---|
| Start | `Lab::start` | `labkit start` |
| Today | `Lab::today` | `labkit today` |
| Timeline | `Lab::timeline` | `labkit timeline` |
| Write | `Lab::write` | `labkit write` |
| Schedule | `Lab::schedule` | (via write/session) |
| Workbench | `Lab::workbench` | (library) |
| Proof | `Lab::proof` | (library) |
| Learn | `Lab::learn` | `labkit learn` |
| Settings | `Lab::settings` | `labkit settings` |

Write allows ugly capture; promotion stays strict. Settings never bypass Act
discipline, proof discipline, or gate policy (A40).
