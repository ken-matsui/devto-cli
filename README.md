# devto-cli

A CLI tool to manage dev.to articles similar to [Zenn CLI](https://www.npmjs.com/package/zenn-cli)

## Installation

You can install this using the `cargo install` command:

```bash
$ cargo install devto-cli
```

## Usage

### Configuration

You may need to export your dev.to token when using some scripts.

```bash
$ export DEVTO_TOKEN=your_token
```

Or you can pass the token as an option:

```bash
$ devto new your-new-article-title --devto-token your_token
```

### Create a new article

```bash
$ devto new your-new-article-title
```

## Contribution

### Build

```bash
$ cargo build
```

Or you can directly execute the binary:

```bash
$ cargo run
```

### Test

```bash
$ cargo build
$ cargo test
```

### Publish

#### [GitHub Releases](https://github.com/ken-matsui/jyt/tags)

```bash
$ git tag v0.1.0
$ git push origin v0.1.0
```

#### [crates.io](https://crates.io/)

```bash
$ cargo publish
```
