use std::io;
use std::io::Read;
use std::fs::ReadDir;
use std::path::Path;
use std::path::PathBuf;
use std::fs::{DirEntry, OpenOptions, create_dir_all};
use std::fs::File;
use std::collections::HashMap;
use std::io::Write;
use std::io::BufWriter;


const PINE_EXTENSION : &str = "pine";
const TARGET_DIRNAME : &str = "target";

fn get_pinescript_files(dir: ReadDir) -> Vec<DirEntry> {
    dir.map(|result|
        result
            .unwrap())
            .filter(|entry| entry.file_type().unwrap().is_file())
            .filter(|file| match file.path().extension() {
                Some(ext) => ext==PINE_EXTENSION,
                _ => false,
            })
            .collect()
}

fn make_path_map(files: Vec<DirEntry>) -> HashMap<String, PathBuf>{
    files.iter()
        .map(|entry| entry.path())
        .fold(HashMap::new(),
              |mut acc, path| {
                  let file_name = format!("{}",
                                          path.file_name().unwrap()
                                              .to_str().unwrap()
                                              .trim_end_matches(PINE_EXTENSION)
                                              .trim_end_matches("."));
                  acc.insert(file_name, path);
                  acc
              })
}

pub fn run() -> io::Result<()> {
    let mut path = Path::new("..").canonicalize()?;
    let project_dir = path.read_dir()?;
    let pine_files: Vec<DirEntry> = get_pinescript_files(project_dir);

    path.push("function");
    let function_dir = path.read_dir()?;
    let function_files: Vec<DirEntry> = get_pinescript_files(function_dir);

    let function_map = make_path_map(function_files);

    for pine_file_entry in pine_files {
        let path = PathBuf::from(pine_file_entry.path());
        let mut pine_file = File::open(pine_file_entry.path())?;
        let pine_file_name = path.components().last().unwrap();

        let mut target_path = PathBuf::from(pine_file_entry.path());
        target_path.pop();
        target_path.push(TARGET_DIRNAME);
        create_dir_all(&target_path)?;
        target_path.push(pine_file_name);

        let mut target_file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(target_path)?;

        let mut contents = String::new();
        pine_file.read_to_string(&mut contents)?;
        let mut writer = BufWriter::new(&mut target_file);

        for line in contents.lines() {
            if line.starts_with("import ") {
                let words: Vec<&str> = line.split(' ').collect();
                if words.len() != 2 {
                    panic!(format!("Import line malformed: {}", line));
                }
                let function = words[1];
                let function_file = match function_map.get(function) {
                    Some(file) => file,
                    None =>  panic!("Import of non-existent function: {}", function)
                };
                let mut function_file = File::open(function_file)?;
                let mut function_contents = String::new();
                function_file.read_to_string(&mut function_contents)?;
                for function_line in function_contents.lines() {
                    writeln!(writer, "{}", function_line)?;
                }
                ()
            }
            else {
                writeln!(writer, "{}", line)?;
                ()
            }
        }
    }
    Ok(())
}