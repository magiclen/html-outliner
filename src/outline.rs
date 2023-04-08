use std::fmt::{self, Display, Formatter, Write};

use crate::OutlineStructure;

const EXTRA_INDENT_WIDTH: usize = 1;

#[derive(Debug, Clone, Default)]
pub struct Outline {
    pub text:         Option<String>,
    pub sub_outlines: Vec<Outline>,
}

impl Outline {
    #[inline]
    pub fn parse_html<S: AsRef<str>>(html: S, max_depth: usize) -> Outline {
        OutlineStructure::parse_html(html, max_depth).into()
    }
}

impl From<OutlineStructure> for Outline {
    #[inline]
    fn from(os: OutlineStructure) -> Self {
        if os.sectioning_type.is_heading() {
            Outline {
                text:         os.heading.map(|heading| heading.into()),
                sub_outlines: Vec::new(),
            }
        } else {
            let mut sub_outlines = Vec::new();

            let mut stack = vec![];
            let mut levels: Vec<u8> = vec![];

            for sub_os in os.sub_outline_structures.into_iter().rev() {
                if sub_os.sectioning_type.is_heading() {
                    let heading = sub_os.heading.unwrap();
                    let heading_level = heading.get_start_level();

                    let mut outline = Outline {
                        text:         Some(heading.into()),
                        sub_outlines: Vec::new(),
                    };

                    while let Some(level) = levels.pop() {
                        if level > heading_level {
                            outline.sub_outlines.push(stack.pop().unwrap());
                        } else {
                            levels.push(level);

                            break;
                        }
                    }

                    levels.push(heading_level);
                    stack.push(outline);
                } else {
                    stack.push(sub_os.into());
                }
            }

            let text = if let Some(heading) = os.heading {
                let heading_level = heading.get_start_level();

                let need_flatten = {
                    let mut b = false;

                    for level in levels.iter().copied().rev() {
                        if level >= heading_level {
                            b = true;
                        }
                    }

                    b
                };

                if need_flatten {
                    let mut outline = Outline {
                        text:         Some(heading.into()),
                        sub_outlines: Vec::new(),
                    };

                    while let Some(level) = levels.pop() {
                        if level > heading_level {
                            outline.sub_outlines.push(stack.pop().unwrap());
                        } else {
                            levels.push(level);

                            break;
                        }
                    }

                    levels.push(heading_level);
                    stack.push(outline);

                    None
                } else {
                    Some(heading.into())
                }
            } else if os.sectioning_type.is_sectioning_content_type() {
                Some(format!("Untitled {}", os.sectioning_type.as_str()))
            } else {
                None
            };

            while let Some(sub_os) = stack.pop() {
                sub_outlines.push(sub_os);
            }

            Outline {
                text,
                sub_outlines,
            }
        }
    }
}

impl Display for Outline {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        format(f, self, 1, 0)
    }
}

fn format(
    f: &mut Formatter<'_>,
    outline: &Outline,
    number: usize,
    indent: usize,
) -> Result<(), fmt::Error> {
    let new_ident = if let Some(text) = outline.text.as_ref() {
        if indent > 0 {
            f.write_char('\n')?;

            for _ in 0..indent {
                f.write_char(' ')?;
            }
        } else if number > 1 {
            f.write_char('\n')?;
        }

        f.write_fmt(format_args!("{}. ", number))?;
        f.write_str(text.as_str())?;

        indent + count_digit(outline.sub_outlines.len()) + 2 + EXTRA_INDENT_WIDTH
    } else {
        indent
    };

    for (i, sub_outline) in outline.sub_outlines.iter().enumerate() {
        format(f, sub_outline, i + 1, new_ident)?;
    }

    Ok(())
}

#[inline]
fn count_digit(n: usize) -> usize {
    (n as f64).log10().floor() as usize + 1
}
