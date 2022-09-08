use mindr;

#[test]
fn it_creates_config() {
    let config = mindr::get_configuration();
    let display_todays = config.getbool("general", "display_todays").unwrap();

    assert_eq!(display_todays.unwrap(), true);
}
