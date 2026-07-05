/// The kind of HTML structure represented by an outline structure node.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum SectioningType {
    // ----- Sectioning Content -----
    /// An `article` element.
    Article,
    /// An `aside` element.
    Aside,
    /// A `nav` element.
    Nav,
    /// A `section` element.
    Section,

    // ----- Element -----
    /// A generic root used while flattening normal wrapper elements.
    Root,
    /// A `body` element.
    Body,
    /// An implied outline item created from a heading after the first heading in a section.
    Heading,
}

impl SectioningType {
    #[inline]
    pub(crate) fn is_sectioning_content_type(&self) -> bool {
        matches!(
            self,
            SectioningType::Article
                | SectioningType::Aside
                | SectioningType::Nav
                | SectioningType::Section
        )
    }

    #[inline]
    pub(crate) fn is_heading(&self) -> bool {
        matches!(self, SectioningType::Heading)
    }

    #[inline]
    pub fn from_sectioning_content_tag<S: AsRef<str>>(s: S) -> Option<SectioningType> {
        let s = s.as_ref();

        match s {
            "article" => Some(SectioningType::Article),
            "aside" => Some(SectioningType::Aside),
            "nav" => Some(SectioningType::Nav),
            "section" => Some(SectioningType::Section),
            _ => None,
        }
    }

    #[inline]
    pub fn as_str(&self) -> &'static str {
        match self {
            SectioningType::Article => "article",
            SectioningType::Aside => "aside",
            SectioningType::Nav => "nav",
            SectioningType::Section => "section",
            SectioningType::Root => "root",
            SectioningType::Body => "body",
            SectioningType::Heading => "heading",
        }
    }
}
