use std::env;
use std::io;
use std::path::{Path, PathBuf};

pub fn to_path_buf(value: &str) -> Result<PathBuf, io::Error> {
    let path = Path::new(value);
    if path.is_absolute() {
        Ok(path.to_path_buf())
    } else {
        env::current_dir()
            .map(|cwd| cwd.join(path))
            .and_then(|absolute_path| absolute_path.canonicalize())
    }
}
