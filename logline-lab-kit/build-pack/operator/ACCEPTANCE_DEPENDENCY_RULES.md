# ACCEPTANCE DEPENDENCY RULES

Acceptance tests are not menu items.

They are gates.

---

## Rule 1 — Never jump forward

If a user mentions A10-A14, do not jump to A10-A14.

First verify A1-A9.

If A1-A9 are not green, implement the earliest failing acceptance test.

---

## Rule 2 — Group by boundary

```txt
A1-A5    Act core boundary
A6-A9    local/spine/projection boundary
A10-A13 proof discipline boundary
A14-A15 pack/profile extension boundary
A16-A19 app/worker authority boundary
A20-A22 clock/report boundary
A23-A25 pack loading boundary
A26-A28 Manhattan serious proof boundary
A29-A30 recovery/debris boundary
```

---

## Rule 3 — Proof discipline

A10-A13 are one proof-boundary slice:

```txt
blocked Act appears
evidence attaches to scope
receipt candidate names exact scope
report does not pretend to be receipt
```

A14 is separate:

```txt
pack loads without changing core
```

Do not merge proof discipline with extension loading.

---

## Rule 4 — No green without evidence

A test may be green only with:

```txt
test name
command output or inspected file evidence
module responsible
scope of what was proven
```
