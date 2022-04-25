use std::fs;
use std::path::Path;

use anyhow::{bail, Result};

use crate::consts::BASE_DIR;

const REPO_NAME: &str = "devto-content";

pub(crate) fn exec() -> Result<()> {
    let repo = Path::new(REPO_NAME);
    if repo.exists() {
        bail!("Directory `{REPO_NAME}` already exists");
    }

    println!("Creating `{REPO_NAME}` ...");
    fs::create_dir(repo)?;
    let gh_action_dir = repo.join(".github").join("workflows");
    fs::create_dir_all(gh_action_dir.clone())?;
    fs::create_dir_all(repo.join(BASE_DIR))?;

    fs::write(repo.join("dev-to-git.json"), "[]")?;
    fs::write(
        gh_action_dir.join("deploy.yml"),
        r#"name: Deploy to dev.to

on:
  push:
    branches: [ main ]

jobs:
  deploy:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2

      - name: Deploy to dev.to
        run: npx dev-to-git --repository-url ${{ github.repositoryUrl }}
        env:
          DEV_TO_GIT_TOKEN: ${{ secrets.DEVTO_TOKEN }}
"#,
    )?;
    fs::write(
        repo.join("README.md"),
        r#"# DEV blog source

https://dev.to/your-dev-id

# Usage

## Configuration

You need to export your dev.to token when using some scripts.

```bash
$ export DEVTO_TOKEN=your_token
```

Or you can pass the token as an option:

```bash
$ devto new your-new-article-title --devto-token your_token
```

## Create a new article

```bash
$ devto new your-new-article-title
```

## Publish an article

> You should [set secrets](https://docs.github.com/en/actions/security-guides/encrypted-secrets#creating-encrypted-secrets-for-a-repository) on your GitHub repository

1. Change the `publish` to true in front matter
2. Commit the changes and push it to your GitHub repository

# Markdown Syntax

* https://dev.to/p/editor_guide
* https://github.com/adam-p/markdown-here/wiki/Markdown-Here-Cheatsheet
* https://github.com/maxime1992/dev.to#how-do-i-add-images-to-my-blog-posts
* `{% embed https://... %}`

# Note

* Committed new revisions, after being published, will be automatically applied by [`dev-to-git`](https://www.npmjs.com/package/dev-to-git)
"#,
    )?;

    println!("\nYou can now start writing your articles:");
    println!("    cd {REPO_NAME}");
    println!("    git init");
    println!("    devto new your-article-title");

    Ok(())
}
