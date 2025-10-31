use std::fs;
use std::io::{self, Write};
use std::path::Path;

use crate::Editor;

pub fn load_from_file<P: AsRef<Path>>(path: P) -> io::Result<Editor> {
    let s = fs::read_to_string(path)?;
    Ok(Editor::from_str(&s))
}

pub fn save_to_file<P: AsRef<Path>>(editor: &Editor, path: P) -> io::Result<()> {
    let mut f = fs::File::create(path)?;
    f.write_all(editor.text().as_bytes())
}
