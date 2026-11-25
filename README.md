# lib-transform

`transform` — Handl stdlib port (pure handl).

## Layout

```text
src/        the .hnl sources (+ lib.rs, ops.rs)
pkg.hnl     package manifest — modules, deps, description, license
Cargo.toml  the crate manifest (native `transform.*` ops)
xtask/      cargo xtask gate (fmt → clippy → test → doc)
scripts/hnl-gate.sh   the gate
workflows/gate.yaml       bupha entry (handl-gates)
```

## Gate

```bash
HNL=../host-cli/target/debug/hnl scripts/hnl-gate.sh
```

The Rust crate gates via `cargo xtask gate`.

## License

MIT.
