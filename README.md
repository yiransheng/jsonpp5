# jsonpp5

A JSON pretty printer accepting `JSON5` as input.

[![asciicast](https://asciinema.org/a/GKlgiguindVmXGcdh0kt7d0SO.svg)](https://asciinema.org/a/GKlgiguindVmXGcdh0kt7d0SO)

## Use Case

Personal use case, hook up with `vim`:
```vim
autocmd FileType json setlocal formatprg=jsonpp5\ --stdin
```
Allows for editing json more of less like javascript, and format it back to JSON, see [json5](https://github.com/callum-oakley/json5-rs) for supported syntax.

## Differences with `serde_json` pretty print

Very little, mostly will try to fit small arrays in one line.

This:

```json
[ "a", "b", "c" ]
```

vs.

```json
[
    "a",
    "b",
    "c"
]
```

## Built With

Mostly glue code between [json5](https://github.com/callum-oakley/json5-rs) and [pretty](https://github.com/Marwes/pretty.rs)

## Status

Currently pretty rough, works but not optimized; and may not handle unicode well (failing a bunch of test cases found in [./tests](./tests)).