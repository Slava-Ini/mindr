pub fn print_item(string: &str, spacing: &str) {
    print!("{item}{spacing}", item = string, spacing = spacing);
}

pub fn prepare_print() {
    print!("{}{}", termion::clear::All, termion::cursor::Goto(1, 1));
}

pub fn finish_print() {
    println!("");
}
