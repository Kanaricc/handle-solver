# Handle Solver

A simple solver of chinese idiom wordle game, `Handle`.

## Getting Started

```
cargo run
```

- `l [pos] <pinyin>`: Locate `<pinyin>` at position `[pos]` (0 ~ 4).
- `r <pinyin>`: Remove `<pinyin>` from candidates.
- `b`: Start a new game.

## TODO

- [ ] Compile to WASM.

## Libraries Used

- [idiom-database](https://github.com/crazywhalecc/idiom-database)

## License

Please refer to the `LICENSE` file.