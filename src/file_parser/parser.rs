extern crate umya_spreadsheet;
use umya_spreadsheet::Spreadsheet;
use serde::Serialize;
use std::collections::HashMap;
use std::fs;
use std::io::prelude::*;

#[derive(Serialize, Debug)]
struct LanguagesList {
    en: HashMap<String, String>,
    ar: HashMap<String, String>
}

enum FileType {
    Ar,
    En
}

impl LanguagesList {
    fn write_file(&self, file_type: FileType, translation_name: &str) -> Result<(), std::io::Error> {
        let value = match file_type {
            FileType::Ar => &self.ar,
            FileType::En => &self.en
        };
        let json = serde_json::to_string_pretty(&value).map_err(|err| {
            std::io::Error::new(std::io::ErrorKind::Other, err)
        })?;
        
        let mut file = match file_type {
            FileType::Ar => {
                let ar_str = "./".to_owned();
                let ar_path_str = ar_str + "ar_" + translation_name + ".json";
                fs::File::create(ar_path_str)?
            },
            FileType::En => {
                let en_str = "./".to_owned();
                let en_path_str = en_str + "en_" + translation_name + ".json";
                fs::File::create(en_path_str)?
            }
        };
        println!("Translation was successfully created");
        file.write_all(json.as_bytes())?;
        Ok(())
    }
}

#[derive(Serialize)]
struct Item {
    key: String,
    en: String,
    ar: String
}

impl Item {
    fn new(index: String, file: &Spreadsheet) -> Item {
        let mut key_str = "A".to_owned();
        let mut en_str = "B".to_owned();
        let mut ar_str = "C".to_owned();
        key_str.push_str(&index);
        en_str.push_str(&index);
        ar_str.push_str(&index);
        let key = file.get_sheet_by_name("Workspaces").unwrap().get_value(key_str);
        let en = file.get_sheet_by_name("Workspaces").unwrap().get_value(en_str);
        let ar = file.get_sheet_by_name("Workspaces").unwrap().get_value(ar_str);

        Item {
            key,
            en,
            ar
        }
    }
}

struct Parser {
    file: Spreadsheet,
    sheet_name: String
}

impl Parser {
    fn new(file: Spreadsheet, sheet_name: String) -> Parser {
        Parser {
            file,
            sheet_name
        }
    }
    fn construct_languages_list(&self) -> LanguagesList {
        let mut list = LanguagesList {
            en: HashMap::new(),
            ar: HashMap::new()
        };
        let cell = &self.file.get_sheet_by_name(&self.sheet_name).unwrap().get_collection_by_column(&1);
        for (index, _) in cell.iter().enumerate() {
            if index > 0 {
                let string_index = index.to_string();
                let item = Item::new(string_index, &self.file);
                list.en.insert(item.key.clone(), item.en);
                list.ar.insert(item.key, item.ar);
            }
        }
        list
    }
}

pub fn parse(file_path: String, sheet_name: Option<String>) {
    let path = std::path::Path::new(&file_path);
    let file = umya_spreadsheet::reader::xlsx::read(path).expect("No excel file");
    let sheet_name = get_sheet_name(sheet_name, &file);
    let parser = Parser::new(file, sheet_name);
    let list = Parser::construct_languages_list(&parser);
    list.write_file(FileType::Ar, &parser.sheet_name).unwrap();
    list.write_file(FileType::En, &parser.sheet_name).unwrap();
}

fn check_if_provided_sheet_exists(file: &Spreadsheet, sheet_name: &str) -> Result<(), String> {
    let result = file.get_sheet_by_name(sheet_name);
    match result {
        Ok(_) => Ok(()),
        Err(err) => Err(err.to_owned()),
    }
}

fn get_sheet_name(name: Option<String>, file: &Spreadsheet) -> String {
    let sheet_collection = file.get_sheet_collection();
    let worksheet = match name {
        Some(name) => {
            check_if_provided_sheet_exists(&file, &name).expect("Wrong sheet name provided");
            name
        },
        None => sheet_collection.get(1).unwrap().get_name().to_string()
    };
    return worksheet
}
