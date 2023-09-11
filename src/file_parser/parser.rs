extern crate umya_spreadsheet;
use umya_spreadsheet::{Spreadsheet, Cell};
use serde::Serialize;
use std::collections::HashMap;
use std::fs;
use std::io::prelude::*;

// 1. Check if file exists
// 2. If sheet name is not provided get first sheet

pub fn parse(file_path: String, sheet_name: Option<String>) {
    let file = get_file(&file_path);
    let sheet = get_cells(&file, sheet_name);
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
