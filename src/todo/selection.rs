use core::str::FromStr;

const WRAPPER: &'static str = " ";
const DEFAULT_SPACING: &'static str = " ";

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

    pub fn get_selected_str(string: &str, style: Self) -> String {
        let selection = match style {
            Selection::Tilde => ("~", WRAPPER),
            Selection::Brackets => ("[", "]"),
            Selection::Outline | Selection::Bold => (WRAPPER, WRAPPER),
        };

        let (start_char, end_char) = selection;

        let result = string.replacen(WRAPPER, start_char, 1);
        result.replace(WRAPPER, end_char)
    }

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
