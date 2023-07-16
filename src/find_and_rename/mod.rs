use crate::rand_ascii::get_random_ascii_printable_code;
use std::ffi::{OsStr, OsString};
use std::fs;
use std::os::unix::prelude::OsStrExt;

pub fn chop(path: &str, sub: bool, cap: usize, ascii: bool) {
    let current_dir = std::path::PathBuf::from(path);
    let mut subs = vec![];
    if !current_dir.exists() {
        println!("Invalid path specified: {}", path);
        return;
    }
    for entry in fs::read_dir(current_dir).expect("Not a valid directory") {
        let entry = entry.expect("Not valid");
        let path = entry.path();
        if path.is_file() && path.file_name().unwrap().len() > cap {
            let extension = path.extension();
            let chopped_name;
            match extension {
                None => {
                    chopped_name = format!(
                        "{}",
                        calculate_name(path.file_name().expect("No file name!"), cap, ascii)
                            .to_str()
                            .expect("Cannot convert name to a valid utf-8 string")
                    );
                }
                Some(extension) => {
                    chopped_name = format!(
                        "{}.{}",
                        calculate_name(
                            path.file_name().expect("No file name!"),
                            cap - (extension.len() + 1),
                            ascii
                        )
                        .to_str()
                        .expect("Cannot convert name to a valid utf-8 string"),
                        extension
                            .to_str()
                            .expect("Cannot extension name to a valid utf-8 string")
                    );
                }
            };
            println!(
                "Original name is {} and chopped name is {}",
                path.file_name().unwrap().to_str().unwrap(),
                chopped_name
            );
            let mut new_path = path.clone().to_owned();
            new_path.set_file_name(chopped_name);
            fs::rename(&path, &new_path)
                .expect(format!("Cannot rename {:?} to {:?}", path, &new_path).as_str());
        } else if path.is_dir() && sub {
            subs.push(String::from(
                path.to_str().expect("Cannot convert path to string"),
            ))
        }
    }
    for p in &subs {
        chop(p.as_str(), sub, cap, ascii);
    }
}

fn calculate_name(name: &OsStr, chop_size: usize, ascii: bool) -> OsString {
    let rand = uuid::Uuid::new_v4().to_string()[..11].to_owned();
    let index = chop_size - rand.len();
    if index < 1 {
        return OsString::from(name);
    }
    let byte_slice = &name.as_bytes()[..index];
    let lossy_name = match ascii {
        true => String::from_utf8_lossy(&byte_slice).to_string().replace(
            |c: char| !c.is_ascii(),
            String::from(get_random_ascii_printable_code()).as_str(),
        ),
        false => String::from_utf8_lossy(&byte_slice).to_string(),
    };
    let name = format!("{}{}", lossy_name, rand);
    match name.len() > chop_size {
        true => OsString::from(&name[..chop_size]),
        false => OsString::from(name),
    }
}
