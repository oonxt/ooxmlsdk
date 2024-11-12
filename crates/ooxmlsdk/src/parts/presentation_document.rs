use hard_xml::{XmlRead, XmlWrite};
pub const RELATIONSHIP_TYPE: &str = "";
#[derive(Clone, Debug, Default)]
pub struct PresentationDocument {
    pub content_types: crate::packages::opc_content_types::Types,
    pub relationships: Option<crate::packages::opc_relationships::Relationships>,
    pub rels_path: String,
    pub inner_path: String,
    pub presentation_part: crate::parts::presentation_part::PresentationPart,
    pub core_file_properties_part: Option<
        crate::parts::core_file_properties_part::CoreFilePropertiesPart,
    >,
    pub extended_file_properties_part: Option<
        crate::parts::extended_file_properties_part::ExtendedFilePropertiesPart,
    >,
    pub custom_file_properties_part: Option<
        crate::parts::custom_file_properties_part::CustomFilePropertiesPart,
    >,
    pub thumbnail_part: Option<crate::parts::thumbnail_part::ThumbnailPart>,
    pub digital_signature_origin_part: Option<
        crate::parts::digital_signature_origin_part::DigitalSignatureOriginPart,
    >,
    pub quick_access_toolbar_customizations_part: Option<
        crate::parts::quick_access_toolbar_customizations_part::QuickAccessToolbarCustomizationsPart,
    >,
    pub ribbon_extensibility_part: Option<
        crate::parts::ribbon_extensibility_part::RibbonExtensibilityPart,
    >,
    pub ribbon_and_backstage_customizations_part: Option<
        crate::parts::ribbon_and_backstage_customizations_part::RibbonAndBackstageCustomizationsPart,
    >,
    pub web_ex_taskpanes_part: Option<
        crate::parts::web_ex_taskpanes_part::WebExTaskpanesPart,
    >,
    pub label_info_part: Option<crate::parts::label_info_part::LabelInfoPart>,
}
impl PresentationDocument {
    pub fn new(path: &str) -> Result<Self, crate::common::SdkError> {
        let zip_file = std::fs::File::open(path)?;
        let reader = std::io::BufReader::new(zip_file);
        let mut archive = zip::ZipArchive::new(reader)?;
        let mut file_path_set: std::collections::HashSet<String> = std::collections::HashSet::new();
        for i in 0..archive.len() {
            let file = archive.by_index(i)?;
            let file_path = match file.enclosed_name() {
                Some(path) => path.to_string_lossy().to_string(),
                None => {
                    continue;
                }
            };
            file_path_set.insert(file_path);
        }
        Self::new_from_archive("", "", "", &file_path_set, &mut archive)
    }
    #[allow(unused_variables)]
    pub(crate) fn new_from_archive(
        parent_path: &str,
        path: &str,
        r_id: &str,
        file_path_set: &std::collections::HashSet<String>,
        archive: &mut zip::ZipArchive<std::io::BufReader<std::fs::File>>,
    ) -> Result<Self, crate::common::SdkError> {
        let content_types = crate::packages::opc_content_types::Types::from_str(
            &crate::common::read_string(&mut archive.by_name("[Content_Types].xml")?)?,
        )?;
        let mut rels_path = "".to_string();
        let child_parent_path = format!("{}{}", parent_path, "");
        let part_target_str = if path.ends_with(".xml") {
            &path[path
                .rfind('/')
                .ok_or_else(|| crate::common::SdkError::CommonError(path.to_string()))?
                + 1..path.len()]
        } else {
            ""
        };
        let presentation_document_rels_path = crate::common::resolve_zip_file_path(
            &format!("{}_rels/{}.rels", child_parent_path, part_target_str),
        );
        let relationships = if let Some(file_path) = file_path_set
            .get(&presentation_document_rels_path)
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
        let mut presentation_part: Option<
            crate::parts::presentation_part::PresentationPart,
        > = None;
        let mut core_file_properties_part: Option<
            crate::parts::core_file_properties_part::CoreFilePropertiesPart,
        > = None;
        let mut extended_file_properties_part: Option<
            crate::parts::extended_file_properties_part::ExtendedFilePropertiesPart,
        > = None;
        let mut custom_file_properties_part: Option<
            crate::parts::custom_file_properties_part::CustomFilePropertiesPart,
        > = None;
        let mut thumbnail_part: Option<crate::parts::thumbnail_part::ThumbnailPart> = None;
        let mut digital_signature_origin_part: Option<
            crate::parts::digital_signature_origin_part::DigitalSignatureOriginPart,
        > = None;
        let mut quick_access_toolbar_customizations_part: Option<
            crate::parts::quick_access_toolbar_customizations_part::QuickAccessToolbarCustomizationsPart,
        > = None;
        let mut ribbon_extensibility_part: Option<
            crate::parts::ribbon_extensibility_part::RibbonExtensibilityPart,
        > = None;
        let mut ribbon_and_backstage_customizations_part: Option<
            crate::parts::ribbon_and_backstage_customizations_part::RibbonAndBackstageCustomizationsPart,
        > = None;
        let mut web_ex_taskpanes_part: Option<
            crate::parts::web_ex_taskpanes_part::WebExTaskpanesPart,
        > = None;
        let mut label_info_part: Option<crate::parts::label_info_part::LabelInfoPart> = None;
        if let Some(relationships) = &relationships {
            for relationship in &relationships.relationship {
                #[allow(clippy::single_match)]
                match relationship.r#type.as_str() {
                    crate::parts::presentation_part::RELATIONSHIP_TYPE => {
                        let target_path = crate::common::resolve_zip_file_path(
                            &format!("{}{}", child_parent_path, relationship.target),
                        );
                        presentation_part = Some(
                            crate::parts::presentation_part::PresentationPart::new_from_archive(
                                &child_parent_path,
                                &target_path,
                                &relationship.id,
                                file_path_set,
                                archive,
                            )?,
                        );
                    }
                    crate::parts::core_file_properties_part::RELATIONSHIP_TYPE => {
                        let target_path = crate::common::resolve_zip_file_path(
                            &format!("{}{}", child_parent_path, relationship.target),
                        );
                        core_file_properties_part = Some(
                            crate::parts::core_file_properties_part::CoreFilePropertiesPart::new_from_archive(
                                &child_parent_path,
                                &target_path,
                                &relationship.id,
                                file_path_set,
                                archive,
                            )?,
                        );
                    }
                    crate::parts::extended_file_properties_part::RELATIONSHIP_TYPE => {
                        let target_path = crate::common::resolve_zip_file_path(
                            &format!("{}{}", child_parent_path, relationship.target),
                        );
                        extended_file_properties_part = Some(
                            crate::parts::extended_file_properties_part::ExtendedFilePropertiesPart::new_from_archive(
                                &child_parent_path,
                                &target_path,
                                &relationship.id,
                                file_path_set,
                                archive,
                            )?,
                        );
                    }
                    crate::parts::custom_file_properties_part::RELATIONSHIP_TYPE => {
                        let target_path = crate::common::resolve_zip_file_path(
                            &format!("{}{}", child_parent_path, relationship.target),
                        );
                        custom_file_properties_part = Some(
                            crate::parts::custom_file_properties_part::CustomFilePropertiesPart::new_from_archive(
                                &child_parent_path,
                                &target_path,
                                &relationship.id,
                                file_path_set,
                                archive,
                            )?,
                        );
                    }
                    crate::parts::thumbnail_part::RELATIONSHIP_TYPE => {
                        let target_path = crate::common::resolve_zip_file_path(
                            &format!("{}{}", child_parent_path, relationship.target),
                        );
                        thumbnail_part = Some(
                            crate::parts::thumbnail_part::ThumbnailPart::new_from_archive(
                                &child_parent_path,
                                &target_path,
                                &relationship.id,
                                file_path_set,
                                archive,
                            )?,
                        );
                    }
                    crate::parts::digital_signature_origin_part::RELATIONSHIP_TYPE => {
                        let target_path = crate::common::resolve_zip_file_path(
                            &format!("{}{}", child_parent_path, relationship.target),
                        );
                        digital_signature_origin_part = Some(
                            crate::parts::digital_signature_origin_part::DigitalSignatureOriginPart::new_from_archive(
                                &child_parent_path,
                                &target_path,
                                &relationship.id,
                                file_path_set,
                                archive,
                            )?,
                        );
                    }
                    crate::parts::quick_access_toolbar_customizations_part::RELATIONSHIP_TYPE => {
                        let target_path = crate::common::resolve_zip_file_path(
                            &format!("{}{}", child_parent_path, relationship.target),
                        );
                        quick_access_toolbar_customizations_part = Some(
                            crate::parts::quick_access_toolbar_customizations_part::QuickAccessToolbarCustomizationsPart::new_from_archive(
                                &child_parent_path,
                                &target_path,
                                &relationship.id,
                                file_path_set,
                                archive,
                            )?,
                        );
                    }
                    crate::parts::ribbon_extensibility_part::RELATIONSHIP_TYPE => {
                        let target_path = crate::common::resolve_zip_file_path(
                            &format!("{}{}", child_parent_path, relationship.target),
                        );
                        ribbon_extensibility_part = Some(
                            crate::parts::ribbon_extensibility_part::RibbonExtensibilityPart::new_from_archive(
                                &child_parent_path,
                                &target_path,
                                &relationship.id,
                                file_path_set,
                                archive,
                            )?,
                        );
                    }
                    crate::parts::ribbon_and_backstage_customizations_part::RELATIONSHIP_TYPE => {
                        let target_path = crate::common::resolve_zip_file_path(
                            &format!("{}{}", child_parent_path, relationship.target),
                        );
                        ribbon_and_backstage_customizations_part = Some(
                            crate::parts::ribbon_and_backstage_customizations_part::RibbonAndBackstageCustomizationsPart::new_from_archive(
                                &child_parent_path,
                                &target_path,
                                &relationship.id,
                                file_path_set,
                                archive,
                            )?,
                        );
                    }
                    crate::parts::web_ex_taskpanes_part::RELATIONSHIP_TYPE => {
                        let target_path = crate::common::resolve_zip_file_path(
                            &format!("{}{}", child_parent_path, relationship.target),
                        );
                        web_ex_taskpanes_part = Some(
                            crate::parts::web_ex_taskpanes_part::WebExTaskpanesPart::new_from_archive(
                                &child_parent_path,
                                &target_path,
                                &relationship.id,
                                file_path_set,
                                archive,
                            )?,
                        );
                    }
                    crate::parts::label_info_part::RELATIONSHIP_TYPE => {
                        let target_path = crate::common::resolve_zip_file_path(
                            &format!("{}{}", child_parent_path, relationship.target),
                        );
                        label_info_part = Some(
                            crate::parts::label_info_part::LabelInfoPart::new_from_archive(
                                &child_parent_path,
                                &target_path,
                                &relationship.id,
                                file_path_set,
                                archive,
                            )?,
                        );
                    }
                    _ => {}
                }
            }
        }
        let presentation_part = presentation_part
            .ok_or_else(|| crate::common::SdkError::CommonError(
                "presentation_part".to_string(),
            ))?;
        Ok(Self {
            content_types,
            rels_path,
            relationships,
            inner_path: path.to_string(),
            presentation_part,
            core_file_properties_part,
            extended_file_properties_part,
            custom_file_properties_part,
            thumbnail_part,
            digital_signature_origin_part,
            quick_access_toolbar_customizations_part,
            ribbon_extensibility_part,
            ribbon_and_backstage_customizations_part,
            web_ex_taskpanes_part,
            label_info_part,
        })
    }
    pub fn save(&self, path: &str) -> Result<(), crate::common::SdkError> {
        use std::io::Write;
        let mut entry_set: std::collections::HashSet<String> = std::collections::HashSet::new();
        let path = std::path::Path::new(path);
        let file = std::fs::File::create(path)?;
        let mut zip = zip::ZipWriter::new(file);
        let options = zip::write::SimpleFileOptions::default()
            .compression_method(zip::CompressionMethod::Deflated)
            .unix_permissions(0o755);
        zip.start_file("[Content_Types].xml", options)?;
        zip.write_all(self.content_types.to_string()?.as_bytes())?;
        self.save_zip("", &mut zip, &mut entry_set)?;
        zip.finish()?;
        Ok(())
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
        let presentation_document_dir_path = crate::common::resolve_zip_file_path(
            &format!("{}{}/", parent_path, ""),
        );
        if !presentation_document_dir_path.is_empty()
            && !entry_set.contains(&presentation_document_dir_path)
        {
            zip.add_directory(
                &presentation_document_dir_path,
                zip::write::SimpleFileOptions::default(),
            )?;
            entry_set.insert(presentation_document_dir_path);
        }
        let child_parent_path = format!("{}{}", parent_path, "");
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
                zip.write_all(relationships.to_string()?.as_bytes())?;
                entry_set.insert(self.rels_path.to_string());
            }
        }
        self.presentation_part.save_zip(&child_parent_path, zip, entry_set)?;
        if let Some(core_file_properties_part) = &self.core_file_properties_part {
            core_file_properties_part.save_zip(&child_parent_path, zip, entry_set)?;
        }
        if let Some(extended_file_properties_part) = &self.extended_file_properties_part
        {
            extended_file_properties_part.save_zip(&child_parent_path, zip, entry_set)?;
        }
        if let Some(custom_file_properties_part) = &self.custom_file_properties_part {
            custom_file_properties_part.save_zip(&child_parent_path, zip, entry_set)?;
        }
        if let Some(thumbnail_part) = &self.thumbnail_part {
            thumbnail_part.save_zip(&child_parent_path, zip, entry_set)?;
        }
        if let Some(digital_signature_origin_part) = &self.digital_signature_origin_part
        {
            digital_signature_origin_part.save_zip(&child_parent_path, zip, entry_set)?;
        }
        if let Some(quick_access_toolbar_customizations_part) = &self
            .quick_access_toolbar_customizations_part
        {
            quick_access_toolbar_customizations_part
                .save_zip(&child_parent_path, zip, entry_set)?;
        }
        if let Some(ribbon_extensibility_part) = &self.ribbon_extensibility_part {
            ribbon_extensibility_part.save_zip(&child_parent_path, zip, entry_set)?;
        }
        if let Some(ribbon_and_backstage_customizations_part) = &self
            .ribbon_and_backstage_customizations_part
        {
            ribbon_and_backstage_customizations_part
                .save_zip(&child_parent_path, zip, entry_set)?;
        }
        if let Some(web_ex_taskpanes_part) = &self.web_ex_taskpanes_part {
            web_ex_taskpanes_part.save_zip(&child_parent_path, zip, entry_set)?;
        }
        if let Some(label_info_part) = &self.label_info_part {
            label_info_part.save_zip(&child_parent_path, zip, entry_set)?;
        }
        Ok(())
    }
}
