# devto-cli [![crates.io version](https://img.shields.io/crates/v/devto-cli.svg)](https://crates.io/crates/devto-cli) [![crates.io downloads](https://img.shields.io/crates/d/devto-cli.svg)](https://crates.io/crates/devto-cli)

A CLI tool to manage dev.to articles similar to [Zenn CLI](https://www.npmjs.com/package/zenn-cli)

## Installation

You can install this using the `cargo install` command:

```bash
$ cargo install devto-cli
```

## Usage

### Configuration

You need to export your dev.to token to use.

```bash
$ export DEVTO_TOKEN=your_token
```

Or you can pass the token as an option:

```bash
$ devto new your-new-article-title --devto-token your_token
```

#### Obtain a dev.to token

1. Go to `Settings` > `Account` > `DEV Community API Keys`
2. Enter description and click on `Generate API Key`

### Generate a repository template

```bash
$ devto start
Creating `devto-content` ...

You can now start writing your articles:
    cd devto-content
    git init
    devto new your-article-title
```

### Create a new article

```bash
devto-content/$ devto new your-new-article-title
```

## Delete an unpublished article

```bash
devto-content/$ devto delete your-unpublised-article-title
```

## (Coming soon) Preview the article

```bash
devto-content/$ devto preview your-draft-article-title
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
