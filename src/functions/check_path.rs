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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_path_valid() {
        let path_str: &'static str = ".";
        let file_name: &'static str = "test_file";

        let result: Result<String, SpudError> = check_path(path_str, file_name);

        assert!(result.is_ok());

        assert_eq!(result.unwrap(), format!("{path_str}/{file_name}.spud"));
    }
}
