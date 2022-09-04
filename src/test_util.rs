use std::env;

pub fn get_current_dir() -> String {
    if let Ok(path) = env::var("CARGO_MANIFEST_DIR") {
        path
    }
    else {
        String::from(env::current_dir().unwrap().to_str().unwrap())
    }
}
