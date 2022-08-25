# futhark2latin

Transliterate Elder Futhark runes into Latin Script (and reversed)

## Usage

> **NOTE**: use hyphen (`-`) to transliterate whole sentences, for example: `lorem-ipsum`

Transliterate futhark runes to latin script:

```
cargo run --release -- --futhark <runes>
```

Transliterate latin script to futhark runes:

```
cargo run --release -- --latin <word>
```

## Web

TODO: Create WASM frontend

## License

`futhark2latin` is licensed under Mozilla Public License 2.0, see [LICENSE](LICENSE) for details.
