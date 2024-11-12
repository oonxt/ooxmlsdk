use hard_xml::{XmlRead, XmlWrite};
pub const RELATIONSHIP_TYPE: &str = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/package";
#[derive(Clone, Debug, Default)]
pub struct EmbeddedPackagePart {
    pub r_id: String,
    pub inner_path: String,
    pub path: String,
}
impl EmbeddedPackagePart {
    #[allow(unused_variables)]
    pub(crate) fn new_from_archive(
        parent_path: &str,
        path: &str,
        r_id: &str,
        file_path_set: &std::collections::HashSet<String>,
        archive: &mut zip::ZipArchive<std::io::BufReader<std::fs::File>>,
    ) -> Result<Self, crate::common::SdkError> {
        Ok(Self {
            r_id: r_id.to_string(),
            inner_path: path.to_string(),
            path: path.to_string(),
        })
    }
    #[allow(unused_variables)]
    pub(crate) fn save_zip(
        &self,
        parent_path: &str,
        zip: &mut zip::ZipWriter<std::fs::File>,
        entry_set: &mut std::collections::HashSet<String>,
    ) -> Result<(), crate::common::SdkError> {
        use std::io::Write;
        let options = zip::write::SimpleFileOptions::default()
            .compression_method(zip::CompressionMethod::Deflated)
            .unix_permissions(0o755);
        let directory_path = crate::common::resolve_zip_file_path(parent_path);
        if !directory_path.is_empty() && !entry_set.contains(&directory_path) {
            zip.add_directory(
                &directory_path,
                zip::write::SimpleFileOptions::default(),
            )?;
            entry_set.insert(directory_path);
        }
        let embedded_package_part_dir_path = crate::common::resolve_zip_file_path(
            &format!("{}{}/", parent_path, "embeddings"),
        );
        if !embedded_package_part_dir_path.is_empty()
            && !entry_set.contains(&embedded_package_part_dir_path)
        {
            zip.add_directory(
                &embedded_package_part_dir_path,
                zip::write::SimpleFileOptions::default(),
            )?;
            entry_set.insert(embedded_package_part_dir_path);
        }
        use std::io::Read;
        let mut buffer = Vec::new();
        let mut file = std::fs::File::open(&self.path)?;
        file.read_to_end(&mut buffer)?;
        if !entry_set.contains(&self.inner_path) {
            zip.start_file(&self.inner_path, options)?;
            zip.write_all(crate::common::SCHEMA_XML.as_bytes())?;
            zip.write_all(&buffer)?;
            entry_set.insert(self.inner_path.to_string());
        }
        buffer.clear();
        Ok(())
    }
}
