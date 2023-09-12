extern crate umya_spreadsheet;
use umya_spreadsheet::{Spreadsheet, Cell};
use serde::Serialize;
use std::collections::HashMap;
use std::fs;
use std::io::prelude::*;

#[derive(Serialize)]
struct LanguagesList {
    en: HashMap<String, String>,
    ar: HashMap<String, String>
}

#[derive(Serialize)]
struct Item {
    key: String,
    en: String,
    ar: String
}

struct Parser {}

pub fn parse(file_path: String, sheet_name: Option<String>) {
    let file = get_file(&file_path);
    // let cells = get_cells(&file, sheet_name);
    let sheet_name = get_sheet_name(sheet_name, &file);
    println!("Sheet name, {}", sheet_name);
}

fn get_sheet_name(name: Option<String>, file: &Spreadsheet) -> String {
    let sheet_collection = file.get_sheet_collection();
    let worksheet = match name {
        Some(name) => name,
        None => sheet_collection.get(1).unwrap().get_name().to_string()
    };
    return worksheet
}

fn get_file(path: &String) -> Spreadsheet {
    let path = std::path::Path::new(path);
    umya_spreadsheet::reader::xlsx::read(path).expect("No exel file")
}

fn get_cells(file: &Spreadsheet, sheet_name: Option<String>) -> Vec<&Cell> {
    let worksheet = match sheet_name {
        Some(name) => file.get_sheet_by_name(&name).unwrap(),
        None => file.get_sheet(&1).unwrap()
    };
    worksheet.get_collection_by_column(&1)
}
