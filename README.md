# buffer-language-server

[![asciicast](https://asciinema.org/a/MiLyGWxpb6KzmHqJd3Ha1ANXm.svg)](https://asciinema.org/a/MiLyGWxpb6KzmHqJd3Ha1ANXm)

> **Warning** This is just a proof-of-concept. Does not support Unicode (yet).

Language server that can autocomplete words found in the current buffer.

This is primary made for the [Helix editor](https://github.com/helix-editor/helix)
which currently lacks this functionality. See https://github.com/helix-editor/helix/issues/1063

Currently, it can only autocomplete the words[^1] found in the current buffer.

[^1]: "Word" is defined as a sequence of ASCII characters without whitespace nor punctuation.

  For example, `one,two,three four!five` contains 5 words.
