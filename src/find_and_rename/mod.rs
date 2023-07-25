use crate::rand_ascii::get_random_ascii_printable_code;
use std::cmp::Ordering;
use std::collections::HashSet;
use std::ffi::{OsStr, OsString};
use std::fs;
use std::os::unix::prelude::OsStrExt;
use std::time::{Duration, SystemTime};

pub fn chop(
    path: &str,
    sub: bool,
    cap: usize,
    ascii: bool,
    dry: bool,
    created: Option<Duration>,
    modified: Option<Duration>,
    ignore: &Option<String>,
) {
    let current_dir = std::path::PathBuf::from(path);
    let mut subs = vec![];
    if !current_dir.exists() {
        println!("Invalid path specified: {}", path);
        return;
    }
    let mut forbidden_extensions: Option<HashSet<&str>> = None;
    if ignore.is_some() {
        forbidden_extensions = Some(ignore.as_ref().unwrap().split(",").collect());
    }
    for entry in fs::read_dir(current_dir).expect("Not a valid directory") {
        let entry = entry.expect("Not valid");
        let path = entry.path();
        if path.is_file() && path.file_name().unwrap().len() > cap {
            let extension = path.extension();
            if forbidden_extensions.is_some() && extension.is_some() {
                let forbidden_extensions = forbidden_extensions.as_ref().unwrap();
                let extension = extension.as_ref().unwrap().to_str().unwrap();
                if forbidden_extensions.contains(extension) {
                    println!(
                        "Skipping this file since it is on the ignore list {:#?}",
                        path
                    );
                    continue;
                }
            }
            let chopped_name;
            let metadata = fs::metadata(&path).expect("Unable to get metadata");
            let creation_time = SystemTime::now()
                .duration_since(metadata.created().expect("Cannot get creation time"))
                .expect("Cannot calculate duration");
            let modified_time = SystemTime::now()
                .duration_since(metadata.modified().expect("Cannot get modified time"))
                .expect("Cannot calculate duration");
            if modified.is_some() && modified.unwrap().cmp(&modified_time) == Ordering::Greater {
                continue;
            }
            if created.is_some() && created.unwrap().cmp(&creation_time) == Ordering::Greater {
                continue;
            }
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
            match dry {
                true => {
                    println!(
                        "DRY RUN ONLY: Original name is {} and chopped name is {}",
                        path.file_name().unwrap().to_str().unwrap(),
                        chopped_name
                    );
                }
                false => {
                    println!(
                        "Original name is {} and chopped name is {}",
                        path.file_name().unwrap().to_str().unwrap(),
                        chopped_name
                    );
                    let mut new_path = path.clone().to_owned();
                    new_path.set_file_name(chopped_name);
                    fs::rename(&path, &new_path)
                        .expect(format!("Cannot rename {:?} to {:?}", path, &new_path).as_str());
                }
            };
        } else if path.is_dir() && sub {
            subs.push(String::from(
                path.to_str().expect("Cannot convert path to string"),
            ))
        }
    }
    for p in &subs {
        chop(p.as_str(), sub, cap, ascii, dry, created, modified, ignore);
    }
}

fn calculate_name(name: &OsStr, chop_size: usize, ascii: bool) -> OsString {
    let rand = uuid::Uuid::new_v4().to_string()[..10].to_owned();
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
    let name = name.as_bytes();
    match name.len() > chop_size {
        true => OsString::from(String::from_utf8_lossy(&name[..chop_size]).to_string()),
        false => OsString::from(String::from_utf8_lossy(&name).to_string()),
    }
}
