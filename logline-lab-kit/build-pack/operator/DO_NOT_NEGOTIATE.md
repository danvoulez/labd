# DO NOT NEGOTIATE

This is a repository recovery and assembly task.

Do not ask the user which phase to implement.
Do not ask whether to continue with A10-A14.
Do not treat acceptance-test numbers as user commands.
Do not jump to a later phase because it was mentioned.
Do not reopen the architecture.

Your first output must be classification and assembly, not a question.

Required first actions:

```txt
1. Unpack package.
2. Read build-pack first.
3. Classify source-fruits.
4. Assemble target repo tree.
5. Identify existing code.
6. Map code to modules.
7. Produce ACCEPTANCE_STATUS.md for A1-A30.
8. Identify earliest failing acceptance test.
9. Begin there.
```

If you ask “should I continue with Phase X?” before doing this, you failed.

If you are uncertain, mark a ghost.
If a field is unknown, leave it blank or set `unknown`.
If code exists but is unverified, say `exists_unverified`.
If a feature is documented but not implemented, say `documented_not_implemented`.
If a result lacks evidence, do not call it done.
