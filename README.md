# rune

A lightweight CLI tool that randomly selects an entry from a compile-time seed file using Fibonacci hashing.

## What it is

`rune` picks a single entry from a list you provide at build time. The list is embedded directly into the binary with no files, no runtime dependencies. Each invocation seeds the hash with the current time's nanosecond component, producing a fast, well-distributed selection with zero external dependencies.

## How it works

At build time, `rune` reads the file pointed to by the `SEED` environment variable and embeds its contents into the binary. At runtime, it hashes the current nanosecond timestamp using the Fibonacci hashing constant `0x9e37_79b9_7f4a_7c15`, shifts out the low-quality bits, and maps the result into the entry list.

## Prerequisites

- Rust stable
- `make`

## Installation

```sh
make build /path/to/your/seed-file
make install
```

The `SEED` path is resolved at compile time, so the binary is fully self-contained after `make install`.

## Usage

```sh
rune
```

Running `rune` prints one randomly selected entry from the seed file it was built with.

## Example seed file

An example seed file lives at [`examples/entries.txt`](examples/entries.txt). Each line is one entry.

```
dawn
dusk
ember
storm
frost
tide
gale
mist
```

Build with it to try `rune` immediately after cloning:

```sh
make build examples/entries.txt
./target/release/rune
```

## Testing

```sh
make test examples/entries.txt
```

## License

MIT, see [LICENSE](LICENSE).
