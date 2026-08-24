use std::{
    fs::{self, File},
    io::{Read, Write},
};

use lightningcss::{
    printer::PrinterOptions,
    stylesheet::{MinifyOptions, ParserOptions, StyleSheet},
};

fn main() {
    println!("cargo::rerun-if-changed=./src/style");

    let mut buf = String::new();
    fs::read_dir("./src/style/")
        .expect("should be able to read stylesheets directory")
        .map(|dir_entry_res| dir_entry_res.unwrap())
        .map(|dir_entry| File::open(dir_entry.path()).unwrap())
        .for_each(|mut file| {
            file.read_to_string(&mut buf).unwrap();
        });

    let mut stylesheet = StyleSheet::parse(&buf, ParserOptions::default()).unwrap();
    stylesheet.minify(MinifyOptions::default()).unwrap();
    let to_css = stylesheet
        .to_css(PrinterOptions {
            minify: true,
            ..PrinterOptions::default()
        })
        .unwrap();

    File::create("./static/style.css")
        .unwrap()
        .write(to_css.code.as_bytes())
        .unwrap();
}
