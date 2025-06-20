use std::fs;
use pulldown_cmark::{Parser, Options};

pub fn read(file_path: &str) {
    let from_file: String = fs::read_to_string(file_path).expect("❌ Could not read the file");
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);

    let parser = Parser::new_ext(&from_file, options);
    let mut html_output = String::new();
    pulldown_cmark::html::push_html(&mut html_output, parser);

    let _ = fs::write("./include/result.html", html_output);

    println!("
    -------------------------------------------------------\n
    Converted file saved in include directory as result.html!\n
    -------------------------------------------------------
    ")

}