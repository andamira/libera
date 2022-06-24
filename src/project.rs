// libera::project
//
//!
//

use std::{
    convert::AsRef,
    env, fs, io,
    path::{Path, PathBuf},
};

/// Returns an absolute `path`, relative to the project's root.
///
/// # Example
/// ```rust
/// # use libera::project_root_path;
/// match project_root_path("") {
///     Ok(p) => println!("Current project root is {:?}", p),
///     Err(e) => println!("Error obtaining project root {:?}", e)
/// };
/// ```
pub fn project_root_path<P: AsRef<Path>>(path: P) -> io::Result<PathBuf> {
    let current_path = env::current_dir()?;
    let mut root_path = current_path.clone();

    for p in current_path.as_path().ancestors() {
        let has_cargo = fs::read_dir(p)?
            .into_iter()
            .any(|p| p.unwrap().file_name() == *"Cargo.toml");
        if has_cargo {
            return Ok(root_path.join(path.as_ref()));
        } else {
            root_path.pop();
        }
    }
    Err(io::Error::new(
        io::ErrorKind::NotFound,
        "Ran out of places to find Cargo.toml",
    ))
}

/// Like [`project_root_path`] but returns a `String`.
///
/// In case of an error the returned string will be empty.
pub fn project_root_path_string<P: AsRef<Path>>(path: P) -> String {
    project_root_path(Path::new(path.as_ref()))
        .map_or("".into(), |p| p.to_str().unwrap().to_owned())
}
