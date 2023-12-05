# Handle Solver

A simple solver of chinese idiom wordle game, `Handle`.

## Getting Started

```
cargo run
```

- `l [pos] <pinyin>`: Locate `<pinyin>` at position `[pos]` (1 ~ 4).
- `r <pinyin>`: Remove `<pinyin>` from candidates.
- `b`: Start a new game.

## TODO

- [ ] Compile to WASM.

## Libraries Used

- [idiom-database](https://github.com/crazywhalecc/idiom-database): We use it as the idiom dataset.

## License

Please refer to the `LICENSE` file.