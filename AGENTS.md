# lib-transform

`transform` — Handl stdlib port: the transform port (map/filter/
reduce) — pure handl plus the Rust crate providing native
`transform.*` ops.

## Layout

| path | role |
| --- | --- |
| `src/index.hnl` | the interface |
| `src/map.hnl`, `filter.hnl`, `reduce.hnl` | the transform modules |
| `src/lib.rs`, `src/ops.rs` | the Rust crate (native `transform.*` ops) |
| `atoms.yaml`, `Cargo.toml`, `xtask/` | atoms / manifest / `cargo xtask` gate |
| `pkg.hnl`, `scripts/hnl-gate.sh`, `workflows/gate.yaml` | manifest / hnl gate / bupha entry |

## Contract

- Transitional layout: `.hnl` surface + native Rust ops behind it.

## Verification

```bash
HNL=../host-cli/target/debug/hnl scripts/hnl-gate.sh
cargo xtask gate        # the Rust crate side
```

## License

MIT.

<!-- myco:agent-awareness -->

## Agent awareness

- Session start / any work discussion: run `myco status --prompt`
  before proposing next steps — know the board first.
- User describes new work → offer to capture it (`myco todo add`,
  `issue new`, `proposal new`). Don't wait to be asked.
- Picking up a task: `myco resume <ID>` is the briefing.
- Wrapping up: `myco handoff <ID>` anything resumable; keep statuses
  honest (claimed → in-progress → blocked/done).

<!-- /myco:agent-awareness -->
