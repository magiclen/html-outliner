use scraper::{ElementRef, Node};

/// A heading found in the parsed HTML tree.
#[derive(Debug, Clone)]
pub struct Heading(HeadingKind);

/// The internal representation of a heading, kept private so a `Heading` cannot be constructed outside this crate.
#[derive(Debug, Clone)]
enum HeadingKind {
    /// A normal `h1` through `h6` heading.
    Header { level: u8, text: String },
    /// A compatibility group created from heading children inside `hgroup`.
    Group(Vec<Heading>),
}

impl Heading {
    /// Returns the heading level at the end of this heading, following the last child heading when this is a group.
    #[inline]
    pub fn get_end_level(&self) -> u8 {
        match &self.0 {
            HeadingKind::Header {
                level, ..
            } => *level,
            HeadingKind::Group(headings) => headings[headings.len() - 1].get_end_level(),
        }
    }

    /// Returns the heading level at the start of this heading, following the first child heading when this is a group.
    #[inline]
    pub fn get_start_level(&self) -> u8 {
        match &self.0 {
            HeadingKind::Header {
                level, ..
            } => *level,
            HeadingKind::Group(headings) => headings[0].get_start_level(),
        }
    }
}

pub(crate) fn create_heading(
    element: ElementRef<'_>,
    depth: usize,
    max_depth: usize,
) -> Option<Heading> {
    if depth > max_depth {
        return None;
    }

    let local_name = element.value().name();
    let local_name_length = local_name.len();

    let mut kind = match local_name_length {
        2 => {
            let stripped_local_name = local_name.strip_prefix('h')?;

            match stripped_local_name.parse::<u8>() {
                Ok(level) if (1..=6).contains(&level) => HeadingKind::Header {
                    level,
                    text: String::new(),
                },
                _ => return None,
            }
        },
        6 if local_name.eq("hgroup") => HeadingKind::Group(Vec::with_capacity(2)),
        _ => return None,
    };

    match &mut kind {
        HeadingKind::Header {
            text, ..
        } => {
            *text = create_text(element, depth, max_depth);
        },
        HeadingKind::Group(headings) => {
            // This keeps the crate's older hgroup behavior by joining heading children instead of choosing only one heading.
            for child in element.child_elements() {
                if let Some(heading) = create_heading(child, depth + 1, max_depth) {
                    headings.push(heading);
                }
            }

            if headings.is_empty() {
                return None;
            }
        },
    }

    Some(Heading(kind))
}

#[inline]
pub(crate) fn create_text(element: ElementRef<'_>, depth: usize, max_depth: usize) -> String {
    if depth > max_depth {
        return String::new();
    }

    let mut raw_text = String::new();
    let mut stack = Vec::new();

    // The explicit stack keeps text extraction iterative and preserves the same order as a recursive depth-first walk.
    for child in element.children().rev() {
        stack.push((depth + 1, child));
    }

    while let Some((node_depth, node)) = stack.pop() {
        if node_depth > max_depth {
            continue;
        }

        match node.value() {
            Node::Text(text) => raw_text.push_str(text),
            Node::Document | Node::Fragment | Node::Element(_) => {
                for child in node.children().rev() {
                    stack.push((node_depth + 1, child));
                }
            },
            _ => {},
        }
    }

    collapse_whitespace(&raw_text)
}

impl From<Heading> for String {
    #[inline]
    fn from(heading: Heading) -> String {
        match heading.0 {
            HeadingKind::Header {
                text, ..
            } => text,
            HeadingKind::Group(headings) => {
                let mut iter = headings.into_iter();

                let mut text: String = iter.next().unwrap().into();

                for heading in iter {
                    text.push_str(" — ");

                    let t: String = heading.into();
                    text.push_str(&t);
                }

                text
            },
        }
    }
}

fn collapse_whitespace(raw_text: &str) -> String {
    let mut text = String::new();

    for word in raw_text.split_whitespace() {
        if !text.is_empty() {
            text.push(' ');
        }

        text.push_str(word);
    }

    text
}
