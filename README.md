# sxp-rs: SVG and PDF Conversion Tool

Welcome to `sxp`, a command-line tool for converting between SVG and PDF
formats, designed for efficiency and ease of use. This tool is ideal for
developers, designers, and anyone who needs to convert these file types as part
of their workflow.

## Features

- **Convert PDF to SVGs:** Effortlessly split a PDF into multiple SVG files, each
  representing a page from the PDF.
- **Convert SVGs to PDF:** Combine multiple SVG files into a single PDF.
- This tool is built using Cairo and Poppler, which are under the LGPL license.

## Installation

```bash
cargo install --git https://github.com/gw31415/sxp-rs
```

## Usage

```plain
Convert a PDF from/to SVG files using Cairo/Poppler

Usage: sxp <COMMAND>

Commands:
  extract   sxp extract [OPTIONS] <PATH>           Extract a single PDF file to SVG files
  merge     sxp merge [FILES]...                   Merge SVG files into a single PDF
  complete  sxp complete <SHELL>                   Generate shell completions
  help                                             Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

## License

`sxp` is open-source software and is licensed under the Apache License, Version
2.0.

## License Note

While `sxp` is licensed under the Apache-2.0 License, it depends on Cairo and
Poppler, which are licensed under the LGPL. Users should be aware of these
dependencies' licenses when using `sxp`.
