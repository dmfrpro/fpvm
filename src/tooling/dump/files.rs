use std::fs;
use std::path::{
    Path,
    PathBuf,
};

pub fn collect_input_files(input_path: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();

    if input_path.is_file() {
        files.push(input_path.to_path_buf());
        return files;
    }

    if input_path.is_dir() {
        collect_from_dir(input_path, &mut files);
        files.sort();
        return files;
    }

    panic!("input path does not exist: {}", input_path.display());
}

fn collect_from_dir(dir: &Path, files: &mut Vec<PathBuf>) {
    for entry in fs::read_dir(dir)
        .unwrap_or_else(|e| panic!("cannot read dir {}: {e}", dir.display()))
    {
        let path = entry
            .unwrap_or_else(|e| panic!("cannot read dir entry: {e}"))
            .path();

        if path.is_dir() {
            collect_from_dir(&path, files);
        } else if path.is_file() {
            files.push(path);
        }
    }
}

pub fn make_output_path(
    input_root: &Path,
    input_file: &Path,
    output_dir: &Path,
) -> PathBuf {
    if input_root.is_file() {
        let file_name = input_file
            .file_name()
            .unwrap_or_else(|| input_file.as_os_str());

        return output_dir
            .join(file_name)
            .with_extension("output");
    }

    let relative_path = input_file
        .strip_prefix(input_root)
        .unwrap_or(input_file);

    output_dir
        .join(relative_path)
        .with_extension("output")
}