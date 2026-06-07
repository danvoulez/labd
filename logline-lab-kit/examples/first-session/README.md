# First session example

A minimal first Lab session using the demo pack and local-only profile:

```sh
labkit emit \
  --lab examples/manifests/lab.json \
  --pack packs/demo/pack.json \
  --profile profiles/local-only/profile.json \
  --act examples/acts/first.act.json
```

This emits the Act into the local outbox, syncs it to the (in-process) spine, and
renders a report from projections. It is exactly what `install/doctor.sh` runs as
its install check.
