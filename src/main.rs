use std::env;
use std::path::PathBuf;

use mindr::app::App;
use mindr::config::Config;

struct Path {
    base_path: [String; 4],
}

impl Path {
    fn new() -> Self {
        let user_name = env::var("USERNAME").expect("Couldn't get system user");
        let base_path: [String; 4] = [
            String::from("/home"),
            user_name,
            String::from(".config"),
            String::from("mindr"),
        ];

        Path { base_path }
    }

    fn get(&self) -> [PathBuf; 2] {
        let mut config_path = self.base_path.iter().collect::<PathBuf>();
        let mut app_path = self.base_path.iter().collect::<PathBuf>();

        config_path.push("mindr.conf");
        app_path.push("todo.txt");

        [config_path, app_path]
    }
}

// TODO: think if it's good to add other crate (not mindr) kind of like namespace for config
fn main() {
    let path = Path::new();
    let [config_path, app_path] = path.get();

    let config = Config::init(&config_path);
    let mut app = App::init(&config, &app_path);

    app.run();
}

// -- For Future: --
// TODO: Write log messages upon each action
