# Grim Dawn Analyzer

`gd-analyzer` is a command line tool for parsing and decompressing `.arc` files
for the game [Grim Dawn](https://www.grimdawn.com/).

`gd-analyzer` is in the early stages and under active development. Part of my
motivation for creating this tool is to learn Rust, so it won't be too polished.
Currently it supports the following options:

```Usage: gd-analyzer [OPTIONS] --archive <ARCHIVE> --command <COMMAND>

Options:
  -a, --archive <ARCHIVE>
          Path of archive

  -f, --file <FILE>
          File to perform command on

  -c, --command <COMMAND>
          Command to execute

          Possible values:
          - list:    List files in archive
          - header:  Print archive header
          - decode:  Decode a specified file
          - index:   Print archive index
          - buffers: Get buffers from file index only

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version```

`decode` should be used with caution since it will panic if it encounters a
non-utf8 character. The `--file` switch is currently only used with `decode`.

## Roadmap

- [ ]Make `decode` command more robust
- [ ]Add interactive mode
- [ ]Add support for `.arz` files
- [ ]Add advanced filtering options
