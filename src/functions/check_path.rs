use std::{error::Error, path::Path};

pub(crate) fn check_path(path_str: &str, file_name: &str) -> Result<String, Box<dyn Error>> {
    let path: &Path = Path::new(path_str);

    if !path.exists() {
        return Err(format!("Path {} does not exist", path.display()).into());
    }

    Ok(format!("{path_str}/{file_name}.spud"))
}
