pub struct Print;

impl Print {
    pub fn item(string: &str, spacing: &str) {
        print!("{item}{spacing}", item = string, spacing = spacing);
    }

    pub fn prepare() {
        print!("{}{}", termion::clear::All, termion::cursor::Goto(1, 1));
    }

    pub fn finsih() {
        println!("");
    }
}

pub struct Cursor;

impl Cursor {
    pub fn hide() {
        print!("{}", termion::cursor::Hide);
    }

    pub fn show() {
        print!("{}", termion::cursor::Show);
    }

    pub fn place(x: u16, y: u16) {
        println!("{}", termion::cursor::Goto(x, y));
    }

    pub fn reset() {
        print!("{}", termion::cursor::Goto(1, 1));
    }
}

pub struct Screen;

impl Screen {
    pub fn clear() {
        print!("{}", termion::clear::All);
    }

    pub fn get_size() -> (u16, u16) {
        termion::terminal_size().expect("Couldn't get the terminal window size")
    }
}
