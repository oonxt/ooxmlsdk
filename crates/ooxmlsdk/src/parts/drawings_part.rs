use hard_xml::{XmlRead, XmlWrite};
pub const RELATIONSHIP_TYPE: &str = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/drawing";
#[derive(Clone, Debug, Default)]
pub struct DrawingsPart {
    pub r_id: String,
    pub relationships: Option<crate::packages::opc_relationships::Relationships>,
    pub rels_path: String,
    pub inner_path: String,
    pub root_element: crate::schemas::schemas_openxmlformats_org_drawingml_2006_spreadsheet_drawing::WorksheetDrawing,
    pub chart_parts: Vec<crate::parts::chart_part::ChartPart>,
    pub extended_chart_parts: Vec<crate::parts::extended_chart_part::ExtendedChartPart>,
    pub diagram_colors_parts: Vec<crate::parts::diagram_colors_part::DiagramColorsPart>,
    pub diagram_data_parts: Vec<crate::parts::diagram_data_part::DiagramDataPart>,
    pub diagram_persist_layout_parts: Vec<
        crate::parts::diagram_persist_layout_part::DiagramPersistLayoutPart,
    >,
    pub diagram_layout_definition_parts: Vec<
        crate::parts::diagram_layout_definition_part::DiagramLayoutDefinitionPart,
    >,
    pub diagram_style_parts: Vec<crate::parts::diagram_style_part::DiagramStylePart>,
    pub image_parts: Vec<crate::parts::image_part::ImagePart>,
    pub custom_xml_parts: Vec<crate::parts::custom_xml_part::CustomXmlPart>,
    pub web_extension_parts: Vec<crate::parts::web_extension_part::WebExtensionPart>,
}
impl DrawingsPart {
    #[allow(unused_variables)]
    pub(crate) fn new_from_archive(
        parent_path: &str,
        path: &str,
        r_id: &str,
        file_path_set: &std::collections::HashSet<String>,
        archive: &mut zip::ZipArchive<std::io::BufReader<std::fs::File>>,
    ) -> Result<Self, crate::common::SdkError> {
        let mut rels_path = "".to_string();
        let child_parent_path = format!("{}{}", parent_path, "../drawings/");
        let part_target_str = if path.ends_with(".xml") {
            &path[path
                .rfind('/')
                .ok_or_else(|| crate::common::SdkError::CommonError(path.to_string()))?
                + 1..path.len()]
        } else {
            ""
        };
        let drawings_part_rels_path = crate::common::resolve_zip_file_path(
            &format!("{}_rels/{}.rels", child_parent_path, part_target_str),
        );
        let relationships = if let Some(file_path) = file_path_set
            .get(&drawings_part_rels_path)
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
            crate::schemas::schemas_openxmlformats_org_drawingml_2006_spreadsheet_drawing::WorksheetDrawing::from_str(
                &crate::common::read_string(&mut archive.by_name(path)?)?,
            )?,
        );
        let mut chart_parts: Vec<crate::parts::chart_part::ChartPart> = vec![];
        let mut extended_chart_parts: Vec<
            crate::parts::extended_chart_part::ExtendedChartPart,
        > = vec![];
        let mut diagram_colors_parts: Vec<
            crate::parts::diagram_colors_part::DiagramColorsPart,
        > = vec![];
        let mut diagram_data_parts: Vec<
            crate::parts::diagram_data_part::DiagramDataPart,
        > = vec![];
        let mut diagram_persist_layout_parts: Vec<
            crate::parts::diagram_persist_layout_part::DiagramPersistLayoutPart,
        > = vec![];
        let mut diagram_layout_definition_parts: Vec<
            crate::parts::diagram_layout_definition_part::DiagramLayoutDefinitionPart,
        > = vec![];
        let mut diagram_style_parts: Vec<
            crate::parts::diagram_style_part::DiagramStylePart,
        > = vec![];
        let mut image_parts: Vec<crate::parts::image_part::ImagePart> = vec![];
        let mut custom_xml_parts: Vec<crate::parts::custom_xml_part::CustomXmlPart> = vec![];
        let mut web_extension_parts: Vec<
            crate::parts::web_extension_part::WebExtensionPart,
        > = vec![];
        if let Some(relationships) = &relationships {
            for relationship in &relationships.relationship {
                #[allow(clippy::single_match)]
                match relationship.r#type.as_str() {
                    crate::parts::chart_part::RELATIONSHIP_TYPE => {
                        let target_path = crate::common::resolve_zip_file_path(
                            &format!("{}{}", child_parent_path, relationship.target),
                        );
                        let chart_part = crate::parts::chart_part::ChartPart::new_from_archive(
                            &child_parent_path,
                            &target_path,
                            &relationship.id,
                            file_path_set,
                            archive,
                        )?;
                        chart_parts.push(chart_part);
                    }
                    crate::parts::extended_chart_part::RELATIONSHIP_TYPE => {
                        let target_path = crate::common::resolve_zip_file_path(
                            &format!("{}{}", child_parent_path, relationship.target),
                        );
                        let extended_chart_part = crate::parts::extended_chart_part::ExtendedChartPart::new_from_archive(
                            &child_parent_path,
                            &target_path,
                            &relationship.id,
                            file_path_set,
                            archive,
                        )?;
                        extended_chart_parts.push(extended_chart_part);
                    }
                    crate::parts::diagram_colors_part::RELATIONSHIP_TYPE => {
                        let target_path = crate::common::resolve_zip_file_path(
                            &format!("{}{}", child_parent_path, relationship.target),
                        );
                        let diagram_colors_part = crate::parts::diagram_colors_part::DiagramColorsPart::new_from_archive(
                            &child_parent_path,
                            &target_path,
                            &relationship.id,
                            file_path_set,
                            archive,
                        )?;
                        diagram_colors_parts.push(diagram_colors_part);
                    }
                    crate::parts::diagram_data_part::RELATIONSHIP_TYPE => {
                        let target_path = crate::common::resolve_zip_file_path(
                            &format!("{}{}", child_parent_path, relationship.target),
                        );
                        let diagram_data_part = crate::parts::diagram_data_part::DiagramDataPart::new_from_archive(
                            &child_parent_path,
                            &target_path,
                            &relationship.id,
                            file_path_set,
                            archive,
                        )?;
                        diagram_data_parts.push(diagram_data_part);
                    }
                    crate::parts::diagram_persist_layout_part::RELATIONSHIP_TYPE => {
                        let target_path = crate::common::resolve_zip_file_path(
                            &format!("{}{}", child_parent_path, relationship.target),
                        );
                        let diagram_persist_layout_part = crate::parts::diagram_persist_layout_part::DiagramPersistLayoutPart::new_from_archive(
                            &child_parent_path,
                            &target_path,
                            &relationship.id,
                            file_path_set,
                            archive,
                        )?;
                        diagram_persist_layout_parts.push(diagram_persist_layout_part);
                    }
                    crate::parts::diagram_layout_definition_part::RELATIONSHIP_TYPE => {
                        let target_path = crate::common::resolve_zip_file_path(
                            &format!("{}{}", child_parent_path, relationship.target),
                        );
                        let diagram_layout_definition_part = crate::parts::diagram_layout_definition_part::DiagramLayoutDefinitionPart::new_from_archive(
                            &child_parent_path,
                            &target_path,
                            &relationship.id,
                            file_path_set,
                            archive,
                        )?;
                        diagram_layout_definition_parts
                            .push(diagram_layout_definition_part);
                    }
                    crate::parts::diagram_style_part::RELATIONSHIP_TYPE => {
                        let target_path = crate::common::resolve_zip_file_path(
                            &format!("{}{}", child_parent_path, relationship.target),
                        );
                        let diagram_style_part = crate::parts::diagram_style_part::DiagramStylePart::new_from_archive(
                            &child_parent_path,
                            &target_path,
                            &relationship.id,
                            file_path_set,
                            archive,
                        )?;
                        diagram_style_parts.push(diagram_style_part);
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
                    crate::parts::custom_xml_part::RELATIONSHIP_TYPE => {
                        let target_path = crate::common::resolve_zip_file_path(
                            &format!("{}{}", child_parent_path, relationship.target),
                        );
                        let custom_xml_part = crate::parts::custom_xml_part::CustomXmlPart::new_from_archive(
                            &child_parent_path,
                            &target_path,
                            &relationship.id,
                            file_path_set,
                            archive,
                        )?;
                        custom_xml_parts.push(custom_xml_part);
                    }
                    crate::parts::web_extension_part::RELATIONSHIP_TYPE => {
                        let target_path = crate::common::resolve_zip_file_path(
                            &format!("{}{}", child_parent_path, relationship.target),
                        );
                        let web_extension_part = crate::parts::web_extension_part::WebExtensionPart::new_from_archive(
                            &child_parent_path,
                            &target_path,
                            &relationship.id,
                            file_path_set,
                            archive,
                        )?;
                        web_extension_parts.push(web_extension_part);
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
            chart_parts,
            extended_chart_parts,
            diagram_colors_parts,
            diagram_data_parts,
            diagram_persist_layout_parts,
            diagram_layout_definition_parts,
            diagram_style_parts,
            image_parts,
            custom_xml_parts,
            web_extension_parts,
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
        let drawings_part_dir_path = crate::common::resolve_zip_file_path(
            &format!("{}{}/", parent_path, "../drawings"),
        );
        if !drawings_part_dir_path.is_empty()
            && !entry_set.contains(&drawings_part_dir_path)
        {
            zip.add_directory(
                &drawings_part_dir_path,
                zip::write::SimpleFileOptions::default(),
            )?;
            entry_set.insert(drawings_part_dir_path);
        }
        if !entry_set.contains(&self.inner_path) {
            zip.start_file(&self.inner_path, options)?;
            zip.write_all(crate::common::SCHEMA_XML.as_bytes())?;
            zip.write_all(self.root_element.to_string()?.as_bytes())?;
            entry_set.insert(self.inner_path.to_string());
        }
        let child_parent_path = format!("{}{}", parent_path, "../drawings/");
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
        for chart_part in &self.chart_parts {
            chart_part.save_zip(&child_parent_path, zip, entry_set)?;
        }
        for extended_chart_part in &self.extended_chart_parts {
            extended_chart_part.save_zip(&child_parent_path, zip, entry_set)?;
        }
        for diagram_colors_part in &self.diagram_colors_parts {
            diagram_colors_part.save_zip(&child_parent_path, zip, entry_set)?;
        }
        for diagram_data_part in &self.diagram_data_parts {
            diagram_data_part.save_zip(&child_parent_path, zip, entry_set)?;
        }
        for diagram_persist_layout_part in &self.diagram_persist_layout_parts {
            diagram_persist_layout_part.save_zip(&child_parent_path, zip, entry_set)?;
        }
        for diagram_layout_definition_part in &self.diagram_layout_definition_parts {
            diagram_layout_definition_part.save_zip(&child_parent_path, zip, entry_set)?;
        }
        for diagram_style_part in &self.diagram_style_parts {
            diagram_style_part.save_zip(&child_parent_path, zip, entry_set)?;
        }
        for image_part in &self.image_parts {
            image_part.save_zip(&child_parent_path, zip, entry_set)?;
        }
        for custom_xml_part in &self.custom_xml_parts {
            custom_xml_part.save_zip(&child_parent_path, zip, entry_set)?;
        }
        for web_extension_part in &self.web_extension_parts {
            web_extension_part.save_zip(&child_parent_path, zip, entry_set)?;
        }
        Ok(())
    }
}
