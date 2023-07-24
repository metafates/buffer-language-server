# buffer-language-server

[![asciicast](https://asciinema.org/a/MiLyGWxpb6KzmHqJd3Ha1ANXm.svg)](https://asciinema.org/a/MiLyGWxpb6KzmHqJd3Ha1ANXm)

> **Warning** This is just a proof-of-concept. Does not support Unicode (yet).

Language server that can autocomplete words found in the current buffer.

This is primary made for the [Helix editor](https://github.com/helix-editor/helix)
which currently lacks this functionality. See https://github.com/helix-editor/helix/issues/1063

Currently, it can only autocomplete the words[^1] found in the current buffer.

## Install

### Cargo

```bash
cargo install buffer-language-server
```

### From source

```bash
cargo install --git https://github.com/metafates/buffer-language-server
```

## Use in your editor

### Helix

If you are using the stable version (<= 23.05), which doesn't support multiple language servers yet,
add these lines to your `languages.toml` (if you want to enable this LSP for the markdown files)

```toml
[[language]]
name = "markdown"
language-server = { command = "buffer-language-server" }
```

Otherwise (you will need the Helix editor compiled from the HEAD [latest commit])

Add these lines to your `languages.toml`

```toml
[language-server.buffer-language-server]
command = "buffer-language-server"
```

Then you can use it as an additional language server.

```toml
[[language]]
name = "markdown"
language-servers = ["buffer-language-server"]

[[language]]
name = "my-language"
language-servers = ["buffer-language-server"]
```

[^1]: "Word" is defined as a sequence of ASCII characters without whitespace nor punctuation. For example, `one,two,three four!five` contains 5 words.
