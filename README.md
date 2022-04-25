# devto-cli

A CLI tool to manage dev.to articles like [Zenn CLI](https://www.npmjs.com/package/zenn-cli)

# Usage

## Configuration

You may need to export your dev.to token when using some scripts.

```bash
$ export DEVTO_TOKEN=your_token
```

Or you can pass the token as an option:

```bash
$ cargo run new your-new-article-title --devto-token your_token
```

## Create a new article

```bash
$ cargo run new your-new-article-title
```
