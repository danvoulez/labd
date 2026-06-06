# FIRST RUN PROTOCOL

Before writing feature code, produce three documents.

---

## 1. SOURCE_FRUITS_CLASSIFICATION.md

For every extracted source fruit:

```txt
path:
what_it_appears_to_be:
classification:
  core_candidate | pack_candidate | profile_candidate | runtime_candidate |
  adapter_candidate | deployment_candidate | documentation_candidate |
  recovery_only | debris
useful_files:
dangerous_or_stale_files:
proposed_destination:
action:
  promote | recover | keep_raw | mark_debris | delete_generated_junk
notes:
```

Classification is not promotion.

A file is not authority because it exists.

---

## 2. REPO_ASSEMBLY_PLAN.md

Must include:

```txt
target tree
folders already present
folders to create
source fruits feeding each target folder
items that remain ghosted
items explicitly not copied
```

No implementation before this exists.

---

## 3. ACCEPTANCE_STATUS.md

For A1-A30:

```txt
id:
status: green | red | unknown | blocked
evidence:
test_command:
responsible_module:
notes:
```

No acceptance test is green without command output or inspected evidence.

---

## Then implement

Start with the earliest acceptance test that is not green.

Never skip dependency order.
