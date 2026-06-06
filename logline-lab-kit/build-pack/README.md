# Build Pack README

This folder is the build-control surface for `logline-lab-kit`.

Use `OPERATOR_PROMPT_V2.md` as the controlling prompt.

The first implementation agent must not choose a phase. It must first produce:

```txt
SOURCE_FRUITS_CLASSIFICATION.md
REPO_ASSEMBLY_PLAN.md
ACCEPTANCE_STATUS.md
```

Only then may it implement the earliest failing acceptance test.
