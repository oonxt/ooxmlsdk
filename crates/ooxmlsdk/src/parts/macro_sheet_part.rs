use hard_xml::{XmlRead, XmlWrite};
pub const RELATIONSHIP_TYPE: &str = "http://schemas.microsoft.com/office/2006/relationships/xlMacrosheet";
#[derive(Clone, Debug, Default)]
pub struct MacroSheetPart {
    pub r_id: String,
    pub relationships: Option<crate::packages::opc_relationships::Relationships>,
    pub rels_path: String,
    pub inner_path: String,
    pub root_element: crate::schemas::schemas_microsoft_com_office_excel_2006_main::Macrosheet,
    pub spreadsheet_printer_settings_parts: Vec<
        crate::parts::spreadsheet_printer_settings_part::SpreadsheetPrinterSettingsPart,
    >,
    pub drawings_part: Option<crate::parts::drawings_part::DrawingsPart>,
    pub vml_drawing_parts: Vec<crate::parts::vml_drawing_part::VmlDrawingPart>,
    pub worksheet_comments_part: Option<
        crate::parts::worksheet_comments_part::WorksheetCommentsPart,
    >,
    pub custom_property_parts: Vec<
        crate::parts::custom_property_part::CustomPropertyPart,
    >,
    pub embedded_object_parts: Vec<
        crate::parts::embedded_object_part::EmbeddedObjectPart,
    >,
    pub embedded_package_parts: Vec<
        crate::parts::embedded_package_part::EmbeddedPackagePart,
    >,
    pub image_parts: Vec<crate::parts::image_part::ImagePart>,
}
impl MacroSheetPart {
    #[allow(unused_variables)]
    pub(crate) fn new_from_archive(
        parent_path: &str,
        path: &str,
        r_id: &str,
        file_path_set: &std::collections::HashSet<String>,
        archive: &mut zip::ZipArchive<std::io::BufReader<std::fs::File>>,
    ) -> Result<Self, crate::common::SdkError> {
        let mut rels_path = "".to_string();
        let child_parent_path = format!("{}{}", parent_path, "macrosheets/");
        let part_target_str = if path.ends_with(".xml") {
            &path[path
                .rfind('/')
                .ok_or_else(|| crate::common::SdkError::CommonError(path.to_string()))?
                + 1..path.len()]
        } else {
            ""
        };
        let macro_sheet_part_rels_path = crate::common::resolve_zip_file_path(
            &format!("{}_rels/{}.rels", child_parent_path, part_target_str),
        );
        let relationships = if let Some(file_path) = file_path_set
            .get(&macro_sheet_part_rels_path)
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
        let root_element = Some(
            crate::schemas::schemas_microsoft_com_office_excel_2006_main::Macrosheet::from_str(
                &crate::common::read_string(&mut archive.by_name(path)?)?,
            )?,
        );
        let mut spreadsheet_printer_settings_parts: Vec<
            crate::parts::spreadsheet_printer_settings_part::SpreadsheetPrinterSettingsPart,
        > = vec![];
        let mut drawings_part: Option<crate::parts::drawings_part::DrawingsPart> = None;
        let mut vml_drawing_parts: Vec<crate::parts::vml_drawing_part::VmlDrawingPart> = vec![];
        let mut worksheet_comments_part: Option<
            crate::parts::worksheet_comments_part::WorksheetCommentsPart,
        > = None;
        let mut custom_property_parts: Vec<
            crate::parts::custom_property_part::CustomPropertyPart,
        > = vec![];
        let mut embedded_object_parts: Vec<
            crate::parts::embedded_object_part::EmbeddedObjectPart,
        > = vec![];
        let mut embedded_package_parts: Vec<
            crate::parts::embedded_package_part::EmbeddedPackagePart,
        > = vec![];
        let mut image_parts: Vec<crate::parts::image_part::ImagePart> = vec![];
        if let Some(relationships) = &relationships {
            for relationship in &relationships.relationship {
                #[allow(clippy::single_match)]
                match relationship.r#type.as_str() {
                    crate::parts::spreadsheet_printer_settings_part::RELATIONSHIP_TYPE => {
                        let target_path = crate::common::resolve_zip_file_path(
                            &format!("{}{}", child_parent_path, relationship.target),
                        );
                        let spreadsheet_printer_settings_part = crate::parts::spreadsheet_printer_settings_part::SpreadsheetPrinterSettingsPart::new_from_archive(
                            &child_parent_path,
                            &target_path,
                            &relationship.id,
                            file_path_set,
                            archive,
                        )?;
                        spreadsheet_printer_settings_parts
                            .push(spreadsheet_printer_settings_part);
                    }
                    crate::parts::drawings_part::RELATIONSHIP_TYPE => {
                        let target_path = crate::common::resolve_zip_file_path(
                            &format!("{}{}", child_parent_path, relationship.target),
                        );
                        drawings_part = Some(
                            crate::parts::drawings_part::DrawingsPart::new_from_archive(
                                &child_parent_path,
                                &target_path,
                                &relationship.id,
                                file_path_set,
                                archive,
                            )?,
                        );
                    }
                    crate::parts::vml_drawing_part::RELATIONSHIP_TYPE => {
                        let target_path = crate::common::resolve_zip_file_path(
                            &format!("{}{}", child_parent_path, relationship.target),
                        );
                        let vml_drawing_part = crate::parts::vml_drawing_part::VmlDrawingPart::new_from_archive(
                            &child_parent_path,
                            &target_path,
                            &relationship.id,
                            file_path_set,
                            archive,
                        )?;
                        vml_drawing_parts.push(vml_drawing_part);
                    }
                    crate::parts::worksheet_comments_part::RELATIONSHIP_TYPE => {
                        let target_path = crate::common::resolve_zip_file_path(
                            &format!("{}{}", child_parent_path, relationship.target),
                        );
                        worksheet_comments_part = Some(
                            crate::parts::worksheet_comments_part::WorksheetCommentsPart::new_from_archive(
                                &child_parent_path,
                                &target_path,
                                &relationship.id,
                                file_path_set,
                                archive,
                            )?,
                        );
                    }
                    crate::parts::custom_property_part::RELATIONSHIP_TYPE => {
                        let target_path = crate::common::resolve_zip_file_path(
                            &format!("{}{}", child_parent_path, relationship.target),
                        );
                        let custom_property_part = crate::parts::custom_property_part::CustomPropertyPart::new_from_archive(
                            &child_parent_path,
                            &target_path,
                            &relationship.id,
                            file_path_set,
                            archive,
                        )?;
                        custom_property_parts.push(custom_property_part);
                    }
                    crate::parts::embedded_object_part::RELATIONSHIP_TYPE => {
                        let target_path = crate::common::resolve_zip_file_path(
                            &format!("{}{}", child_parent_path, relationship.target),
                        );
                        let embedded_object_part = crate::parts::embedded_object_part::EmbeddedObjectPart::new_from_archive(
                            &child_parent_path,
                            &target_path,
                            &relationship.id,
                            file_path_set,
                            archive,
                        )?;
                        embedded_object_parts.push(embedded_object_part);
                    }
                    crate::parts::embedded_package_part::RELATIONSHIP_TYPE => {
                        let target_path = crate::common::resolve_zip_file_path(
                            &format!("{}{}", child_parent_path, relationship.target),
                        );
                        let embedded_package_part = crate::parts::embedded_package_part::EmbeddedPackagePart::new_from_archive(
                            &child_parent_path,
                            &target_path,
                            &relationship.id,
                            file_path_set,
                            archive,
                        )?;
                        embedded_package_parts.push(embedded_package_part);
                    }
                    crate::parts::image_part::RELATIONSHIP_TYPE => {
                        let target_path = crate::common::resolve_zip_file_path(
                            &format!("{}{}", child_parent_path, relationship.target),
                        );
                        let image_part = crate::parts::image_part::ImagePart::new_from_archive(
                            &child_parent_path,
                            &target_path,
                            &relationship.id,
                            file_path_set,
                            archive,
                        )?;
                        image_parts.push(image_part);
                    }
                    _ => {}
                }
            }
        }
        let root_element = root_element
            .ok_or_else(|| crate::common::SdkError::CommonError(
                "root_element".to_string(),
            ))?;
        Ok(Self {
            r_id: r_id.to_string(),
            rels_path,
            relationships,
            inner_path: path.to_string(),
            root_element,
            spreadsheet_printer_settings_parts,
            drawings_part,
            vml_drawing_parts,
            worksheet_comments_part,
            custom_property_parts,
            embedded_object_parts,
            embedded_package_parts,
            image_parts,
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
        let macro_sheet_part_dir_path = crate::common::resolve_zip_file_path(
            &format!("{}{}/", parent_path, "macrosheets"),
        );
        if !macro_sheet_part_dir_path.is_empty()
            && !entry_set.contains(&macro_sheet_part_dir_path)
        {
            zip.add_directory(
                &macro_sheet_part_dir_path,
                zip::write::SimpleFileOptions::default(),
            )?;
            entry_set.insert(macro_sheet_part_dir_path);
        }
        if !entry_set.contains(&self.inner_path) {
            zip.start_file(&self.inner_path, options)?;
            zip.write_all(self.root_element.to_string()?.as_bytes())?;
            entry_set.insert(self.inner_path.to_string());
        }
        let child_parent_path = format!("{}{}", parent_path, "macrosheets/");
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
        for spreadsheet_printer_settings_part in &self.spreadsheet_printer_settings_parts
        {
            spreadsheet_printer_settings_part
                .save_zip(&child_parent_path, zip, entry_set)?;
        }
        if let Some(drawings_part) = &self.drawings_part {
            drawings_part.save_zip(&child_parent_path, zip, entry_set)?;
        }
        for vml_drawing_part in &self.vml_drawing_parts {
            vml_drawing_part.save_zip(&child_parent_path, zip, entry_set)?;
        }
        if let Some(worksheet_comments_part) = &self.worksheet_comments_part {
            worksheet_comments_part.save_zip(&child_parent_path, zip, entry_set)?;
        }
        for custom_property_part in &self.custom_property_parts {
            custom_property_part.save_zip(&child_parent_path, zip, entry_set)?;
        }
        for embedded_object_part in &self.embedded_object_parts {
            embedded_object_part.save_zip(&child_parent_path, zip, entry_set)?;
        }
        for embedded_package_part in &self.embedded_package_parts {
            embedded_package_part.save_zip(&child_parent_path, zip, entry_set)?;
        }
        for image_part in &self.image_parts {
            image_part.save_zip(&child_parent_path, zip, entry_set)?;
        }
        Ok(())
    }
}
