HTML Outliner
====================

[![CI](https://github.com/magiclen/html-outliner/actions/workflows/ci.yml/badge.svg)](https://github.com/magiclen/html-outliner/actions/workflows/ci.yml)

Outline HTML documents for better SEO.

## Outline Algorithm

This crate builds a practical SEO and table-of-contents outline.
It uses the parsed HTML tree, but it only keeps the parts that are useful for a document outline.
It starts from the document `body` when a `body` element is available.
If no `body` element is found, it falls back to the parsed root element.

The algorithm treats `article`, `aside`, `nav`, and `section` as sectioning content.
Each sectioning content element becomes a nested outline item.
If a sectioning content element has no heading, the output uses `Untitled article`, `Untitled aside`, `Untitled nav`, or `Untitled section`.
The algorithm treats `blockquote`, `body`, `details`, `dialog`, `fieldset`, `figure`, and `td` as sectioning roots inside the current walk.
Inner sectioning roots are skipped so their headings do not become part of the current outline.

The algorithm treats `h1` through `h6` as headings.
The first heading found in a section names that section.
Later headings in the same section start new implied outline items.
Heading levels decide nesting for these implied outline items.
A lower-level heading such as `h3` becomes a child of the nearest previous heading with a smaller level number such as `h2`.
A heading with the same or a higher level closes earlier implied heading groups and becomes a sibling.

The algorithm looks through normal wrapper elements such as `header` and `div`.
A heading inside a normal wrapper can still name the current section.
After a wrapper contributes a heading or a nested outline, later sibling headings are treated as later headings instead of replacing the section heading.

Heading text is collected from descendant text nodes.
Whitespace is collapsed so inline markup such as `<h1>Design <em>notes</em></h1>` becomes `Design notes`.
For compatibility with earlier versions of this crate, `hgroup` joins its heading children with ` — `.

The `max_depth` argument limits how deeply the parser walk can read.
Nodes deeper than `max_depth` are ignored by the outline builder.

## Difference From A Browser Accessibility Tree

A browser accessibility tree is built for assistive technologies and includes roles, names, states, focus behavior, and visibility rules.
This crate does not build that kind of tree.
It only reads the HTML elements and text needed for a simple document outline.
Use this crate for content structure checks, not as a replacement for accessibility testing.

## Difference From The Old HTML5 Outline Algorithm

The old HTML5 outline algorithm tried to create a full section model for the whole document.
This crate is smaller and more predictable.
It returns a displayable outline from sectioning content and heading levels.
It does not expose a current outlinee model or associate every DOM node with a generated section.

## Using This Crate For SEO

This crate can help SEO work by showing whether a page has a clear heading structure.
Use the generated outline to check whether the main topic appears early and whether the subtopics are easy to scan.
Look for missing headings, untitled sections, confusing heading levels, duplicated heading text, or headings that do not match the visible content.
You can also use the outline to generate a table of contents or to validate pages in CI.
This does not guarantee better rankings, but it helps search engines and users understand the page more easily.

## Examples

```rust
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
