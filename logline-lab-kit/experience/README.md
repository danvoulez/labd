# experience/

The nine experience surfaces are **not** directories of UI code. They are realized
headless-first:

- library functions on the Lab host: `crates/logline-lab-labd/src/experience.rs`
- stable JSON read-models (each tagged `logline.view.*.v0`)
- `labkit` commands: start / today / timeline / write / schedule / workbench /
  proof / learn / settings / tick / storage
- MCP read exposure: `apps/mcp-server`

Contract: `docs/SURFACES.md`. This directory exists to make the mapping explicit;
a GUI/TUI/ChatGPT-App would live here later as a *consumer* of those contracts.
