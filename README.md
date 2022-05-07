<div align="center">
  <h1>elucidate</h1>

[![Crates.io](https://img.shields.io/crates/v/elucidate.svg)](https://crates.io/crates/elucidate)
[![Docs.rs](https://docs.rs/elucidate/badge.svg)](https://docs.rs/elucidate)
[![CI](https://github.com/dark-fusion/elucidate/workflows/CI/badge.svg)](https://github.com/dark-fusion/elucidate/actions)
[![Coverage Status](https://coveralls.io/repos/github/dark-fusion/elucidate/badge.svg?branch=main)](https://coveralls.io/github/dark-fusion/elucidate?branch=main)

</div>

`elucidate` is a powerful JSON parser built to fast, efficient and correct.

The official documentation can be found in the [crate's documentation][docs-rs].

## Project Description

Since the project is currently in very early development, some things are naturally in flux. The
goals of the project at this time are aimed around allowing end-users to:

- Create custom JSON data pipelines
- Validate and transform JSON data
- Securely handle untrusted data without accidentally executing malicious code

### JSON Syntax

The specification that defines valid JSON syntax and the format itself can be found
within [IETF RFC 8259][rfc-8259].

## Feature Roadmap

The following list of features is basic and represents what is required for a minimally-viable
product (MVP). The scope of the project is intentionally as small as possible.

- [ ] Parse JSON data types as defined in [RFC 8259][rfc-8259].
    - [ ] array
    - [X] boolean
    - [X] null
    - [ ] number
    - [ ] object
    - [ ] string
- [ ] Handle structured data such as `Record` entries (key-value pairs)
- [ ] Convert, transform or normalize strings containing escape codes
- [ ] Create **Reader** and **Writer** APIs
    - [ ] Implement async-compatible extension traits

## Design Choices

### Language

[Rust](https://rust-lang.org/) was chosen for its high-performance at runtime, resource
efficiency and memory-safety guarantees. It also provides very useful language constructs such as
pattern matching and a strong type system that includes algebraic data types.

### Parsing Approach

`elucidate` uses a [recursive descent parsing][recursive-descent-wiki] strategy. This is implemented
via [parser combinators][parser-combinator-wiki]. Parser combinators are, in essence, higher-order
functions that are chained together in various ways, producing increasingly complex (and useful!)
parsing functions.

Parser combinators offer a unique degree of flexibility that is particularly suited to parsing JSON.
They provide solutions for handling
minute details and oddities and operate at similar speeds of handwritten
parsers.

Writing a JSON parser is notoriously difficult _to get right_ due to
the [ambiguities present in the official specification][parsing-json-abiguities]. Unfortunately, the
original [RFC][rfc-8259] is quite vague. The impact this has had on software designed to
parse the format is significant.

### External Dependencies

This project depends on a few well-maintained crates:

- [nom](https://github.com/Geal/nom)

## License

Licensed under the [MIT License](/LICENSE):

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in `elucidate` by you, shall be licensed under the MIT License, without any additional
terms or conditions.

See [CONTRIBUTING.md](/CONTRIBUTING.md).

## References and Special Thanks

We would like to give a special thanks to the following people and organizations:

[Nicolas Seriot](https://seriot.ch/) for his terrific work outlining the hidden complexity of
parsing JSON.

<!-- External links -->

[docs-rs]: https://docs.rs/elucidate/latest/elucidate

[rfc-8259]: https://datatracker.ietf.org/doc/html/rfc8259

[parser-combinator-wiki]: https://en.wikipedia.org/wiki/Parser_combinator

[recursive-descent-wiki]: https://en.wikipedia.org/wiki/Recursive_descent_parser

[parsing-json-minefield]: https://seriot.ch/projects/parsing_json.html

[parsing-json-ambiguities]: https://seriot.ch/projects/parsing_json.html#26
