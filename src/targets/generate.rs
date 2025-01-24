use anyhow::Result;
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};

use super::FileObject;
use crate::packet::{FieldType, Packet};

pub trait Lang {
    fn type_map(ft: &FieldType) -> &str;
    fn render_template(packets: &[Packet]) -> Result<Vec<FileObject>>;
}

pub struct CodeGenerator<'a> {
    dest: &'a Path,
}

impl<'a> CodeGenerator<'a> {
    pub fn new(dest: &'a Path) -> Self {
        CodeGenerator { dest }
    }

    pub fn to_code<T: Lang>(&self, packets: &[Packet]) -> Result<Vec<PathBuf>> {
        // Call out to the target to generate the new files
        let file_objs = T::render_template(packets)?;

        let mut written_files = vec![];

        for file_obj in file_objs.as_slice() {
            let dest_file = self.dest.join(&file_obj.filename);
            self.save_file(&dest_file, &file_obj.contents)?;
            written_files.push(dest_file);
        }

        Ok(written_files)
    }

    fn save_file(&self, filename: &Path, content_str: &str) -> Result<()> {
        // Ensure entire path exists
        fs::create_dir_all(filename.parent().expect("invalid parent path"))?;

        let mut file = File::create(filename)?;
        file.write_all(content_str.as_bytes())?;

        Ok(())
    }
}
