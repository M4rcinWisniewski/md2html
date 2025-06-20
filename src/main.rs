use clap::{arg, Command};
use std::path::{Path, PathBuf};
mod convert_files;


fn main() {
    let matches = Command::new("Parser")
        .version("1.0")
        .about("Converts markdown files to HTML")
        .arg(arg!(--input <INPUT_FILE>).required(true))
        .get_matches();

        let _dir = Path::new("./include");
        let file_name = matches.get_one::<String>("input").expect("required");

        let full_path: PathBuf = _dir.join(file_name);

        convert_files::read( full_path.to_str().unwrap());
}
