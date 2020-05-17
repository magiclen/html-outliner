HTML Outliner
====================

[![Build Status](https://travis-ci.org/magiclen/html-outliner.svg?branch=master)](https://travis-ci.org/magiclen/html-outliner)

Outline HTML documents for better SEO.

## Examples

```rust
extern crate html_outliner;

use html_outliner::Outline;

const MAX_DEPTH: usize = 50;

let outline = Outline::parse_html(r"
<h1>Header Top</h1>
<h1>Header 1</h1>
<h2>Header 2</h2>
<h3>Header 3</h3>
<h4>Header 4</h4>
<h5>Header 5</h5>
<h6>Header 6</h6>
", MAX_DEPTH);

println!("{}", outline);

/*
1. Header Top
2. Header 1
    1. Header 2
        1. Header 3
            1. Header 4
                1. Header 5
                    1. Header 6
*/
```

## Crates.io

https://crates.io/crates/html-outliner

## Documentation

https://docs.rs/html-outliner

## License

[MIT](LICENSE)