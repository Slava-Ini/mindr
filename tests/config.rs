use mindr::config::Config;
use mindr::todo::selection::Selection;
use serial_test::serial;

use std::path::PathBuf;
use std::{env, fs};

fn get_config_path() -> PathBuf {
    // TODO: improve error handling
    let user_name = env::var("USERNAME").expect("Couldn't get system user");

    let path: PathBuf = [
        "/home",
        user_name.as_str(),
        ".config",
        "mindr",
        "test_mindr.conf",
    ]
    .iter()
    .collect();

    path
}

#[test]
#[serial]
fn it_creates_new_config() {
    let path = get_config_path();

    if path.exists() {
        fs::remove_file(&path).unwrap();
    }

    let expected_config = Config {
        path: &path,
        ..Default::default()
    };
    let config = Config::init(&path);

    assert_eq!(expected_config, config);
}

#[test]
#[serial]
fn it_reads_existing_config() {
    let path = get_config_path();

    if path.exists() {
        fs::remove_file(&path).unwrap();
    }

    let mut config = Config::init(&path);

    config.display_todays = false;
    config.remind_unfinished = false;
    config.auto_hide_menu = true;
    config.hide_menu_timeout = 1000;
    config.selection_style = Selection::Tilde;

    config.save();

    let saved_config = Config::init(&path);

    let expected_config = Config {
        path: &path,
        display_todays: false,
        remind_unfinished: false,
        auto_hide_menu: true,
        hide_menu_timeout: 1000,
        selection_style: Selection::Tilde,
        ..Default::default()
    };

    assert_eq!(expected_config, saved_config);
}

// TODO: later cover a test case where user changes some values to be wrong in config and tries to
// run the program, see `read_ini`
