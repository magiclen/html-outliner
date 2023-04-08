use kuchiki::{parse_html, traits::TendrilSink, NodeData, NodeRef};

use crate::{heading::*, sectioning_type::SectioningType};

const SECTIONING_ROOTS: [&str; 7] =
    ["blockquote", "body", "details", "dialog", "fieldset", "figure", "td"];

#[derive(Debug, Clone)]
pub struct OutlineStructure {
    pub sectioning_type:        SectioningType,
    pub heading:                Option<Heading>,
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
    pub fn parse_html<S: AsRef<str>>(html: S, max_depth: usize) -> OutlineStructure {
        let node = parse_html().one(html.as_ref());

        if let Some(outline_structure) =
            create_outline_structure_finding_body(node.clone(), 0, max_depth)
        {
            outline_structure
        } else {
            create_outline_structure(SectioningType::Root, node, 0, max_depth)
                .unwrap_or_else(|| OutlineStructure::new(SectioningType::Root))
        }
    }
}

pub(crate) fn create_outline_structure(
    sectioning_type: SectioningType,
    node: NodeRef,
    depth: usize,
    max_depth: usize,
) -> Option<OutlineStructure> {
    if depth > max_depth {
        return None;
    }

    let mut outline_structure = OutlineStructure::new(sectioning_type);

    let mut find_heading = true;

    for child in node.children() {
        if let NodeData::Element(element_data) = child.data() {
            let local_name: &str = &element_data.name.local;

            if SECTIONING_ROOTS.binary_search(&local_name).is_ok() {
                continue;
            }

            if let Some(sub_sectioning_type) =
                SectioningType::from_sectioning_content_tag(local_name)
            {
                if let Some(sub_outline_structure) =
                    create_outline_structure(sub_sectioning_type, child, depth + 1, max_depth)
                {
                    outline_structure.sub_outline_structures.push(sub_outline_structure);
                }
            } else if let Some(heading) = create_heading(child.clone(), depth + 1, max_depth) {
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
                }

                continue;
            }

            find_heading = false;
        }
    }

    Some(outline_structure)
}

pub(crate) fn create_outline_structure_finding_body(
    node: NodeRef,
    depth: usize,
    max_depth: usize,
) -> Option<OutlineStructure> {
    if depth > max_depth {
        return None;
    }

    if let NodeData::Element(element_data) = node.data() {
        let local_name: &str = &element_data.name.local;

        if local_name == "body" {
            return Some(
                create_outline_structure(SectioningType::Body, node.clone(), depth, max_depth)
                    .unwrap_or_else(|| OutlineStructure::new(SectioningType::Root)),
            );
        }
    }

    for child in node.children() {
        if let Some(outline_structure) =
            create_outline_structure_finding_body(child, depth + 1, max_depth)
        {
            return Some(outline_structure);
        }
    }

    None
}
