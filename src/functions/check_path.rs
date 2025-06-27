use std::path::Path;

use crate::SpudError;

pub(crate) fn check_path(path_str: &str, file_name: &str) -> Result<String, SpudError> {
    let path: &Path = Path::new(path_str);

    if !path.exists() {
        return Err(SpudError::InvalidPath(format!(
            "Path {} does not exist",
            path.display()
        )));
    }

    Ok(format!("{path_str}/{file_name}.spud"))
}
