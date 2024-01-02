# Rust Axum Full Course

- From [Jeremy Chone](https://www.youtube.com/watch?v=XZtlD_m59sM) thanks him.

## Start app

- Install `cargo-watch`:

```
cargo install cargo-watch
```

- Use command:

```
$cargo run
```

- Or watch `src` folder for backend:

```
cargo watch -q -c -w src/ -x run
```

## Test

- To watch `tests` forlder:

```
cargo watch -q -c -w tests/ -x "test -q quick_dev -- --nocapture"
```
