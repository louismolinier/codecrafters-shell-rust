use std::path::Path;

pub fn is_in_folder(folder: &str, arg: &str) -> bool {
    let path = Path::new(folder).join(arg);
    return path.exists() && path.is_file();
}
