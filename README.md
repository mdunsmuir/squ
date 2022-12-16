# squ

Simple command-line utility for converting quotation marks in plaintext files to "smart quotes". A very, very, very thin wrapper around the [crowbook-text-processing](https://crates.io/crates/crowbook-text-processing) crate.

### Why?

Writing and typesetting should be two separate processes. Unfortunately, mankind has inflicted upon itself the diabolical torments of WYSIWYG/"rich text" editing, bringing writing and typesetting together in an exceedingly unhappy marriage.

If you like writing in plaintext editors with regular quotes, but need to convert your writing to "smart quotes" for publication/distribution and the tools you're using don't already support doing so, `squ` is a fast, flexible option.

## Installation

1. [Install Rust](https://www.rust-lang.org/tools/install)
2. `cargo install squ`

## Usage

`squ` attempts to convert the quotes in an existing file to "smart quotes". It can either output the result to stdout or replace the contents of the input file in place. It can't, yet, operate on stdin. Sorry.

```
$ squ --help    
Convert plain quotes (single and double) in a file to matched "smart" quotes

Usage: squ [OPTIONS] <FILE>

Arguments:
  <FILE>  The file to convert

Options:
  -i, --in-place  
  -h, --help      Print help information
  -V, --version   Print version information
```

### Example

The file `foo.txt` contains text quoted with reqular quotation marks. After processing by `squ`, those quotes have been converted to matching "smart quotes", making `foo.txt` a dirty liar. Note the use of the `-i` flag to operate on `foo.txt` in-place.

```
$ cat foo.txt 
"This file doesn't have smart quotes!"
$ squ -i foo.txt 
$ cat foo.txt   
“This file doesn’t have smart quotes!”
```

## TODO

* Inverted operation, converting "smart quotes" to plain quotes. This is really easy to do with sed and similar but could still be handy.
* Operate on stdin.
* Other related functionality: emdashes, extraneous space removal, etc.
