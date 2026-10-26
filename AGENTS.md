# Repository rules

- Keep changes minimal and focused on the requested scope.
- Add focused tests for nontrivial logic changes.
- Use the `gh` CLI to interact with GitHub.

Run before committing:

```sh
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
```
