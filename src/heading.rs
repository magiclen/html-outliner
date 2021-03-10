extern crate kuchiki;
extern crate regex;

use kuchiki::{NodeData, NodeRef};

use regex::Regex;

lazy_static! {
    static ref RE_NEW_LINE: Regex = Regex::new("\n[\n\t ]*").unwrap();
}

#[derive(Debug, Clone)]
pub enum Heading {
    Header {
        level: u8,
        text: String,
    },
    Group(Vec<Heading>),
}

impl Heading {
    #[inline]
    pub fn get_end_level(&self) -> u8 {
        match self {
            Heading::Header {
                level,
                ..
            } => *level,
            Heading::Group(headings) => headings[headings.len() - 1].get_end_level(),
        }
    }

    #[inline]
    pub fn get_start_level(&self) -> u8 {
        match self {
            Heading::Header {
                level,
                ..
            } => *level,
            Heading::Group(headings) => headings[0].get_end_level(),
        }
    }
}

pub(crate) fn create_heading(node: NodeRef, depth: usize, max_depth: usize) -> Option<Heading> {
    if depth > max_depth {
        return None;
    }

    let mut heading = if let NodeData::Element(element_data) = node.data() {
        let local_name: &str = &element_data.name.local;

        let local_name_length = local_name.len();

        match local_name_length {
            2 => {
                if let Some(stripped_local_name) = local_name.strip_prefix('h') {
                    match stripped_local_name.parse::<u8>() {
                        Ok(level) if (1..=6).contains(&level) => {
                            Heading::Header {
                                level,
                                text: String::new(),
                            }
                        }
                        _ => return None,
                    }
                } else {
                    return None;
                }
            }
            6 => {
                if local_name.eq("hgroup") {
                    Heading::Group(Vec::with_capacity(2))
                } else {
                    return None;
                }
            }
            _ => return None,
        }
    } else {
        return None;
    };

    match &mut heading {
        Heading::Header {
            text,
            ..
        } => {
            for child in node.children() {
                create_text(text, child, depth + 1, max_depth);
            }
        }
        Heading::Group(headings) => {
            for child in node.children() {
                if let Some(heading) = create_heading(child, depth + 1, max_depth) {
                    headings.push(heading);
                }
            }

            if headings.is_empty() {
                return None;
            }
        }
    }

    Some(heading)
}

impl From<Heading> for String {
    #[inline]
    fn from(heading: Heading) -> String {
        match heading {
            Heading::Header {
                text,
                ..
            } => text,
            Heading::Group(headings) => {
                let mut iter = headings.into_iter();

                let mut text: String = iter.next().unwrap().into();

                for heading in iter {
                    text.push_str(" — ");

                    let t: String = heading.into();
                    text.push_str(&t);
                }

                text
            }
        }
    }
}

#[inline]
pub(crate) fn create_text(text: &mut String, node: NodeRef, depth: usize, max_depth: usize) {
    if depth > max_depth {
        return;
    }

    if let NodeData::Text(t) = node.data() {
        let t = t.borrow();

        text.push_str(RE_NEW_LINE.replace(t.as_str(), " ").trim());
    } else {
        for child in node.children() {
            create_text(text, child, depth + 1, max_depth);
        }
    }
}
