use scraper::{ElementRef, Html};

use crate::{heading::*, sectioning_type::SectioningType};

const SECTIONING_ROOTS: [&str; 7] =
    ["blockquote", "body", "details", "dialog", "fieldset", "figure", "td"];

/// A compact intermediate outline tree built from the parsed HTML tree.
#[derive(Debug, Clone)]
pub struct OutlineStructure {
    /// The kind of HTML structure that created this node.
    pub sectioning_type:        SectioningType,
    /// The heading that names this node when one was found.
    pub heading:                Option<Heading>,
    /// The nested outline structures found inside this node.
    pub sub_outline_structures: Vec<OutlineStructure>,
}

impl OutlineStructure {
    #[inline]
    pub(crate) fn new(sectioning_type: SectioningType) -> OutlineStructure {
        OutlineStructure {
            sectioning_type,
            heading: None,
            sub_outline_structures: Vec::new(),
        }
    }

    #[inline]
    /// Parses HTML and builds the intermediate outline tree.
    pub fn parse_html<S: AsRef<str>>(html: S, max_depth: usize) -> OutlineStructure {
        let document = Html::parse_document(html.as_ref());
        let root_element = document.root_element();

        if let Some(outline_structure) =
            create_outline_structure_finding_body(root_element, 0, max_depth)
        {
            outline_structure
        } else {
            create_outline_structure(SectioningType::Root, root_element, 0, max_depth)
                .unwrap_or_else(|| OutlineStructure::new(SectioningType::Root))
        }
    }
}

pub(crate) fn create_outline_structure(
    sectioning_type: SectioningType,
    element: ElementRef<'_>,
    depth: usize,
    max_depth: usize,
) -> Option<OutlineStructure> {
    if depth > max_depth {
        return None;
    }

    let mut outline_structure = OutlineStructure::new(sectioning_type);

    // The first heading names the current section, and later headings become child outline items.
    let mut find_heading = true;

    for child in element.child_elements() {
        let local_name = child.value().name();

        // Inner sectioning roots keep their headings out of the current outline.
        if SECTIONING_ROOTS.binary_search(&local_name).is_ok() {
            continue;
        }

        if let Some(sub_sectioning_type) = SectioningType::from_sectioning_content_tag(local_name) {
            if let Some(sub_outline_structure) =
                create_outline_structure(sub_sectioning_type, child, depth + 1, max_depth)
            {
                outline_structure.sub_outline_structures.push(sub_outline_structure);
            }
        } else if let Some(heading) = create_heading(child, depth + 1, max_depth) {
            if find_heading {
                outline_structure.heading = Some(heading);
            } else {
                let mut sub_outline_structure = OutlineStructure::new(SectioningType::Heading);

                sub_outline_structure.heading = Some(heading);

                outline_structure.sub_outline_structures.push(sub_outline_structure);
            }
        } else {
            if let Some(sub_outline_structure) =
                create_outline_structure(SectioningType::Root, child, depth + 1, max_depth)
            {
                // A normal wrapper can contain the first heading, and that must still close the first-heading phase.
                let has_sub_outline_structures =
                    !sub_outline_structure.sub_outline_structures.is_empty();
                let has_heading = sub_outline_structure.heading.is_some();

                if let Some(heading) = sub_outline_structure.heading {
                    if find_heading {
                        outline_structure.heading = Some(heading);
                    } else {
                        let mut sub_outline_structure =
                            OutlineStructure::new(SectioningType::Heading);

                        sub_outline_structure.heading = Some(heading);

                        outline_structure.sub_outline_structures.push(sub_outline_structure);
                    }
                }

                for os in sub_outline_structure.sub_outline_structures {
                    outline_structure.sub_outline_structures.push(os);
                }

                if has_heading || has_sub_outline_structures {
                    find_heading = false;
                }
            }

            continue;
        }

        find_heading = false;
    }

    Some(outline_structure)
}

pub(crate) fn create_outline_structure_finding_body(
    element: ElementRef<'_>,
    depth: usize,
    max_depth: usize,
) -> Option<OutlineStructure> {
    if depth > max_depth {
        return None;
    }

    if element.value().name() == "body" {
        return Some(
            create_outline_structure(SectioningType::Body, element, depth, max_depth)
                .unwrap_or_else(|| OutlineStructure::new(SectioningType::Root)),
        );
    }

    for child in element.child_elements() {
        if let Some(outline_structure) =
            create_outline_structure_finding_body(child, depth + 1, max_depth)
        {
            return Some(outline_structure);
        }
    }

    None
}
