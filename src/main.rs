// use configparser::ini::Ini;
// use std::{
//     env,
//     fs::{self, File},
//     path::Path,
// };

// fn get_config_path() -> String {
//     let user_name = env::var("USERNAME")
//         .expect("Couldn't get system user")
//         .to_owned();

//     format!("/home/{user_name}/.config/mindr/mindr.conf")
// }

// fn get_config(path: &Path) -> Ini {
//     let mut config = Ini::new();

//     config.load(path).expect("Couldn't parse configuration file");
//     config
// }
use mindr;

fn main() {
    let config = mindr::get_configuration();
    println!("{:?}", config);
    // let config_path = get_config_path();
    // let path = Path::new(&config_path);

    // if !path.exists() {
    //     let prefix = path.parent().expect("Couldn't get the path prefix");

    //     fs::create_dir_all(prefix).expect("Couldn't create a directory");
    //     File::create(path).expect("Couldn't create configuration file");
    // }

    // let config = get_config(path);
    // println!("{:#?}", config);
}