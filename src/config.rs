use std::{
    env,
    fs::{self, File},
    io::Write,
    path::Path,
};

pub fn config() {
    let config_path = format!(
        "/home/{}/.config/",
        env::var("LOGNAME").unwrap_or(".config".to_owned())
    );

    let file_path = format!(
        "/home/{}/.config/.xonfig",
        env::var("LOGNAME").unwrap_or(".config".to_owned())
    );

    if !Path::new(&file_path).exists() {
        fs::create_dir_all(config_path);
        let mut file = File::create(file_path).unwrap();
        file.write_all(b"color=blue\n");
    }
}
