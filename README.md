# Ply for Zed

Syntax highlighting and language-server launch for the Ply language.

The extension id is `ply`. It registers the suffixes `.ply` and `.d.ply`.

## Grammar

Highlighting comes from [tree-sitter-ply](https://github.com/algosail/tree-sitter-ply). Zed clones the revision named in `extension.toml` and compiles `src/parser.c`. This repository does not vendor the grammar.

## Language server

The extension does not bundle `plylang-ls`. It starts a server in this order:

1. `PLYLANG_LS_BIN`, when that variable is set in the shell environment Zed sees.
2. `target/debug/plylang-ls`, when the open worktree is the plylang compiler (`Cargo.toml` lists `crates/plylang-ls`).
3. `plylang-ls` on `PATH`.

Build the server from the compiler repository with `cargo build -p plylang-ls`. Reloading the extension does not restart a running server. From a `.ply` buffer, run `language server: restart`.

## Develop

Run `zed: install dev extension` and choose this directory. Rust and the `wasm32-wasip2` target are required. Zed compiles the extension.

## Publish

The catalog entry is a pull request to [zed-industries/extensions](https://github.com/zed-industries/extensions). This repository has to be public first. From a fork of that catalog:

```sh
git submodule add https://github.com/algosail/zed-ply.git extensions/ply
```

Add this to `extensions.toml`. The version must match `extension.toml` at the submodule commit:

```toml
[ply]
submodule = "extensions/ply"
version = "0.0.1"
```

Run `pnpm sort-extensions` in the catalog. The submodule URL has to be HTTPS, and the checked-out commit has to be on a branch.
