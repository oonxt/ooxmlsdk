use hard_xml::{XmlRead, XmlWrite};
pub const RELATIONSHIP_TYPE: &str = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/glossaryDocument";
#[derive(Clone, Debug, Default)]
pub struct GlossaryDocumentPart {
    pub r_id: String,
    pub relationships: Option<crate::packages::opc_relationships::Relationships>,
    pub rels_path: String,
    pub inner_path: String,
    pub root_element: crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::GlossaryDocument,
    pub wordprocessing_comments_part: Option<
        crate::parts::wordprocessing_comments_part::WordprocessingCommentsPart,
    >,
    pub document_settings_part: Option<
        crate::parts::document_settings_part::DocumentSettingsPart,
    >,
    pub endnotes_part: Option<crate::parts::endnotes_part::EndnotesPart>,
    pub font_table_part: Option<crate::parts::font_table_part::FontTablePart>,
    pub footnotes_part: Option<crate::parts::footnotes_part::FootnotesPart>,
    pub numbering_definitions_part: Option<
        crate::parts::numbering_definitions_part::NumberingDefinitionsPart,
    >,
    pub style_definitions_part: Option<
        crate::parts::style_definitions_part::StyleDefinitionsPart,
    >,
    pub styles_with_effects_part: Option<
        crate::parts::styles_with_effects_part::StylesWithEffectsPart,
    >,
    pub web_settings_part: Option<crate::parts::web_settings_part::WebSettingsPart>,
    pub footer_parts: Vec<crate::parts::footer_part::FooterPart>,
    pub header_parts: Vec<crate::parts::header_part::HeaderPart>,
    pub wordprocessing_printer_settings_parts: Vec<
        crate::parts::wordprocessing_printer_settings_part::WordprocessingPrinterSettingsPart,
    >,
    pub customization_part: Option<crate::parts::customization_part::CustomizationPart>,
    pub vba_project_part: Option<crate::parts::vba_project_part::VbaProjectPart>,
    pub wordprocessing_comments_ex_part: Option<
        crate::parts::wordprocessing_comments_ex_part::WordprocessingCommentsExPart,
    >,
    pub wordprocessing_people_part: Option<
        crate::parts::wordprocessing_people_part::WordprocessingPeoplePart,
    >,
    pub wordprocessing_comments_ids_part: Option<
        crate::parts::wordprocessing_comments_ids_part::WordprocessingCommentsIdsPart,
    >,
    pub document_tasks_part: Option<
        crate::parts::document_tasks_part::DocumentTasksPart,
    >,
    pub word_comments_extensible_part: Option<
        crate::parts::word_comments_extensible_part::WordCommentsExtensiblePart,
    >,
    pub alternative_format_import_parts: Vec<
        crate::parts::alternative_format_import_part::AlternativeFormatImportPart,
    >,
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
    pub embedded_control_persistence_parts: Vec<
        crate::parts::embedded_control_persistence_part::EmbeddedControlPersistencePart,
    >,
    pub embedded_object_parts: Vec<
        crate::parts::embedded_object_part::EmbeddedObjectPart,
    >,
    pub embedded_package_parts: Vec<
        crate::parts::embedded_package_part::EmbeddedPackagePart,
    >,
    pub image_parts: Vec<crate::parts::image_part::ImagePart>,
    pub model3_d_reference_relationship_parts: Vec<
        crate::parts::model3_d_reference_relationship_part::Model3DReferenceRelationshipPart,
    >,
}
impl GlossaryDocumentPart {
    #[allow(unused_variables)]
    pub(crate) fn new_from_archive(
        parent_path: &str,
        path: &str,
        r_id: &str,
        file_path_set: &std::collections::HashSet<String>,
        archive: &mut zip::ZipArchive<std::io::BufReader<std::fs::File>>,
    ) -> Result<Self, crate::common::SdkError> {
        let mut rels_path = "".to_string();
        let child_parent_path = format!("{}{}", parent_path, "glossary/");
        let part_target_str = if path.ends_with(".xml") {
            &path[path
                .rfind('/')
                .ok_or_else(|| crate::common::SdkError::CommonError(path.to_string()))?
                + 1..path.len()]
        } else {
            ""
        };
        let glossary_document_part_rels_path = crate::common::resolve_zip_file_path(
            &format!("{}_rels/{}.rels", child_parent_path, part_target_str),
        );
        let relationships = if let Some(file_path) = file_path_set
            .get(&glossary_document_part_rels_path)
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
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::GlossaryDocument::from_str(
                &crate::common::read_string(&mut archive.by_name(path)?)?,
            )?,
        );
        let mut wordprocessing_comments_part: Option<
            crate::parts::wordprocessing_comments_part::WordprocessingCommentsPart,
        > = None;
        let mut document_settings_part: Option<
            crate::parts::document_settings_part::DocumentSettingsPart,
        > = None;
        let mut endnotes_part: Option<crate::parts::endnotes_part::EndnotesPart> = None;
        let mut font_table_part: Option<crate::parts::font_table_part::FontTablePart> = None;
        let mut footnotes_part: Option<crate::parts::footnotes_part::FootnotesPart> = None;
        let mut numbering_definitions_part: Option<
            crate::parts::numbering_definitions_part::NumberingDefinitionsPart,
        > = None;
        let mut style_definitions_part: Option<
            crate::parts::style_definitions_part::StyleDefinitionsPart,
        > = None;
        let mut styles_with_effects_part: Option<
            crate::parts::styles_with_effects_part::StylesWithEffectsPart,
        > = None;
        let mut web_settings_part: Option<
            crate::parts::web_settings_part::WebSettingsPart,
        > = None;
        let mut footer_parts: Vec<crate::parts::footer_part::FooterPart> = vec![];
        let mut header_parts: Vec<crate::parts::header_part::HeaderPart> = vec![];
        let mut wordprocessing_printer_settings_parts: Vec<
            crate::parts::wordprocessing_printer_settings_part::WordprocessingPrinterSettingsPart,
        > = vec![];
        let mut customization_part: Option<
            crate::parts::customization_part::CustomizationPart,
        > = None;
        let mut vba_project_part: Option<
            crate::parts::vba_project_part::VbaProjectPart,
        > = None;
        let mut wordprocessing_comments_ex_part: Option<
            crate::parts::wordprocessing_comments_ex_part::WordprocessingCommentsExPart,
        > = None;
        let mut wordprocessing_people_part: Option<
            crate::parts::wordprocessing_people_part::WordprocessingPeoplePart,
        > = None;
        let mut wordprocessing_comments_ids_part: Option<
            crate::parts::wordprocessing_comments_ids_part::WordprocessingCommentsIdsPart,
        > = None;
        let mut document_tasks_part: Option<
            crate::parts::document_tasks_part::DocumentTasksPart,
        > = None;
        let mut word_comments_extensible_part: Option<
            crate::parts::word_comments_extensible_part::WordCommentsExtensiblePart,
        > = None;
        let mut alternative_format_import_parts: Vec<
            crate::parts::alternative_format_import_part::AlternativeFormatImportPart,
        > = vec![];
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
        let mut embedded_control_persistence_parts: Vec<
            crate::parts::embedded_control_persistence_part::EmbeddedControlPersistencePart,
        > = vec![];
        let mut embedded_object_parts: Vec<
            crate::parts::embedded_object_part::EmbeddedObjectPart,
        > = vec![];
        let mut embedded_package_parts: Vec<
            crate::parts::embedded_package_part::EmbeddedPackagePart,
        > = vec![];
        let mut image_parts: Vec<crate::parts::image_part::ImagePart> = vec![];
        let mut model3_d_reference_relationship_parts: Vec<
            crate::parts::model3_d_reference_relationship_part::Model3DReferenceRelationshipPart,
        > = vec![];
        if let Some(relationships) = &relationships {
            for relationship in &relationships.relationship {
                #[allow(clippy::single_match)]
                match relationship.r#type.as_str() {
                    crate::parts::wordprocessing_comments_part::RELATIONSHIP_TYPE => {
                        let target_path = crate::common::resolve_zip_file_path(
                            &format!("{}{}", child_parent_path, relationship.target),
                        );
                        wordprocessing_comments_part = Some(
                            crate::parts::wordprocessing_comments_part::WordprocessingCommentsPart::new_from_archive(
                                &child_parent_path,
                                &target_path,
                                &relationship.id,
                                file_path_set,
                                archive,
                            )?,
                        );
                    }
                    crate::parts::document_settings_part::RELATIONSHIP_TYPE => {
                        let target_path = crate::common::resolve_zip_file_path(
                            &format!("{}{}", child_parent_path, relationship.target),
                        );
                        document_settings_part = Some(
                            crate::parts::document_settings_part::DocumentSettingsPart::new_from_archive(
                                &child_parent_path,
                                &target_path,
                                &relationship.id,
                                file_path_set,
                                archive,
                            )?,
                        );
                    }
                    crate::parts::endnotes_part::RELATIONSHIP_TYPE => {
                        let target_path = crate::common::resolve_zip_file_path(
                            &format!("{}{}", child_parent_path, relationship.target),
                        );
                        endnotes_part = Some(
                            crate::parts::endnotes_part::EndnotesPart::new_from_archive(
                                &child_parent_path,
                                &target_path,
                                &relationship.id,
                                file_path_set,
                                archive,
                            )?,
                        );
                    }
                    crate::parts::font_table_part::RELATIONSHIP_TYPE => {
                        let target_path = crate::common::resolve_zip_file_path(
                            &format!("{}{}", child_parent_path, relationship.target),
                        );
                        font_table_part = Some(
                            crate::parts::font_table_part::FontTablePart::new_from_archive(
                                &child_parent_path,
                                &target_path,
                                &relationship.id,
                                file_path_set,
                                archive,
                            )?,
                        );
                    }
                    crate::parts::footnotes_part::RELATIONSHIP_TYPE => {
                        let target_path = crate::common::resolve_zip_file_path(
                            &format!("{}{}", child_parent_path, relationship.target),
                        );
                        footnotes_part = Some(
                            crate::parts::footnotes_part::FootnotesPart::new_from_archive(
                                &child_parent_path,
                                &target_path,
                                &relationship.id,
                                file_path_set,
                                archive,
                            )?,
                        );
                    }
                    crate::parts::numbering_definitions_part::RELATIONSHIP_TYPE => {
                        let target_path = crate::common::resolve_zip_file_path(
                            &format!("{}{}", child_parent_path, relationship.target),
                        );
                        numbering_definitions_part = Some(
                            crate::parts::numbering_definitions_part::NumberingDefinitionsPart::new_from_archive(
                                &child_parent_path,
                                &target_path,
                                &relationship.id,
                                file_path_set,
                                archive,
                            )?,
                        );
                    }
                    crate::parts::style_definitions_part::RELATIONSHIP_TYPE => {
                        let target_path = crate::common::resolve_zip_file_path(
                            &format!("{}{}", child_parent_path, relationship.target),
                        );
                        style_definitions_part = Some(
                            crate::parts::style_definitions_part::StyleDefinitionsPart::new_from_archive(
                                &child_parent_path,
                                &target_path,
                                &relationship.id,
                                file_path_set,
                                archive,
                            )?,
                        );
                    }
                    crate::parts::styles_with_effects_part::RELATIONSHIP_TYPE => {
                        let target_path = crate::common::resolve_zip_file_path(
                            &format!("{}{}", child_parent_path, relationship.target),
                        );
                        styles_with_effects_part = Some(
                            crate::parts::styles_with_effects_part::StylesWithEffectsPart::new_from_archive(
                                &child_parent_path,
                                &target_path,
                                &relationship.id,
                                file_path_set,
                                archive,
                            )?,
                        );
                    }
                    crate::parts::web_settings_part::RELATIONSHIP_TYPE => {
                        let target_path = crate::common::resolve_zip_file_path(
                            &format!("{}{}", child_parent_path, relationship.target),
                        );
                        web_settings_part = Some(
                            crate::parts::web_settings_part::WebSettingsPart::new_from_archive(
                                &child_parent_path,
                                &target_path,
                                &relationship.id,
                                file_path_set,
                                archive,
                            )?,
                        );
                    }
                    crate::parts::footer_part::RELATIONSHIP_TYPE => {
                        let target_path = crate::common::resolve_zip_file_path(
                            &format!("{}{}", child_parent_path, relationship.target),
                        );
                        let footer_part = crate::parts::footer_part::FooterPart::new_from_archive(
                            &child_parent_path,
                            &target_path,
                            &relationship.id,
                            file_path_set,
                            archive,
                        )?;
                        footer_parts.push(footer_part);
                    }
                    crate::parts::header_part::RELATIONSHIP_TYPE => {
                        let target_path = crate::common::resolve_zip_file_path(
                            &format!("{}{}", child_parent_path, relationship.target),
                        );
                        let header_part = crate::parts::header_part::HeaderPart::new_from_archive(
                            &child_parent_path,
                            &target_path,
                            &relationship.id,
                            file_path_set,
                            archive,
                        )?;
                        header_parts.push(header_part);
                    }
                    crate::parts::wordprocessing_printer_settings_part::RELATIONSHIP_TYPE => {
                        let target_path = crate::common::resolve_zip_file_path(
                            &format!("{}{}", child_parent_path, relationship.target),
                        );
                        let wordprocessing_printer_settings_part = crate::parts::wordprocessing_printer_settings_part::WordprocessingPrinterSettingsPart::new_from_archive(
                            &child_parent_path,
                            &target_path,
                            &relationship.id,
                            file_path_set,
                            archive,
                        )?;
                        wordprocessing_printer_settings_parts
                            .push(wordprocessing_printer_settings_part);
                    }
                    crate::parts::customization_part::RELATIONSHIP_TYPE => {
                        let target_path = crate::common::resolve_zip_file_path(
                            &format!("{}{}", child_parent_path, relationship.target),
                        );
                        customization_part = Some(
                            crate::parts::customization_part::CustomizationPart::new_from_archive(
                                &child_parent_path,
                                &target_path,
                                &relationship.id,
                                file_path_set,
                                archive,
                            )?,
                        );
                    }
                    crate::parts::vba_project_part::RELATIONSHIP_TYPE => {
                        let target_path = crate::common::resolve_zip_file_path(
                            &format!("{}{}", child_parent_path, relationship.target),
                        );
                        vba_project_part = Some(
                            crate::parts::vba_project_part::VbaProjectPart::new_from_archive(
                                &child_parent_path,
                                &target_path,
                                &relationship.id,
                                file_path_set,
                                archive,
                            )?,
                        );
                    }
                    crate::parts::wordprocessing_comments_ex_part::RELATIONSHIP_TYPE => {
                        let target_path = crate::common::resolve_zip_file_path(
                            &format!("{}{}", child_parent_path, relationship.target),
                        );
                        wordprocessing_comments_ex_part = Some(
                            crate::parts::wordprocessing_comments_ex_part::WordprocessingCommentsExPart::new_from_archive(
                                &child_parent_path,
                                &target_path,
                                &relationship.id,
                                file_path_set,
                                archive,
                            )?,
                        );
                    }
                    crate::parts::wordprocessing_people_part::RELATIONSHIP_TYPE => {
                        let target_path = crate::common::resolve_zip_file_path(
                            &format!("{}{}", child_parent_path, relationship.target),
                        );
                        wordprocessing_people_part = Some(
                            crate::parts::wordprocessing_people_part::WordprocessingPeoplePart::new_from_archive(
                                &child_parent_path,
                                &target_path,
                                &relationship.id,
                                file_path_set,
                                archive,
                            )?,
                        );
                    }
                    crate::parts::wordprocessing_comments_ids_part::RELATIONSHIP_TYPE => {
                        let target_path = crate::common::resolve_zip_file_path(
                            &format!("{}{}", child_parent_path, relationship.target),
                        );
                        wordprocessing_comments_ids_part = Some(
                            crate::parts::wordprocessing_comments_ids_part::WordprocessingCommentsIdsPart::new_from_archive(
                                &child_parent_path,
                                &target_path,
                                &relationship.id,
                                file_path_set,
                                archive,
                            )?,
                        );
                    }
                    crate::parts::document_tasks_part::RELATIONSHIP_TYPE => {
                        let target_path = crate::common::resolve_zip_file_path(
                            &format!("{}{}", child_parent_path, relationship.target),
                        );
                        document_tasks_part = Some(
                            crate::parts::document_tasks_part::DocumentTasksPart::new_from_archive(
                                &child_parent_path,
                                &target_path,
                                &relationship.id,
                                file_path_set,
                                archive,
                            )?,
                        );
                    }
                    crate::parts::word_comments_extensible_part::RELATIONSHIP_TYPE => {
                        let target_path = crate::common::resolve_zip_file_path(
                            &format!("{}{}", child_parent_path, relationship.target),
                        );
                        word_comments_extensible_part = Some(
                            crate::parts::word_comments_extensible_part::WordCommentsExtensiblePart::new_from_archive(
                                &child_parent_path,
                                &target_path,
                                &relationship.id,
                                file_path_set,
                                archive,
                            )?,
                        );
                    }
                    crate::parts::alternative_format_import_part::RELATIONSHIP_TYPE => {
                        let target_path = crate::common::resolve_zip_file_path(
                            &format!("{}{}", child_parent_path, relationship.target),
                        );
                        let alternative_format_import_part = crate::parts::alternative_format_import_part::AlternativeFormatImportPart::new_from_archive(
                            &child_parent_path,
                            &target_path,
                            &relationship.id,
                            file_path_set,
                            archive,
                        )?;
                        alternative_format_import_parts
                            .push(alternative_format_import_part);
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
            wordprocessing_comments_part,
            document_settings_part,
            endnotes_part,
            font_table_part,
            footnotes_part,
            numbering_definitions_part,
            style_definitions_part,
            styles_with_effects_part,
            web_settings_part,
            footer_parts,
            header_parts,
            wordprocessing_printer_settings_parts,
            customization_part,
            vba_project_part,
            wordprocessing_comments_ex_part,
            wordprocessing_people_part,
            wordprocessing_comments_ids_part,
            document_tasks_part,
            word_comments_extensible_part,
            alternative_format_import_parts,
            chart_parts,
            extended_chart_parts,
            diagram_colors_parts,
            diagram_data_parts,
            diagram_persist_layout_parts,
            diagram_layout_definition_parts,
            diagram_style_parts,
            embedded_control_persistence_parts,
            embedded_object_parts,
            embedded_package_parts,
            image_parts,
            model3_d_reference_relationship_parts,
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
        let glossary_document_part_dir_path = crate::common::resolve_zip_file_path(
            &format!("{}{}/", parent_path, "glossary"),
        );
        if !glossary_document_part_dir_path.is_empty()
            && !entry_set.contains(&glossary_document_part_dir_path)
        {
            zip.add_directory(
                &glossary_document_part_dir_path,
                zip::write::SimpleFileOptions::default(),
            )?;
            entry_set.insert(glossary_document_part_dir_path);
        }
        if !entry_set.contains(&self.inner_path) {
            zip.start_file(&self.inner_path, options)?;
            zip.write_all(crate::common::SCHEMA_XML.as_bytes())?;
            zip.write_all(self.root_element.to_string()?.as_bytes())?;
            entry_set.insert(self.inner_path.to_string());
        }
        let child_parent_path = format!("{}{}", parent_path, "glossary/");
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
        if let Some(wordprocessing_comments_part) = &self.wordprocessing_comments_part {
            wordprocessing_comments_part.save_zip(&child_parent_path, zip, entry_set)?;
        }
        if let Some(document_settings_part) = &self.document_settings_part {
            document_settings_part.save_zip(&child_parent_path, zip, entry_set)?;
        }
        if let Some(endnotes_part) = &self.endnotes_part {
            endnotes_part.save_zip(&child_parent_path, zip, entry_set)?;
        }
        if let Some(font_table_part) = &self.font_table_part {
            font_table_part.save_zip(&child_parent_path, zip, entry_set)?;
        }
        if let Some(footnotes_part) = &self.footnotes_part {
            footnotes_part.save_zip(&child_parent_path, zip, entry_set)?;
        }
        if let Some(numbering_definitions_part) = &self.numbering_definitions_part {
            numbering_definitions_part.save_zip(&child_parent_path, zip, entry_set)?;
        }
        if let Some(style_definitions_part) = &self.style_definitions_part {
            style_definitions_part.save_zip(&child_parent_path, zip, entry_set)?;
        }
        if let Some(styles_with_effects_part) = &self.styles_with_effects_part {
            styles_with_effects_part.save_zip(&child_parent_path, zip, entry_set)?;
        }
        if let Some(web_settings_part) = &self.web_settings_part {
            web_settings_part.save_zip(&child_parent_path, zip, entry_set)?;
        }
        for footer_part in &self.footer_parts {
            footer_part.save_zip(&child_parent_path, zip, entry_set)?;
        }
        for header_part in &self.header_parts {
            header_part.save_zip(&child_parent_path, zip, entry_set)?;
        }
        for wordprocessing_printer_settings_part in &self
            .wordprocessing_printer_settings_parts
        {
            wordprocessing_printer_settings_part
                .save_zip(&child_parent_path, zip, entry_set)?;
        }
        if let Some(customization_part) = &self.customization_part {
            customization_part.save_zip(&child_parent_path, zip, entry_set)?;
        }
        if let Some(vba_project_part) = &self.vba_project_part {
            vba_project_part.save_zip(&child_parent_path, zip, entry_set)?;
        }
        if let Some(wordprocessing_comments_ex_part) = &self
            .wordprocessing_comments_ex_part
        {
            wordprocessing_comments_ex_part
                .save_zip(&child_parent_path, zip, entry_set)?;
        }
        if let Some(wordprocessing_people_part) = &self.wordprocessing_people_part {
            wordprocessing_people_part.save_zip(&child_parent_path, zip, entry_set)?;
        }
        if let Some(wordprocessing_comments_ids_part) = &self
            .wordprocessing_comments_ids_part
        {
            wordprocessing_comments_ids_part
                .save_zip(&child_parent_path, zip, entry_set)?;
        }
        if let Some(document_tasks_part) = &self.document_tasks_part {
            document_tasks_part.save_zip(&child_parent_path, zip, entry_set)?;
        }
        if let Some(word_comments_extensible_part) = &self.word_comments_extensible_part
        {
            word_comments_extensible_part.save_zip(&child_parent_path, zip, entry_set)?;
        }
        for alternative_format_import_part in &self.alternative_format_import_parts {
            alternative_format_import_part.save_zip(&child_parent_path, zip, entry_set)?;
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
        for embedded_control_persistence_part in &self.embedded_control_persistence_parts
        {
            embedded_control_persistence_part
                .save_zip(&child_parent_path, zip, entry_set)?;
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
        for model3_d_reference_relationship_part in &self
            .model3_d_reference_relationship_parts
        {
            model3_d_reference_relationship_part
                .save_zip(&child_parent_path, zip, entry_set)?;
        }
        Ok(())
    }
}
