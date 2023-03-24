use std::{fs, string};


pub fn validate_path_gui(path: &String) -> bool {

    let read_dir = fs::read_dir(path);

    if read_dir.is_ok() {
        let unwrapped_dir = read_dir.unwrap();

        for file in unwrapped_dir.into_iter() {
            if file.is_err() {
                continue;
            }

            let string_result = fs::read_to_string(file.unwrap().path());

            if string_result.is_err() {
                continue;
            }
        }

        return true;
    }

    // println!("Detected the following: {}", );

    // let read_contents = std::fs::read_to_string(path);

    // if !read_contents.is_err() {
    //     return true;
    // } 
    
    false
}