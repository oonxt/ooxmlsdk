use hard_xml::{XmlRead, XmlWrite};
pub const RELATIONSHIP_TYPE: &str = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/slide";
#[derive(Clone, Debug, Default)]
pub struct SlidePart {
    pub r_id: String,
    pub relationships: Option<crate::packages::opc_relationships::Relationships>,
    pub rels_path: String,
    pub inner_path: String,
    pub root_element: crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::Slide,
    pub custom_xml_parts: Vec<crate::parts::custom_xml_part::CustomXmlPart>,
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
    pub embedded_object_parts: Vec<
        crate::parts::embedded_object_part::EmbeddedObjectPart,
    >,
    pub embedded_package_parts: Vec<
        crate::parts::embedded_package_part::EmbeddedPackagePart,
    >,
    pub image_parts: Vec<crate::parts::image_part::ImagePart>,
    pub vml_drawing_parts: Vec<crate::parts::vml_drawing_part::VmlDrawingPart>,
    pub embedded_control_persistence_binary_data_parts: Vec<
        crate::parts::embedded_control_persistence_binary_data_part::EmbeddedControlPersistenceBinaryDataPart,
    >,
    pub model3_d_reference_relationship_parts: Vec<
        crate::parts::model3_d_reference_relationship_part::Model3DReferenceRelationshipPart,
    >,
    pub slide_comments_part: Option<
        crate::parts::slide_comments_part::SlideCommentsPart,
    >,
    pub notes_slide_part: Option<crate::parts::notes_slide_part::NotesSlidePart>,
    pub theme_override_part: Option<
        crate::parts::theme_override_part::ThemeOverridePart,
    >,
    pub slide_layout_part: Option<crate::parts::slide_layout_part::SlideLayoutPart>,
    pub slide_sync_data_part: Option<
        crate::parts::slide_sync_data_part::SlideSyncDataPart,
    >,
    pub user_defined_tags_parts: Vec<
        crate::parts::user_defined_tags_part::UserDefinedTagsPart,
    >,
    pub slide_parts: Vec<crate::parts::slide_part::SlidePart>,
    pub embedded_control_persistence_parts: Vec<
        crate::parts::embedded_control_persistence_part::EmbeddedControlPersistencePart,
    >,
    pub web_extension_parts: Vec<crate::parts::web_extension_part::WebExtensionPart>,
    pub comment_parts: Vec<
        crate::parts::power_point_comment_part::PowerPointCommentPart,
    >,
}
impl SlidePart {
    #[allow(unused_variables)]
    pub(crate) fn new_from_archive(
        parent_path: &str,
        path: &str,
        r_id: &str,
        file_path_set: &std::collections::HashSet<String>,
        archive: &mut zip::ZipArchive<std::io::BufReader<std::fs::File>>,
    ) -> Result<Self, crate::common::SdkError> {
        let mut rels_path = "".to_string();
        let child_parent_path = format!("{}{}", parent_path, "slides/");
        let part_target_str = if path.ends_with(".xml") {
            &path[path
                .rfind('/')
                .ok_or_else(|| crate::common::SdkError::CommonError(path.to_string()))?
                + 1..path.len()]
        } else {
            ""
        };
        let slide_part_rels_path = crate::common::resolve_zip_file_path(
            &format!("{}_rels/{}.rels", child_parent_path, part_target_str),
        );
        let relationships = if let Some(file_path) = file_path_set
            .get(&slide_part_rels_path)
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
            crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::Slide::from_str(
                &crate::common::read_string(&mut archive.by_name(path)?)?,
            )?,
        );
        let mut custom_xml_parts: Vec<crate::parts::custom_xml_part::CustomXmlPart> = vec![];
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
        let mut embedded_object_parts: Vec<
            crate::parts::embedded_object_part::EmbeddedObjectPart,
        > = vec![];
        let mut embedded_package_parts: Vec<
            crate::parts::embedded_package_part::EmbeddedPackagePart,
        > = vec![];
        let mut image_parts: Vec<crate::parts::image_part::ImagePart> = vec![];
        let mut vml_drawing_parts: Vec<crate::parts::vml_drawing_part::VmlDrawingPart> = vec![];
        let mut embedded_control_persistence_binary_data_parts: Vec<
            crate::parts::embedded_control_persistence_binary_data_part::EmbeddedControlPersistenceBinaryDataPart,
        > = vec![];
        let mut model3_d_reference_relationship_parts: Vec<
            crate::parts::model3_d_reference_relationship_part::Model3DReferenceRelationshipPart,
        > = vec![];
        let mut slide_comments_part: Option<
            crate::parts::slide_comments_part::SlideCommentsPart,
        > = None;
        let mut notes_slide_part: Option<
            crate::parts::notes_slide_part::NotesSlidePart,
        > = None;
        let mut theme_override_part: Option<
            crate::parts::theme_override_part::ThemeOverridePart,
        > = None;
        let mut slide_layout_part: Option<
            crate::parts::slide_layout_part::SlideLayoutPart,
        > = None;
        let mut slide_sync_data_part: Option<
            crate::parts::slide_sync_data_part::SlideSyncDataPart,
        > = None;
        let mut user_defined_tags_parts: Vec<
            crate::parts::user_defined_tags_part::UserDefinedTagsPart,
        > = vec![];
        let mut slide_parts: Vec<crate::parts::slide_part::SlidePart> = vec![];
        let mut embedded_control_persistence_parts: Vec<
            crate::parts::embedded_control_persistence_part::EmbeddedControlPersistencePart,
        > = vec![];
        let mut web_extension_parts: Vec<
            crate::parts::web_extension_part::WebExtensionPart,
        > = vec![];
        let mut comment_parts: Vec<
            crate::parts::power_point_comment_part::PowerPointCommentPart,
        > = vec![];
        if let Some(relationships) = &relationships {
            for relationship in &relationships.relationship {
                #[allow(clippy::single_match)]
                match relationship.r#type.as_str() {
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
                    crate::parts::slide_comments_part::RELATIONSHIP_TYPE => {
                        let target_path = crate::common::resolve_zip_file_path(
                            &format!("{}{}", child_parent_path, relationship.target),
                        );
                        slide_comments_part = Some(
                            crate::parts::slide_comments_part::SlideCommentsPart::new_from_archive(
                                &child_parent_path,
                                &target_path,
                                &relationship.id,
                                file_path_set,
                                archive,
                            )?,
                        );
                    }
                    crate::parts::notes_slide_part::RELATIONSHIP_TYPE => {
                        let target_path = crate::common::resolve_zip_file_path(
                            &format!("{}{}", child_parent_path, relationship.target),
                        );
                        notes_slide_part = Some(
                            crate::parts::notes_slide_part::NotesSlidePart::new_from_archive(
                                &child_parent_path,
                                &target_path,
                                &relationship.id,
                                file_path_set,
                                archive,
                            )?,
                        );
                    }
                    crate::parts::theme_override_part::RELATIONSHIP_TYPE => {
                        let target_path = crate::common::resolve_zip_file_path(
                            &format!("{}{}", child_parent_path, relationship.target),
                        );
                        theme_override_part = Some(
                            crate::parts::theme_override_part::ThemeOverridePart::new_from_archive(
                                &child_parent_path,
                                &target_path,
                                &relationship.id,
                                file_path_set,
                                archive,
                            )?,
                        );
                    }
                    crate::parts::slide_layout_part::RELATIONSHIP_TYPE => {
                        let target_path = crate::common::resolve_zip_file_path(
                            &format!("{}{}", child_parent_path, relationship.target),
                        );
                        slide_layout_part = Some(
                            crate::parts::slide_layout_part::SlideLayoutPart::new_from_archive(
                                &child_parent_path,
                                &target_path,
                                &relationship.id,
                                file_path_set,
                                archive,
                            )?,
                        );
                    }
                    crate::parts::slide_sync_data_part::RELATIONSHIP_TYPE => {
                        let target_path = crate::common::resolve_zip_file_path(
                            &format!("{}{}", child_parent_path, relationship.target),
                        );
                        slide_sync_data_part = Some(
                            crate::parts::slide_sync_data_part::SlideSyncDataPart::new_from_archive(
                                &child_parent_path,
                                &target_path,
                                &relationship.id,
                                file_path_set,
                                archive,
                            )?,
                        );
                    }
                    crate::parts::user_defined_tags_part::RELATIONSHIP_TYPE => {
                        let target_path = crate::common::resolve_zip_file_path(
                            &format!("{}{}", child_parent_path, relationship.target),
                        );
                        let user_defined_tags_part = crate::parts::user_defined_tags_part::UserDefinedTagsPart::new_from_archive(
                            &child_parent_path,
                            &target_path,
                            &relationship.id,
                            file_path_set,
                            archive,
                        )?;
                        user_defined_tags_parts.push(user_defined_tags_part);
                    }
                    crate::parts::slide_part::RELATIONSHIP_TYPE => {
                        let target_path = crate::common::resolve_zip_file_path(
                            &format!("{}{}", child_parent_path, relationship.target),
                        );
                        let slide_part = crate::parts::slide_part::SlidePart::new_from_archive(
                            &child_parent_path,
                            &target_path,
                            &relationship.id,
                            file_path_set,
                            archive,
                        )?;
                        slide_parts.push(slide_part);
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
                    crate::parts::power_point_comment_part::RELATIONSHIP_TYPE => {
                        let target_path = crate::common::resolve_zip_file_path(
                            &format!("{}{}", child_parent_path, relationship.target),
                        );
                        let power_point_comment_part = crate::parts::power_point_comment_part::PowerPointCommentPart::new_from_archive(
                            &child_parent_path,
                            &target_path,
                            &relationship.id,
                            file_path_set,
                            archive,
                        )?;
                        comment_parts.push(power_point_comment_part);
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
            custom_xml_parts,
            chart_parts,
            extended_chart_parts,
            diagram_colors_parts,
            diagram_data_parts,
            diagram_persist_layout_parts,
            diagram_layout_definition_parts,
            diagram_style_parts,
            embedded_object_parts,
            embedded_package_parts,
            image_parts,
            vml_drawing_parts,
            embedded_control_persistence_binary_data_parts,
            model3_d_reference_relationship_parts,
            slide_comments_part,
            notes_slide_part,
            theme_override_part,
            slide_layout_part,
            slide_sync_data_part,
            user_defined_tags_parts,
            slide_parts,
            embedded_control_persistence_parts,
            web_extension_parts,
            comment_parts,
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
        let slide_part_dir_path = crate::common::resolve_zip_file_path(
            &format!("{}{}/", parent_path, "slides"),
        );
        if !slide_part_dir_path.is_empty() && !entry_set.contains(&slide_part_dir_path) {
            zip.add_directory(
                &slide_part_dir_path,
                zip::write::SimpleFileOptions::default(),
            )?;
            entry_set.insert(slide_part_dir_path);
        }
        if !entry_set.contains(&self.inner_path) {
            zip.start_file(&self.inner_path, options)?;
            zip.write_all(crate::common::SCHEMA_XML.as_bytes())?;
            zip.write_all(self.root_element.to_string()?.as_bytes())?;
            entry_set.insert(self.inner_path.to_string());
        }
        let child_parent_path = format!("{}{}", parent_path, "slides/");
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
        for custom_xml_part in &self.custom_xml_parts {
            custom_xml_part.save_zip(&child_parent_path, zip, entry_set)?;
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
        for embedded_object_part in &self.embedded_object_parts {
            embedded_object_part.save_zip(&child_parent_path, zip, entry_set)?;
        }
        for embedded_package_part in &self.embedded_package_parts {
            embedded_package_part.save_zip(&child_parent_path, zip, entry_set)?;
        }
        for image_part in &self.image_parts {
            image_part.save_zip(&child_parent_path, zip, entry_set)?;
        }
        for vml_drawing_part in &self.vml_drawing_parts {
            vml_drawing_part.save_zip(&child_parent_path, zip, entry_set)?;
        }
        for embedded_control_persistence_binary_data_part in &self
            .embedded_control_persistence_binary_data_parts
        {
            embedded_control_persistence_binary_data_part
                .save_zip(&child_parent_path, zip, entry_set)?;
        }
        for model3_d_reference_relationship_part in &self
            .model3_d_reference_relationship_parts
        {
            model3_d_reference_relationship_part
                .save_zip(&child_parent_path, zip, entry_set)?;
        }
        if let Some(slide_comments_part) = &self.slide_comments_part {
            slide_comments_part.save_zip(&child_parent_path, zip, entry_set)?;
        }
        if let Some(notes_slide_part) = &self.notes_slide_part {
            notes_slide_part.save_zip(&child_parent_path, zip, entry_set)?;
        }
        if let Some(theme_override_part) = &self.theme_override_part {
            theme_override_part.save_zip(&child_parent_path, zip, entry_set)?;
        }
        if let Some(slide_layout_part) = &self.slide_layout_part {
            slide_layout_part.save_zip(&child_parent_path, zip, entry_set)?;
        }
        if let Some(slide_sync_data_part) = &self.slide_sync_data_part {
            slide_sync_data_part.save_zip(&child_parent_path, zip, entry_set)?;
        }
        for user_defined_tags_part in &self.user_defined_tags_parts {
            user_defined_tags_part.save_zip(&child_parent_path, zip, entry_set)?;
        }
        for slide_part in &self.slide_parts {
            slide_part.save_zip(&child_parent_path, zip, entry_set)?;
        }
        for embedded_control_persistence_part in &self.embedded_control_persistence_parts
        {
            embedded_control_persistence_part
                .save_zip(&child_parent_path, zip, entry_set)?;
        }
        for web_extension_part in &self.web_extension_parts {
            web_extension_part.save_zip(&child_parent_path, zip, entry_set)?;
        }
        for power_point_comment_part in &self.comment_parts {
            power_point_comment_part.save_zip(&child_parent_path, zip, entry_set)?;
        }
        Ok(())
    }
}
