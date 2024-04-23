
use std::fs;
use std::env;

use xml_structs::Data;
pub mod xml_structs;

fn main() {
    let current_dir = env::current_dir().unwrap();
    let path = format!("{}\\dataset.xml", current_dir.into_os_string().into_string().unwrap());

    println!("Read xml file at: {}", path);
    
    let result = read_xml_file(path);

    println!("{:?}", result)
}

fn read_xml_file(path: String) -> Data {
    let xml_data = fs::read_to_string(path).expect("Unable to read file");
    quick_xml::de::from_str::<Data>(&xml_data).unwrap()
}