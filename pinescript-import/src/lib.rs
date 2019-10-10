use std::io;
use std::path::Path;
use std::fs::DirEntry;
use std::collections::HashMap;

const PINE_EXTENSION : &str = "pine";

#[allow(unused_variables)]
pub fn run() -> io::Result<()> {
    let mut path = Path::new("..").canonicalize().unwrap();
    let project_dir = path.read_dir().expect("Reading of project dir failed");
    let pine_files: Vec<DirEntry> = project_dir.map(|result| result.unwrap()).filter(|entry| entry.file_type().unwrap().is_file()).filter(|file| match file.path().extension() {Some(ext) => ext==PINE_EXTENSION, _ => false,}).collect();
    path.push("function");
    let function_dir = path.read_dir().expect("Reading of function dir failed");
    let function_files: Vec<DirEntry> = function_dir.map(|result| result.unwrap()).filter(|entry| entry.file_type().unwrap().is_file()).filter(|file| match file.path().extension() {Some(ext) => ext==PINE_EXTENSION, _ => false,}).collect();
    let functions =  function_files.iter().map(|entry| entry.path()).fold(HashMap::new(), |mut acc,  path| {let file_name = format!("{}", path.file_name().unwrap().to_str().unwrap().trim_end_matches(PINE_EXTENSION).trim_end_matches("."));acc.insert(file_name, path); acc});

    let tsr_function = functions.get("TSR").unwrap();
    println!("TSR function file: {:?}", tsr_function);
    Ok(())
}