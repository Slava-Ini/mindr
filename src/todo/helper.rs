pub fn print_item(string: &str, spacing: &str) {
    print!("{item}{spacing}", item = string, spacing = spacing);
}

pub fn prepare_print() {
    print!("{}{}", termion::clear::All, termion::cursor::Goto(1, 1));
}

pub fn finish_print() {
    println!("");
}

pub fn hide_cursor() {
    print!("{}", termion::cursor::Hide);
}
pub fn show_cursor() {
    print!("{}", termion::cursor::Show);
}

pub fn move_cursor(x: u16, y: u16) {
    println!("{}", termion::cursor::Goto(x, y));
}
