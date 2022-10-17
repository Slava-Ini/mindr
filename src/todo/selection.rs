use core::str::FromStr;

use super::helper::print_item;

const WRAPPER: &'static str = " ";
const DEFAULT_SPACING: &'static str = " ";

pub struct PrintStyle<'a> {
    pub selection: Option<&'a Selection>,
    pub strikethrough: bool,
    pub spacing: Option<&'a str>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Selection {
    Brackets,
    Tilde,
    Outline,
    Bold,
}

impl Selection {
    pub fn as_str(&self) -> &'static str {
        match self {
            Selection::Brackets => "brackets",
            Selection::Tilde => "tilde",
            Selection::Outline => "outline",
            Selection::Bold => "bold",
        }
    }

    pub fn get_selected_str(string: &str, style: &Self) -> String {
        let selection = match style {
            Selection::Tilde => ("~", WRAPPER),
            Selection::Brackets => ("[", "]"),
            Selection::Outline | Selection::Bold => (WRAPPER, WRAPPER),
        };

        let (start_char, end_char) = selection;

        format!(
            "{first_char}{rest}{last_char}",
            first_char = start_char,
            rest = &string[1..string.len() - 1],
            last_char = end_char
        )
        // let result = string.replacen(WRAPPER, start_char, 1);
        // result.replace(WRAPPER, end_char)
    }

    // TODO: delete the following after refactor
    pub fn print_outline(string: &str, spacing: Option<&str>) {
        print!(
            "{bg}{fg}{item}{bg_clear}{fg_clear}{spacing}",
            bg = termion::color::Bg(termion::color::White),
            fg = termion::color::Fg(termion::color::Black),
            item = string,
            bg_clear = termion::color::Bg(termion::color::Reset),
            fg_clear = termion::color::Fg(termion::color::Reset),
            spacing = spacing.unwrap_or(DEFAULT_SPACING)
        );
    }

    pub fn print_bold(string: &str, spacing: Option<&str>) {
        print!(
            "{bold}{item}{reset}{spacing}",
            bold = termion::style::Bold,
            item = string,
            reset = termion::style::Reset,
            spacing = spacing.unwrap_or(DEFAULT_SPACING)
        );
    }

    pub fn print_strikethrough(string: &str, spacing: Option<&str>) {
        print!(
            "{strikethrough}{item}{reset}{spacing}",
            strikethrough = termion::style::CrossedOut,
            item = string,
            reset = termion::style::Reset,
            spacing = spacing.unwrap_or(DEFAULT_SPACING)
        );
    }

    pub fn print_styled(string: &str, style: PrintStyle) {
        let PrintStyle {
            selection,
            strikethrough,
            spacing,
        } = style;

        let text = if strikethrough {
            let last_index = string.len() - 1;
            // TODO: can be improved in the future for non-ASCII chars
            format!(
                "{first_char}{strikethrough}{rest_chars}{reset}{last_char}",
                first_char = &string[..1],
                strikethrough = termion::style::CrossedOut,
                rest_chars = &string[1..last_index],
                reset = termion::style::Reset,
                last_char = &string[last_index..],
            )
        } else {
            string.to_owned()
        };

        // TODO: first re-write menu and then fix the bugs
        let text = Selection::get_selected_str(&text, selection.unwrap_or(&Selection::Brackets));

        match selection {
            Some(Selection::Bold) => {
                print!(
                    "{bold}{item}{reset}{spacing}",
                    bold = termion::style::Bold,
                    item = text,
                    reset = termion::style::Reset,
                    spacing = spacing.unwrap_or(DEFAULT_SPACING)
                );
            }
            Some(Selection::Outline) => {
                // TODO: all match arms' text can be put above
                print!("TEXT: |{}|", text);
                print!(
                    "{bg}{fg}{item}{bg_clear}{fg_clear}{spacing}",
                    bg = termion::color::Bg(termion::color::White),
                    fg = termion::color::Fg(termion::color::Black),
                    item = text,
                    bg_clear = termion::color::Bg(termion::color::Reset),
                    fg_clear = termion::color::Fg(termion::color::Reset),
                    spacing = spacing.unwrap_or(DEFAULT_SPACING)
                );
            }
            Some(Selection::Brackets) => {
                print_item(&text, spacing.unwrap_or(DEFAULT_SPACING));
            }
            Some(Selection::Tilde) => {
                print_item(&text, spacing.unwrap_or(DEFAULT_SPACING));
            }
            None => {
                print_item(&text, spacing.unwrap_or(DEFAULT_SPACING));
            }
        }
    }
}

impl FromStr for Selection {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "brackets" => Ok(Selection::Brackets),
            "tilde" => Ok(Selection::Tilde),
            "outline" => Ok(Selection::Outline),
            "bold" => Ok(Selection::Bold),
            _ => {
                return Err(
                    "No such selection style available, try using 'brackets/tilde/outline/bold'",
                )
            }
        }
    }
}
