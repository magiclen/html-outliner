#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum SectioningType {
    // ----- Sectioning Content -----
    Article,
    Aside,
    Nav,
    Section,

    // ----- Element -----
    Root,
    Body,
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
