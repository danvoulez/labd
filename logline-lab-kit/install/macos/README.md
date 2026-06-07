# install/macos

v0 installs from source (requires Rust/Cargo): run `bash install/install.sh`
(macos shell / container). Pre-built macos artifacts and a native package are a
SOON distribution decision (RELEASE_SCOPE). For Docker, build inside a Rust image
and copy `target/release/labkit` + `recovery-scan`.
