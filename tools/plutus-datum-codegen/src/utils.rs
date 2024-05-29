use std::path::{Path, PathBuf};

pub fn convert_to_camel_case(ident: &str) -> String {
    let mut camel_case = String::new();
    let mut uppercase = true;
    for c in ident.chars() {
        match c {
            '_' | '-' => {
                uppercase = true;
            }
            '$' | '@' => {
                // ignored
            }
            c => {
                if uppercase {
                    camel_case.push(c.to_ascii_uppercase());
                    uppercase = false;
                } else {
                    camel_case.push(c);
                }
            }
        }
    }
    camel_case
}

pub fn copy_dir_all(from: &Path, to: &Path) -> std::io::Result<()> {
    std::fs::create_dir_all(to)?;
    for entry_res in std::fs::read_dir(from)? {
        let entry = entry_res?;
        if entry.file_type()?.is_dir() {
            copy_dir_all(&entry.path(), &to.join(entry.file_name()))?;
        } else {
            std::fs::copy(entry.path(), to.join(entry.file_name()))?;
        }
    }
    Ok(())
}

pub fn read_dir_to_string_map(
    map: &mut Vec<(PathBuf, String)>,
    dir: impl AsRef<Path>,
) -> std::io::Result<()> {
    for entry_res in std::fs::read_dir(&dir)? {
        let entry = entry_res?;
        if entry.file_type()?.is_dir() {
            read_dir_to_string_map(map, entry.path())?;
        } else {
            map.push((
                dir.as_ref().join(entry.file_name()).to_path_buf(),
                std::fs::read_to_string(entry.path())?,
            ));
        }
    }
    Ok(())
}
