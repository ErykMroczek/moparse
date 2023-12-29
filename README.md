# *moparse* - Modelica language parser

*moparse* is a parser for the [Modelica](https://modelica.org/) language.

I have created it to use it as a crate/library in my other tools like
[mofmt](https://github.com/ErykMroczek/mofmt), but in the future I want
to provide an executable form that would print the parser output in the
JSON format, so it could be consumable by anything.

The parser is working more or less correctly, but it is very much work
in progress, so every version may bring breaking changes.

## Installation

*moparse* is a Rust crate and can be installed with Cargo:

```bash
cargo install moparse
```

## License

MPL-2.0

## Authors

Eryk Mroczek: <mroczek.eryk@gmail.com>
