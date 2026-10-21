# dusk

**d**isk **us**age — see which folders eat your disk, with colored bars in the terminal.

![dusk demo](demo.gif)

## Install

macOS / Linux:

```sh
curl -LsSf https://github.com/krisdimitrov/dusk/releases/latest/download/dusk-installer.sh | sh
```

Windows (PowerShell):

```powershell
powershell -ExecutionPolicy Bypass -c "irm https://github.com/krisdimitrov/dusk/releases/latest/download/dusk-installer.ps1 | iex"
```

Or grab a binary from [Releases](https://github.com/krisdimitrov/dusk/releases).

## Usage

```sh
dusk ~/Downloads        # analyze a folder
dusk . -n 5             # top 5 entries only
```

## Development

```sh
cargo run -- ~/Downloads
cargo test
vhs demo.tape           # re-record demo.gif (brew install vhs)
```

Releases are built by `dist` (`.github/workflows/release.yml`). To cut one, bump
`version` in `Cargo.toml`, commit, then `git tag vX.Y.Z && git push --tags`.

## Layout

- `src/main.rs` — CLI arguments (clap)
- `src/scan.rs` — walks the filesystem and adds up sizes
- `src/render.rs` — bars, colors, human-readable sizes

## Roadmap

See [docs/ROADMAP.md](docs/ROADMAP.md).

## License

Licensed under the [MIT License](LICENSE).
