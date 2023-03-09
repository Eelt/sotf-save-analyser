
pub fn validate_path_gui(path: &String) -> bool {

    let read_contents = std::fs::read_to_string(path);

    if !read_contents.is_err() {
        return true;
    } 
    
    false
}