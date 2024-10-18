# stego-rust

Implementation of the Least-Significant Bit steganography method in Rust.

## Commands
```sh
Usage: steganography [OPTIONS] --input-image <INPUT_IMAGE> --output-image <OUTPUT_IMAGE> <COMMAND>

Commands:
  hide    Hide something in an existing image provided in --input-image flag
  reveal  Reveal something in an existing image provided in --input-image flag
  help    Print this message or the help of the given subcommand(s)

Options:
  -i, --input-image <INPUT_IMAGE>    Path of the image you want to hide/reveal something
  -o, --output-image <OUTPUT_IMAGE>  Path of the resulting image after hiding/revealing something
  -t, --is-text                      If you want to hide a text or an image
  -s, --text <TEXT>                  The text you want to hide [default: ]
  -n, --n-characters <N_CHARACTERS>  Number of characters you want to reveal [default: 16]
  -h, --help                         Print help
  -V, --version                      Print version
```