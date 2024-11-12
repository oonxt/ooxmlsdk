use hard_xml::{XmlRead, XmlWrite};
pub const RELATIONSHIP_TYPE: &str = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/worksheet";
#[derive(Clone, Debug, Default)]
pub struct WorksheetPart {
    pub r_id: String,
    pub relationships: Option<crate::packages::opc_relationships::Relationships>,
    pub rels_path: String,
    pub inner_path: String,
    pub root_element: crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Worksheet,
    pub spreadsheet_printer_settings_parts: Vec<
        crate::parts::spreadsheet_printer_settings_part::SpreadsheetPrinterSettingsPart,
    >,
    pub drawings_part: Option<crate::parts::drawings_part::DrawingsPart>,
    pub vml_drawing_parts: Vec<crate::parts::vml_drawing_part::VmlDrawingPart>,
    pub worksheet_comments_part: Option<
        crate::parts::worksheet_comments_part::WorksheetCommentsPart,
    >,
    pub pivot_table_parts: Vec<crate::parts::pivot_table_part::PivotTablePart>,
    pub single_cell_table_part: Option<
        crate::parts::single_cell_table_part::SingleCellTablePart,
    >,
    pub table_definition_parts: Vec<
        crate::parts::table_definition_part::TableDefinitionPart,
    >,
    pub embedded_control_persistence_parts: Vec<
        crate::parts::embedded_control_persistence_part::EmbeddedControlPersistencePart,
    >,
    pub control_properties_parts: Vec<
        crate::parts::control_properties_part::ControlPropertiesPart,
    >,
    pub embedded_object_parts: Vec<
        crate::parts::embedded_object_part::EmbeddedObjectPart,
    >,
    pub embedded_package_parts: Vec<
        crate::parts::embedded_package_part::EmbeddedPackagePart,
    >,
    pub image_parts: Vec<crate::parts::image_part::ImagePart>,
    pub custom_property_parts: Vec<
        crate::parts::custom_property_part::CustomPropertyPart,
    >,
    pub worksheet_sort_map_part: Option<
        crate::parts::worksheet_sort_map_part::WorksheetSortMapPart,
    >,
    pub query_table_parts: Vec<crate::parts::query_table_part::QueryTablePart>,
    pub embedded_control_persistence_binary_data_parts: Vec<
        crate::parts::embedded_control_persistence_binary_data_part::EmbeddedControlPersistenceBinaryDataPart,
    >,
    pub slicers_parts: Vec<crate::parts::slicers_part::SlicersPart>,
    pub time_line_parts: Vec<crate::parts::time_line_part::TimeLinePart>,
    pub worksheet_threaded_comments_parts: Vec<
        crate::parts::worksheet_threaded_comments_part::WorksheetThreadedCommentsPart,
    >,
    pub model3_d_reference_relationship_parts: Vec<
        crate::parts::model3_d_reference_relationship_part::Model3DReferenceRelationshipPart,
    >,
    pub named_sheet_views_parts: Vec<
        crate::parts::named_sheet_views_part::NamedSheetViewsPart,
    >,
}
impl WorksheetPart {
    #[allow(unused_variables)]
    pub(crate) fn new_from_archive(
        parent_path: &str,
        path: &str,
        r_id: &str,
        file_path_set: &std::collections::HashSet<String>,
        archive: &mut zip::ZipArchive<std::io::BufReader<std::fs::File>>,
    ) -> Result<Self, crate::common::SdkError> {
        let mut rels_path = "".to_string();
        let child_parent_path = format!("{}{}", parent_path, "worksheets/");
        let part_target_str = if path.ends_with(".xml") {
            &path[path
                .rfind('/')
                .ok_or_else(|| crate::common::SdkError::CommonError(path.to_string()))?
                + 1..path.len()]
        } else {
            ""
        };
        let worksheet_part_rels_path = crate::common::resolve_zip_file_path(
            &format!("{}_rels/{}.rels", child_parent_path, part_target_str),
        );
        let relationships = if let Some(file_path) = file_path_set
            .get(&worksheet_part_rels_path)
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
            crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Worksheet::from_str(
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
        let mut pivot_table_parts: Vec<crate::parts::pivot_table_part::PivotTablePart> = vec![];
        let mut single_cell_table_part: Option<
            crate::parts::single_cell_table_part::SingleCellTablePart,
        > = None;
        let mut table_definition_parts: Vec<
            crate::parts::table_definition_part::TableDefinitionPart,
        > = vec![];
        let mut embedded_control_persistence_parts: Vec<
            crate::parts::embedded_control_persistence_part::EmbeddedControlPersistencePart,
        > = vec![];
        let mut control_properties_parts: Vec<
            crate::parts::control_properties_part::ControlPropertiesPart,
        > = vec![];
        let mut embedded_object_parts: Vec<
            crate::parts::embedded_object_part::EmbeddedObjectPart,
        > = vec![];
        let mut embedded_package_parts: Vec<
            crate::parts::embedded_package_part::EmbeddedPackagePart,
        > = vec![];
        let mut image_parts: Vec<crate::parts::image_part::ImagePart> = vec![];
        let mut custom_property_parts: Vec<
            crate::parts::custom_property_part::CustomPropertyPart,
        > = vec![];
        let mut worksheet_sort_map_part: Option<
            crate::parts::worksheet_sort_map_part::WorksheetSortMapPart,
        > = None;
        let mut query_table_parts: Vec<crate::parts::query_table_part::QueryTablePart> = vec![];
        let mut embedded_control_persistence_binary_data_parts: Vec<
            crate::parts::embedded_control_persistence_binary_data_part::EmbeddedControlPersistenceBinaryDataPart,
        > = vec![];
        let mut slicers_parts: Vec<crate::parts::slicers_part::SlicersPart> = vec![];
        let mut time_line_parts: Vec<crate::parts::time_line_part::TimeLinePart> = vec![];
        let mut worksheet_threaded_comments_parts: Vec<
            crate::parts::worksheet_threaded_comments_part::WorksheetThreadedCommentsPart,
        > = vec![];
        let mut model3_d_reference_relationship_parts: Vec<
            crate::parts::model3_d_reference_relationship_part::Model3DReferenceRelationshipPart,
        > = vec![];
        let mut named_sheet_views_parts: Vec<
            crate::parts::named_sheet_views_part::NamedSheetViewsPart,
        > = vec![];
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
                    crate::parts::pivot_table_part::RELATIONSHIP_TYPE => {
                        let target_path = crate::common::resolve_zip_file_path(
                            &format!("{}{}", child_parent_path, relationship.target),
                        );
                        let pivot_table_part = crate::parts::pivot_table_part::PivotTablePart::new_from_archive(
                            &child_parent_path,
                            &target_path,
                            &relationship.id,
                            file_path_set,
                            archive,
                        )?;
                        pivot_table_parts.push(pivot_table_part);
                    }
                    crate::parts::single_cell_table_part::RELATIONSHIP_TYPE => {
                        let target_path = crate::common::resolve_zip_file_path(
                            &format!("{}{}", child_parent_path, relationship.target),
                        );
                        single_cell_table_part = Some(
                            crate::parts::single_cell_table_part::SingleCellTablePart::new_from_archive(
                                &child_parent_path,
                                &target_path,
                                &relationship.id,
                                file_path_set,
                                archive,
                            )?,
                        );
                    }
                    crate::parts::table_definition_part::RELATIONSHIP_TYPE => {
                        let target_path = crate::common::resolve_zip_file_path(
                            &format!("{}{}", child_parent_path, relationship.target),
                        );
                        let table_definition_part = crate::parts::table_definition_part::TableDefinitionPart::new_from_archive(
                            &child_parent_path,
                            &target_path,
                            &relationship.id,
                            file_path_set,
                            archive,
                        )?;
                        table_definition_parts.push(table_definition_part);
                    }
                    crate::parts::embedded_control_persistence_part::RELATIONSHIP_TYPE => {
                        let target_path = crate::common::resolve_zip_file_path(
                            &format!("{}{}", child_parent_path, relationship.target),
                        );
                        let embedded_control_persistence_part = crate::parts::embedded_control_persistence_part::EmbeddedControlPersistencePart::new_from_archive(
                            &child_parent_path,
                            &target_path,
                            &relationship.id,
                            file_path_set,
                            archive,
                        )?;
                        embedded_control_persistence_parts
                            .push(embedded_control_persistence_part);
                    }
                    crate::parts::control_properties_part::RELATIONSHIP_TYPE => {
                        let target_path = crate::common::resolve_zip_file_path(
                            &format!("{}{}", child_parent_path, relationship.target),
                        );
                        let control_properties_part = crate::parts::control_properties_part::ControlPropertiesPart::new_from_archive(
                            &child_parent_path,
                            &target_path,
                            &relationship.id,
                            file_path_set,
                            archive,
                        )?;
                        control_properties_parts.push(control_properties_part);
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
                    crate::parts::worksheet_sort_map_part::RELATIONSHIP_TYPE => {
                        let target_path = crate::common::resolve_zip_file_path(
                            &format!("{}{}", child_parent_path, relationship.target),
                        );
                        worksheet_sort_map_part = Some(
                            crate::parts::worksheet_sort_map_part::WorksheetSortMapPart::new_from_archive(
                                &child_parent_path,
                                &target_path,
                                &relationship.id,
                                file_path_set,
                                archive,
                            )?,
                        );
                    }
                    crate::parts::query_table_part::RELATIONSHIP_TYPE => {
                        let target_path = crate::common::resolve_zip_file_path(
                            &format!("{}{}", child_parent_path, relationship.target),
                        );
                        let query_table_part = crate::parts::query_table_part::QueryTablePart::new_from_archive(
                            &child_parent_path,
                            &target_path,
                            &relationship.id,
                            file_path_set,
                            archive,
                        )?;
                        query_table_parts.push(query_table_part);
                    }
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
                    crate::parts::slicers_part::RELATIONSHIP_TYPE => {
                        let target_path = crate::common::resolve_zip_file_path(
                            &format!("{}{}", child_parent_path, relationship.target),
                        );
                        let slicers_part = crate::parts::slicers_part::SlicersPart::new_from_archive(
                            &child_parent_path,
                            &target_path,
                            &relationship.id,
                            file_path_set,
                            archive,
                        )?;
                        slicers_parts.push(slicers_part);
                    }
                    crate::parts::time_line_part::RELATIONSHIP_TYPE => {
                        let target_path = crate::common::resolve_zip_file_path(
                            &format!("{}{}", child_parent_path, relationship.target),
                        );
                        let time_line_part = crate::parts::time_line_part::TimeLinePart::new_from_archive(
                            &child_parent_path,
                            &target_path,
                            &relationship.id,
                            file_path_set,
                            archive,
                        )?;
                        time_line_parts.push(time_line_part);
                    }
                    crate::parts::worksheet_threaded_comments_part::RELATIONSHIP_TYPE => {
                        let target_path = crate::common::resolve_zip_file_path(
                            &format!("{}{}", child_parent_path, relationship.target),
                        );
                        let worksheet_threaded_comments_part = crate::parts::worksheet_threaded_comments_part::WorksheetThreadedCommentsPart::new_from_archive(
                            &child_parent_path,
                            &target_path,
                            &relationship.id,
                            file_path_set,
                            archive,
                        )?;
                        worksheet_threaded_comments_parts
                            .push(worksheet_threaded_comments_part);
                    }
                    crate::parts::model3_d_reference_relationship_part::RELATIONSHIP_TYPE => {
                        let target_path = crate::common::resolve_zip_file_path(
                            &format!("{}{}", child_parent_path, relationship.target),
                        );
                        let model3_d_reference_relationship_part = crate::parts::model3_d_reference_relationship_part::Model3DReferenceRelationshipPart::new_from_archive(
                            &child_parent_path,
                            &target_path,
                            &relationship.id,
                            file_path_set,
                            archive,
                        )?;
                        model3_d_reference_relationship_parts
                            .push(model3_d_reference_relationship_part);
                    }
                    crate::parts::named_sheet_views_part::RELATIONSHIP_TYPE => {
                        let target_path = crate::common::resolve_zip_file_path(
                            &format!("{}{}", child_parent_path, relationship.target),
                        );
                        let named_sheet_views_part = crate::parts::named_sheet_views_part::NamedSheetViewsPart::new_from_archive(
                            &child_parent_path,
                            &target_path,
                            &relationship.id,
                            file_path_set,
                            archive,
                        )?;
                        named_sheet_views_parts.push(named_sheet_views_part);
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
            pivot_table_parts,
            single_cell_table_part,
            table_definition_parts,
            embedded_control_persistence_parts,
            control_properties_parts,
            embedded_object_parts,
            embedded_package_parts,
            image_parts,
            custom_property_parts,
            worksheet_sort_map_part,
            query_table_parts,
            embedded_control_persistence_binary_data_parts,
            slicers_parts,
            time_line_parts,
            worksheet_threaded_comments_parts,
            model3_d_reference_relationship_parts,
            named_sheet_views_parts,
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
        let worksheet_part_dir_path = crate::common::resolve_zip_file_path(
            &format!("{}{}/", parent_path, "worksheets"),
        );
        if !worksheet_part_dir_path.is_empty()
            && !entry_set.contains(&worksheet_part_dir_path)
        {
            zip.add_directory(
                &worksheet_part_dir_path,
                zip::write::SimpleFileOptions::default(),
            )?;
            entry_set.insert(worksheet_part_dir_path);
        }
        if !entry_set.contains(&self.inner_path) {
            zip.start_file(&self.inner_path, options)?;
            zip.write_all(crate::common::SCHEMA_XML.as_bytes())?;
            zip.write_all(self.root_element.to_string()?.as_bytes())?;
            entry_set.insert(self.inner_path.to_string());
        }
        let child_parent_path = format!("{}{}", parent_path, "worksheets/");
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
        for pivot_table_part in &self.pivot_table_parts {
            pivot_table_part.save_zip(&child_parent_path, zip, entry_set)?;
        }
        if let Some(single_cell_table_part) = &self.single_cell_table_part {
            single_cell_table_part.save_zip(&child_parent_path, zip, entry_set)?;
        }
        for table_definition_part in &self.table_definition_parts {
            table_definition_part.save_zip(&child_parent_path, zip, entry_set)?;
        }
        for embedded_control_persistence_part in &self.embedded_control_persistence_parts
        {
            embedded_control_persistence_part
                .save_zip(&child_parent_path, zip, entry_set)?;
        }
        for control_properties_part in &self.control_properties_parts {
            control_properties_part.save_zip(&child_parent_path, zip, entry_set)?;
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
        for custom_property_part in &self.custom_property_parts {
            custom_property_part.save_zip(&child_parent_path, zip, entry_set)?;
        }
        if let Some(worksheet_sort_map_part) = &self.worksheet_sort_map_part {
            worksheet_sort_map_part.save_zip(&child_parent_path, zip, entry_set)?;
        }
        for query_table_part in &self.query_table_parts {
            query_table_part.save_zip(&child_parent_path, zip, entry_set)?;
        }
        for embedded_control_persistence_binary_data_part in &self
            .embedded_control_persistence_binary_data_parts
        {
            embedded_control_persistence_binary_data_part
                .save_zip(&child_parent_path, zip, entry_set)?;
        }
        for slicers_part in &self.slicers_parts {
            slicers_part.save_zip(&child_parent_path, zip, entry_set)?;
        }
        for time_line_part in &self.time_line_parts {
            time_line_part.save_zip(&child_parent_path, zip, entry_set)?;
        }
        for worksheet_threaded_comments_part in &self.worksheet_threaded_comments_parts {
            worksheet_threaded_comments_part
                .save_zip(&child_parent_path, zip, entry_set)?;
        }
        for model3_d_reference_relationship_part in &self
            .model3_d_reference_relationship_parts
        {
            model3_d_reference_relationship_part
                .save_zip(&child_parent_path, zip, entry_set)?;
        }
        for named_sheet_views_part in &self.named_sheet_views_parts {
            named_sheet_views_part.save_zip(&child_parent_path, zip, entry_set)?;
        }
        Ok(())
    }
}
