use std::path::Path;

pub(crate) fn check_path(path_str: &str, file_name: &str) -> Option<String> {
    let path: &Path = Path::new(path_str);

    if !path.exists() {
        tracing::error!("Path {} does not exist", path.display());

        return None;
    }

    Some(format!("{path_str}/{file_name}.spud"))
}
