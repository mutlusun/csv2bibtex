# csv2bibtex

A small command line utility to parse CSV files to Bib(La)TeX files. This 
utility is not optimised for speed but should be fast enough to parse even 
larger CSV files. Please fill a bug report if it is too slow for you.


## Usage

```
csv2bibtex 0.1.0
mutluyuz


USAGE:
    csv2bibtex [FLAGS] [OPTIONS] <INPUT> <OUTPUT>

FLAGS:
        --biblatex       Print output in BibLaTeX mode (do not use with --bibtex)
        --bibtex         Print output in BibTeX mode (do not use with --biblatex)
    -h, --help           Prints help information
    -l, --lazy           Try to recover from as much errors as possible.
        --no-defaults    Don't add default field mappings.
    -V, --version        Prints version information

OPTIONS:
    -d, --delimiter <DELIMITER>       Delimiter between cells in CSV file
    -f, --field-mapping <FIELD>...    Assignment of csv fields to bibtex fields
    -v, --verbosity <LEVEL>           Verbosity level, either DEBUG, INFO, WARN, or ERROR

ARGS:
    <INPUT>     Input file to use
    <OUTPUT>    Output file to use
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

CSV fields can even be combined:

```
csv2bibtex \
    --field-mapping "pages=[[StartPage]]--[[EndPage]]" \
    input.csv \
    output.bib
```

The field mapping argument can be given multiple times to map multiple fields.


## Installation

This is a small rust utility. You have to install 
[rust](https://www.rust-lang.org/) and execute the following commands:

```
git clone https://codeberg.org/mutluyuz/csv2bibtex
cd csv2bibtex
cargo install --path .
```


## Bugs

Of course there are not any bugs in this software. However, you may encounter 
unexpected behavior. Please report them at 
https://codeberg.org/mutluyuz/csv2bibtex/issues.
