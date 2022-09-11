use mindr;

#[test]
    // TODO: add tests for:
    // 1. Create new config when there is no path
    // 2. Read existing config
fn it_creates_config() {
    let config = mindr::get_configuration();
    let display_todays = config.getbool("general", "display_todays").unwrap();

    assert_eq!(display_todays.unwrap(), true);
}
