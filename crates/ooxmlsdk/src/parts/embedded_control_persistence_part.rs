use hard_xml::{XmlRead, XmlWrite};
pub const RELATIONSHIP_TYPE: &str = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/control";
#[derive(Clone, Debug, Default)]
pub struct EmbeddedControlPersistencePart {
    pub r_id: String,
    pub relationships: Option<crate::packages::opc_relationships::Relationships>,
    pub rels_path: String,
    pub inner_path: String,
    pub path: String,
    data: Option<Vec<u8>>,
    pub embedded_control_persistence_binary_data_parts: Vec<
        crate::parts::embedded_control_persistence_binary_data_part::EmbeddedControlPersistenceBinaryDataPart,
    >,
}
impl EmbeddedControlPersistencePart {
    #[allow(unused_variables)]
    pub(crate) fn new_from_archive(
        parent_path: &str,
        path: &str,
        r_id: &str,
        file_path_set: &std::collections::HashSet<String>,
        archive: &mut zip::ZipArchive<std::io::BufReader<std::fs::File>>,
    ) -> Result<Self, crate::common::SdkError> {
        let mut rels_path = "".to_string();
        let child_parent_path = format!("{}{}", parent_path, "embeddings/");
        let part_target_str = if path.ends_with(".xml") {
            &path[path
                .rfind('/')
                .ok_or_else(|| crate::common::SdkError::CommonError(path.to_string()))?
                + 1..path.len()]
        } else {
            ""
        };
        let embedded_control_persistence_part_rels_path = crate::common::resolve_zip_file_path(
            &format!("{}_rels/{}.rels", child_parent_path, part_target_str),
        );
        let relationships = if let Some(file_path) = file_path_set
            .get(&embedded_control_persistence_part_rels_path)
        {
            rels_path = file_path.to_string();
            Some(
                crate::packages::opc_relationships::Relationships::from_str(
                    &crate::common::read_string(&mut archive.by_name(file_path)?)?,
                )?,
            )
        } else {
            None
        };
        let data = match archive.by_name(path) {
            Ok(mut file) => {
                use std::io::Read;
                let mut buffer = Vec::new();
                file.read_to_end(&mut buffer)?;
                Some(buffer)
            }
            Err(_) => None,
        };
        let mut embedded_control_persistence_binary_data_parts: Vec<
            crate::parts::embedded_control_persistence_binary_data_part::EmbeddedControlPersistenceBinaryDataPart,
        > = vec![];
        if let Some(relationships) = &relationships {
            for relationship in &relationships.relationship {
                #[allow(clippy::single_match)]
                match relationship.r#type.as_str() {
                    crate::parts::embedded_control_persistence_binary_data_part::RELATIONSHIP_TYPE => {
                        let target_path = crate::common::resolve_zip_file_path(
                            &format!("{}{}", child_parent_path, relationship.target),
                        );
                        let embedded_control_persistence_binary_data_part = crate::parts::embedded_control_persistence_binary_data_part::EmbeddedControlPersistenceBinaryDataPart::new_from_archive(
                            &child_parent_path,
                            &target_path,
                            &relationship.id,
                            file_path_set,
                            archive,
                        )?;
                        embedded_control_persistence_binary_data_parts
                            .push(embedded_control_persistence_binary_data_part);
                    }
                    _ => {}
                }
            }
        }
        Ok(Self {
            r_id: r_id.to_string(),
            rels_path,
            relationships,
            inner_path: path.to_string(),
            path: path.to_string(),
            data,
            embedded_control_persistence_binary_data_parts,
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
        let embedded_control_persistence_part_dir_path = crate::common::resolve_zip_file_path(
            &format!("{}{}/", parent_path, "embeddings"),
        );
        if !embedded_control_persistence_part_dir_path.is_empty()
            && !entry_set.contains(&embedded_control_persistence_part_dir_path)
        {
            zip.add_directory(
                &embedded_control_persistence_part_dir_path,
                zip::write::SimpleFileOptions::default(),
            )?;
            entry_set.insert(embedded_control_persistence_part_dir_path);
        }
        if !entry_set.contains(&self.inner_path) && self.data.is_some() {
            let mut data = self.data.clone().unwrap();
            zip.start_file(&self.inner_path, options)?;
            zip.write_all(&mut data)?;
            entry_set.insert(self.inner_path.to_string());
        } else {
            use std::io::Read;
            match std::fs::File::open(&self.path) {
                Ok(mut file) => {
                    let mut buffer = Vec::new();
                    file.read_to_end(&mut buffer)?;
                    zip.start_file(&self.inner_path, options)?;
                    zip.write_all(&buffer)?;
                    entry_set.insert(self.inner_path.to_string());
                }
                Err(e) => {}
            }
        }
        let child_parent_path = format!("{}{}", parent_path, "embeddings/");
        if let Some(relationships) = &self.relationships {
            let rels_dir_path = crate::common::resolve_zip_file_path(
                &format!("{}_rels", child_parent_path),
            );
            if !rels_dir_path.is_empty() && !entry_set.contains(&rels_dir_path) {
                zip.add_directory(
                    &rels_dir_path,
                    zip::write::SimpleFileOptions::default(),
                )?;
                entry_set.insert(rels_dir_path);
            }
            if !entry_set.contains(&self.rels_path) {
                zip.start_file(&self.rels_path, options)?;
                zip.write_all(crate::common::SCHEMA_XML.as_bytes())?;
                zip.write_all(relationships.to_string()?.as_bytes())?;
                entry_set.insert(self.rels_path.to_string());
            }
        }
        for embedded_control_persistence_binary_data_part in &self
            .embedded_control_persistence_binary_data_parts
        {
            embedded_control_persistence_binary_data_part
                .save_zip(&child_parent_path, zip, entry_set)?;
        }
        Ok(())
    }
}
