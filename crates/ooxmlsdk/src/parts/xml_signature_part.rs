use hard_xml::{XmlRead, XmlWrite};
pub const RELATIONSHIP_TYPE: &str = "http://schemas.openxmlformats.org/package/2006/relationships/digital-signature/signature";
#[derive(Clone, Debug, Default)]
pub struct XmlSignaturePart {
    pub r_id: String,
    pub inner_path: String,
    pub content: String,
}
impl XmlSignaturePart {
    #[allow(unused_variables)]
    pub(crate) fn new_from_archive(
        parent_path: &str,
        path: &str,
        r_id: &str,
        file_path_set: &std::collections::HashSet<String>,
        archive: &mut zip::ZipArchive<std::io::BufReader<std::fs::File>>,
    ) -> Result<Self, crate::common::SdkError> {
        use std::io::Read;
        let mut content = String::new();
        {
            let mut file = std::io::BufReader::new(archive.by_name(path)?);
            file.read_to_string(&mut content)?;
        };
        Ok(Self {
            r_id: r_id.to_string(),
            inner_path: path.to_string(),
            content,
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
        let xml_signature_part_dir_path = crate::common::resolve_zip_file_path(
            &format!("{}{}/", parent_path, "_xmlsignatures"),
        );
        if !xml_signature_part_dir_path.is_empty()
            && !entry_set.contains(&xml_signature_part_dir_path)
        {
            zip.add_directory(
                &xml_signature_part_dir_path,
                zip::write::SimpleFileOptions::default(),
            )?;
            entry_set.insert(xml_signature_part_dir_path);
        }
        if !entry_set.contains(&self.inner_path) {
            zip.start_file(&self.inner_path, options)?;
            zip.write_all(self.content.as_bytes())?;
            entry_set.insert(self.inner_path.to_string());
        }
        Ok(())
    }
}
