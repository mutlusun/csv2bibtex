# csv2bibtex

A small command line utility to parse CSV files to Bib(La)TeX files.


## Usage

```
csv2bibtex 0.2.0
mutluyuz


USAGE:
    csv2bibtex [OPTIONS] <INPUT> <OUTPUT>

ARGS:
    <INPUT>     Input file to use
    <OUTPUT>    Output file to use

OPTIONS:
        --biblatex                  Print output in BibLaTeX mode (default)
        --bibtex                    Print output in BibTeX mode
    -d, --delimiter <DELIMITER>     Delimiter between cells in CSV file
    -f, --field-mapping <FIELD>     Assignment of csv fields to bibtex fields
    -h, --help                      Print help information
    -l, --lazy                      Try to recover from as much errors as possible.
        --no-defaults               Don't add default field mappings and verbatim fields.
    -v, --verbosity <LEVEL>         Verbosity level, either DEBUG, INFO, WARN, or ERROR
    -V, --version                   Print version information
        --verbatim-field <FIELD>    Bib(La)TeX verbatim fields, like url, file or doi
```

Usage is really intuitive: `csv2bibtex INPUTFILE OUTPUTFILE`. CSV fields can be 
mapped to BibTeX fields with the `--field-mapping` argument. In the following 
example, the CSV field `AUTH` is mapped to the BibTeX field `author`:

```
csv2bibtex \
    --field-mapping "author=[[AUTH]]" \
    input.csv \
    output.bib
```

The CSV field has to be enclosed betwee `[[` and `]]`. This offers the 
possibility to add regular characters like in the following example:

```
csv2bibtex \
    --field-mapping "pages=[[StartPage]]--[[EndPage]]" \
    --field-mapping "journal=My Great Journal" \
    input.csv \
    output.bib
```

There are two special fields: `entrytype` and `bibtexkey`. The former specifies 
the type of the BibTeX entry, the latter the BibTeX key. They can be used like 
any other field (see above). In addition, there are some default field mappings 
set (like `title=[[titles]]`, use `--no-defaults` to prevent this).
The field mapping argument can be given multiple times to map multiple fields.

The `--verbatim-field` argument can be used to not escape a certain field
(e.g., `file`, `doi`, or `url`). This means that for example an url like
"https://www.example.com/?1234%56" stays this way and is not changed to 
"https://www.example.com/?1234\\%56".


## Installation

This is a small rust utility. You have to install 
[rust](https://www.rust-lang.org/) and execute the following commands:

```
git clone https://codeberg.org/mutluyuz/csv2bibtex
cd csv2bibtex
cargo install --path .
```


## Bugs and Contributing

Of course there are not any bugs in this software. However, you may encounter 
unexpected behavior. Please report them at 
https://codeberg.org/mutluyuz/csv2bibtex/issues.
Please use also the Codeberg page for pull requests.

## Speed

Some very rough benchmarking on my (rather old) laptop yielded about 500.000
processed lines per second. However, this depends on the number of fields and
thus, runtime might be a bit slower or faster. You can run your own benchmarks
with `cargo bench`.
