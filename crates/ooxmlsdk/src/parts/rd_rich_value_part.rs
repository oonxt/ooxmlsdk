use hard_xml::{XmlRead, XmlWrite};
pub const RELATIONSHIP_TYPE: &str = "http://schemas.microsoft.com/office/2017/06/relationships/rdRichValue";
#[derive(Clone, Debug, Default)]
pub struct RdRichValuePart {
    pub r_id: String,
    pub inner_path: String,
    pub root_element: crate::schemas::schemas_microsoft_com_office_spreadsheetml_2017_richdata::RichValueData,
}
impl RdRichValuePart {
    #[allow(unused_variables)]
    pub(crate) fn new_from_archive(
        parent_path: &str,
        path: &str,
        r_id: &str,
        file_path_set: &std::collections::HashSet<String>,
        archive: &mut zip::ZipArchive<std::io::BufReader<std::fs::File>>,
    ) -> Result<Self, crate::common::SdkError> {
        let root_element = Some(
            crate::schemas::schemas_microsoft_com_office_spreadsheetml_2017_richdata::RichValueData::from_str(
                &crate::common::read_string(&mut archive.by_name(path)?)?,
            )?,
        );
        let root_element = root_element
            .ok_or_else(|| crate::common::SdkError::CommonError(
                "root_element".to_string(),
            ))?;
        Ok(Self {
            r_id: r_id.to_string(),
            inner_path: path.to_string(),
            root_element,
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
        let rd_rich_value_part_dir_path = crate::common::resolve_zip_file_path(
            &format!("{}{}/", parent_path, "richData"),
        );
        if !rd_rich_value_part_dir_path.is_empty()
            && !entry_set.contains(&rd_rich_value_part_dir_path)
        {
            zip.add_directory(
                &rd_rich_value_part_dir_path,
                zip::write::SimpleFileOptions::default(),
            )?;
            entry_set.insert(rd_rich_value_part_dir_path);
        }
        if !entry_set.contains(&self.inner_path) {
            zip.start_file(&self.inner_path, options)?;
            zip.write_all(crate::common::SCHEMA_XML.as_bytes())?;
            zip.write_all(self.root_element.to_string()?.as_bytes())?;
            entry_set.insert(self.inner_path.to_string());
        }
        Ok(())
    }
}
