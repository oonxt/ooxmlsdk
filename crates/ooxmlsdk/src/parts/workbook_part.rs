use hard_xml::{XmlRead, XmlWrite};
pub const RELATIONSHIP_TYPE: &str = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument";
#[derive(Clone, Debug, Default)]
pub struct WorkbookPart {
    pub r_id: String,
    pub relationships: Option<crate::packages::opc_relationships::Relationships>,
    pub rels_path: String,
    pub inner_path: String,
    pub root_element: crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Workbook,
    pub custom_xml_parts: Vec<crate::parts::custom_xml_part::CustomXmlPart>,
    pub calculation_chain_part: Option<
        crate::parts::calculation_chain_part::CalculationChainPart,
    >,
    pub cell_metadata_part: Option<crate::parts::cell_metadata_part::CellMetadataPart>,
    pub connections_part: Option<crate::parts::connections_part::ConnectionsPart>,
    pub custom_xml_mappings_part: Option<
        crate::parts::custom_xml_mappings_part::CustomXmlMappingsPart,
    >,
    pub shared_string_table_part: Option<
        crate::parts::shared_string_table_part::SharedStringTablePart,
    >,
    pub workbook_revision_header_part: Option<
        crate::parts::workbook_revision_header_part::WorkbookRevisionHeaderPart,
    >,
    pub workbook_user_data_part: Option<
        crate::parts::workbook_user_data_part::WorkbookUserDataPart,
    >,
    pub workbook_styles_part: Option<
        crate::parts::workbook_styles_part::WorkbookStylesPart,
    >,
    pub theme_part: Option<crate::parts::theme_part::ThemePart>,
    pub thumbnail_part: Option<crate::parts::thumbnail_part::ThumbnailPart>,
    pub volatile_dependencies_part: Option<
        crate::parts::volatile_dependencies_part::VolatileDependenciesPart,
    >,
    pub chartsheet_parts: Vec<crate::parts::chartsheet_part::ChartsheetPart>,
    pub dialogsheet_parts: Vec<crate::parts::dialogsheet_part::DialogsheetPart>,
    pub external_workbook_parts: Vec<
        crate::parts::external_workbook_part::ExternalWorkbookPart,
    >,
    pub pivot_table_cache_definition_parts: Vec<
        crate::parts::pivot_table_cache_definition_part::PivotTableCacheDefinitionPart,
    >,
    pub worksheet_parts: Vec<crate::parts::worksheet_part::WorksheetPart>,
    pub excel_attached_toolbars_part: Option<
        crate::parts::excel_attached_toolbars_part::ExcelAttachedToolbarsPart,
    >,
    pub vba_project_part: Option<crate::parts::vba_project_part::VbaProjectPart>,
    pub macro_sheet_parts: Vec<crate::parts::macro_sheet_part::MacroSheetPart>,
    pub international_macro_sheet_parts: Vec<
        crate::parts::international_macro_sheet_part::InternationalMacroSheetPart,
    >,
    pub custom_data_properties_parts: Vec<
        crate::parts::custom_data_properties_part::CustomDataPropertiesPart,
    >,
    pub slicer_cache_parts: Vec<crate::parts::slicer_cache_part::SlicerCachePart>,
    pub time_line_cache_parts: Vec<
        crate::parts::time_line_cache_part::TimeLineCachePart,
    >,
    pub workbook_person_parts: Vec<
        crate::parts::workbook_person_part::WorkbookPersonPart,
    >,
    pub rd_rich_value_parts: Vec<crate::parts::rd_rich_value_part::RdRichValuePart>,
    pub ct_rd_rich_value_structure_parts: Vec<
        crate::parts::rd_rich_value_structure_part::RdRichValueStructurePart,
    >,
    pub rd_array_parts: Vec<crate::parts::rd_array_part::RdArrayPart>,
    pub rich_styles_parts: Vec<crate::parts::rich_styles_part::RichStylesPart>,
    pub rd_supporting_property_bag_parts: Vec<
        crate::parts::rd_supporting_property_bag_part::RdSupportingPropertyBagPart,
    >,
    pub rd_supporting_property_bag_structure_parts: Vec<
        crate::parts::rd_supporting_property_bag_structure_part::RdSupportingPropertyBagStructurePart,
    >,
    pub rd_rich_value_types_parts: Vec<
        crate::parts::rd_rich_value_types_part::RdRichValueTypesPart,
    >,
    pub rd_rich_value_web_image_part: Option<
        crate::parts::rd_rich_value_web_image_part::RdRichValueWebImagePart,
    >,
    pub feature_property_bags_part: Option<
        crate::parts::feature_property_bags_part::FeaturePropertyBagsPart,
    >,
}
impl WorkbookPart {
    #[allow(unused_variables)]
    pub(crate) fn new_from_archive(
        parent_path: &str,
        path: &str,
        r_id: &str,
        file_path_set: &std::collections::HashSet<String>,
        archive: &mut zip::ZipArchive<std::io::BufReader<std::fs::File>>,
    ) -> Result<Self, crate::common::SdkError> {
        let mut rels_path = "".to_string();
        let child_parent_path = format!("{}{}", parent_path, "xl/");
        let part_target_str = if path.ends_with(".xml") {
            &path[path
                .rfind('/')
                .ok_or_else(|| crate::common::SdkError::CommonError(path.to_string()))?
                + 1..path.len()]
        } else {
            ""
        };
        let workbook_part_rels_path = crate::common::resolve_zip_file_path(
            &format!("{}_rels/{}.rels", child_parent_path, part_target_str),
        );
        let relationships = if let Some(file_path) = file_path_set
            .get(&workbook_part_rels_path)
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
            crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Workbook::from_str(
                &crate::common::read_string(&mut archive.by_name(path)?)?,
            )?,
        );
        let mut custom_xml_parts: Vec<crate::parts::custom_xml_part::CustomXmlPart> = vec![];
        let mut calculation_chain_part: Option<
            crate::parts::calculation_chain_part::CalculationChainPart,
        > = None;
        let mut cell_metadata_part: Option<
            crate::parts::cell_metadata_part::CellMetadataPart,
        > = None;
        let mut connections_part: Option<
            crate::parts::connections_part::ConnectionsPart,
        > = None;
        let mut custom_xml_mappings_part: Option<
            crate::parts::custom_xml_mappings_part::CustomXmlMappingsPart,
        > = None;
        let mut shared_string_table_part: Option<
            crate::parts::shared_string_table_part::SharedStringTablePart,
        > = None;
        let mut workbook_revision_header_part: Option<
            crate::parts::workbook_revision_header_part::WorkbookRevisionHeaderPart,
        > = None;
        let mut workbook_user_data_part: Option<
            crate::parts::workbook_user_data_part::WorkbookUserDataPart,
        > = None;
        let mut workbook_styles_part: Option<
            crate::parts::workbook_styles_part::WorkbookStylesPart,
        > = None;
        let mut theme_part: Option<crate::parts::theme_part::ThemePart> = None;
        let mut thumbnail_part: Option<crate::parts::thumbnail_part::ThumbnailPart> = None;
        let mut volatile_dependencies_part: Option<
            crate::parts::volatile_dependencies_part::VolatileDependenciesPart,
        > = None;
        let mut chartsheet_parts: Vec<crate::parts::chartsheet_part::ChartsheetPart> = vec![];
        let mut dialogsheet_parts: Vec<
            crate::parts::dialogsheet_part::DialogsheetPart,
        > = vec![];
        let mut external_workbook_parts: Vec<
            crate::parts::external_workbook_part::ExternalWorkbookPart,
        > = vec![];
        let mut pivot_table_cache_definition_parts: Vec<
            crate::parts::pivot_table_cache_definition_part::PivotTableCacheDefinitionPart,
        > = vec![];
        let mut worksheet_parts: Vec<crate::parts::worksheet_part::WorksheetPart> = vec![];
        let mut excel_attached_toolbars_part: Option<
            crate::parts::excel_attached_toolbars_part::ExcelAttachedToolbarsPart,
        > = None;
        let mut vba_project_part: Option<
            crate::parts::vba_project_part::VbaProjectPart,
        > = None;
        let mut macro_sheet_parts: Vec<crate::parts::macro_sheet_part::MacroSheetPart> = vec![];
        let mut international_macro_sheet_parts: Vec<
            crate::parts::international_macro_sheet_part::InternationalMacroSheetPart,
        > = vec![];
        let mut custom_data_properties_parts: Vec<
            crate::parts::custom_data_properties_part::CustomDataPropertiesPart,
        > = vec![];
        let mut slicer_cache_parts: Vec<
            crate::parts::slicer_cache_part::SlicerCachePart,
        > = vec![];
        let mut time_line_cache_parts: Vec<
            crate::parts::time_line_cache_part::TimeLineCachePart,
        > = vec![];
        let mut workbook_person_parts: Vec<
            crate::parts::workbook_person_part::WorkbookPersonPart,
        > = vec![];
        let mut rd_rich_value_parts: Vec<
            crate::parts::rd_rich_value_part::RdRichValuePart,
        > = vec![];
        let mut ct_rd_rich_value_structure_parts: Vec<
            crate::parts::rd_rich_value_structure_part::RdRichValueStructurePart,
        > = vec![];
        let mut rd_array_parts: Vec<crate::parts::rd_array_part::RdArrayPart> = vec![];
        let mut rich_styles_parts: Vec<crate::parts::rich_styles_part::RichStylesPart> = vec![];
        let mut rd_supporting_property_bag_parts: Vec<
            crate::parts::rd_supporting_property_bag_part::RdSupportingPropertyBagPart,
        > = vec![];
        let mut rd_supporting_property_bag_structure_parts: Vec<
            crate::parts::rd_supporting_property_bag_structure_part::RdSupportingPropertyBagStructurePart,
        > = vec![];
        let mut rd_rich_value_types_parts: Vec<
            crate::parts::rd_rich_value_types_part::RdRichValueTypesPart,
        > = vec![];
        let mut rd_rich_value_web_image_part: Option<
            crate::parts::rd_rich_value_web_image_part::RdRichValueWebImagePart,
        > = None;
        let mut feature_property_bags_part: Option<
            crate::parts::feature_property_bags_part::FeaturePropertyBagsPart,
        > = None;
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
                    crate::parts::calculation_chain_part::RELATIONSHIP_TYPE => {
                        let target_path = crate::common::resolve_zip_file_path(
                            &format!("{}{}", child_parent_path, relationship.target),
                        );
                        calculation_chain_part = Some(
                            crate::parts::calculation_chain_part::CalculationChainPart::new_from_archive(
                                &child_parent_path,
                                &target_path,
                                &relationship.id,
                                file_path_set,
                                archive,
                            )?,
                        );
                    }
                    crate::parts::cell_metadata_part::RELATIONSHIP_TYPE => {
                        let target_path = crate::common::resolve_zip_file_path(
                            &format!("{}{}", child_parent_path, relationship.target),
                        );
                        cell_metadata_part = Some(
                            crate::parts::cell_metadata_part::CellMetadataPart::new_from_archive(
                                &child_parent_path,
                                &target_path,
                                &relationship.id,
                                file_path_set,
                                archive,
                            )?,
                        );
                    }
                    crate::parts::connections_part::RELATIONSHIP_TYPE => {
                        let target_path = crate::common::resolve_zip_file_path(
                            &format!("{}{}", child_parent_path, relationship.target),
                        );
                        connections_part = Some(
                            crate::parts::connections_part::ConnectionsPart::new_from_archive(
                                &child_parent_path,
                                &target_path,
                                &relationship.id,
                                file_path_set,
                                archive,
                            )?,
                        );
                    }
                    crate::parts::custom_xml_mappings_part::RELATIONSHIP_TYPE => {
                        let target_path = crate::common::resolve_zip_file_path(
                            &format!("{}{}", child_parent_path, relationship.target),
                        );
                        custom_xml_mappings_part = Some(
                            crate::parts::custom_xml_mappings_part::CustomXmlMappingsPart::new_from_archive(
                                &child_parent_path,
                                &target_path,
                                &relationship.id,
                                file_path_set,
                                archive,
                            )?,
                        );
                    }
                    crate::parts::shared_string_table_part::RELATIONSHIP_TYPE => {
                        let target_path = crate::common::resolve_zip_file_path(
                            &format!("{}{}", child_parent_path, relationship.target),
                        );
                        shared_string_table_part = Some(
                            crate::parts::shared_string_table_part::SharedStringTablePart::new_from_archive(
                                &child_parent_path,
                                &target_path,
                                &relationship.id,
                                file_path_set,
                                archive,
                            )?,
                        );
                    }
                    crate::parts::workbook_revision_header_part::RELATIONSHIP_TYPE => {
                        let target_path = crate::common::resolve_zip_file_path(
                            &format!("{}{}", child_parent_path, relationship.target),
                        );
                        workbook_revision_header_part = Some(
                            crate::parts::workbook_revision_header_part::WorkbookRevisionHeaderPart::new_from_archive(
                                &child_parent_path,
                                &target_path,
                                &relationship.id,
                                file_path_set,
                                archive,
                            )?,
                        );
                    }
                    crate::parts::workbook_user_data_part::RELATIONSHIP_TYPE => {
                        let target_path = crate::common::resolve_zip_file_path(
                            &format!("{}{}", child_parent_path, relationship.target),
                        );
                        workbook_user_data_part = Some(
                            crate::parts::workbook_user_data_part::WorkbookUserDataPart::new_from_archive(
                                &child_parent_path,
                                &target_path,
                                &relationship.id,
                                file_path_set,
                                archive,
                            )?,
                        );
                    }
                    crate::parts::workbook_styles_part::RELATIONSHIP_TYPE => {
                        let target_path = crate::common::resolve_zip_file_path(
                            &format!("{}{}", child_parent_path, relationship.target),
                        );
                        workbook_styles_part = Some(
                            crate::parts::workbook_styles_part::WorkbookStylesPart::new_from_archive(
                                &child_parent_path,
                                &target_path,
                                &relationship.id,
                                file_path_set,
                                archive,
                            )?,
                        );
                    }
                    crate::parts::theme_part::RELATIONSHIP_TYPE => {
                        let target_path = crate::common::resolve_zip_file_path(
                            &format!("{}{}", child_parent_path, relationship.target),
                        );
                        theme_part = Some(
                            crate::parts::theme_part::ThemePart::new_from_archive(
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
                    crate::parts::volatile_dependencies_part::RELATIONSHIP_TYPE => {
                        let target_path = crate::common::resolve_zip_file_path(
                            &format!("{}{}", child_parent_path, relationship.target),
                        );
                        volatile_dependencies_part = Some(
                            crate::parts::volatile_dependencies_part::VolatileDependenciesPart::new_from_archive(
                                &child_parent_path,
                                &target_path,
                                &relationship.id,
                                file_path_set,
                                archive,
                            )?,
                        );
                    }
                    crate::parts::chartsheet_part::RELATIONSHIP_TYPE => {
                        let target_path = crate::common::resolve_zip_file_path(
                            &format!("{}{}", child_parent_path, relationship.target),
                        );
                        let chartsheet_part = crate::parts::chartsheet_part::ChartsheetPart::new_from_archive(
                            &child_parent_path,
                            &target_path,
                            &relationship.id,
                            file_path_set,
                            archive,
                        )?;
                        chartsheet_parts.push(chartsheet_part);
                    }
                    crate::parts::dialogsheet_part::RELATIONSHIP_TYPE => {
                        let target_path = crate::common::resolve_zip_file_path(
                            &format!("{}{}", child_parent_path, relationship.target),
                        );
                        let dialogsheet_part = crate::parts::dialogsheet_part::DialogsheetPart::new_from_archive(
                            &child_parent_path,
                            &target_path,
                            &relationship.id,
                            file_path_set,
                            archive,
                        )?;
                        dialogsheet_parts.push(dialogsheet_part);
                    }
                    crate::parts::external_workbook_part::RELATIONSHIP_TYPE => {
                        let target_path = crate::common::resolve_zip_file_path(
                            &format!("{}{}", child_parent_path, relationship.target),
                        );
                        let external_workbook_part = crate::parts::external_workbook_part::ExternalWorkbookPart::new_from_archive(
                            &child_parent_path,
                            &target_path,
                            &relationship.id,
                            file_path_set,
                            archive,
                        )?;
                        external_workbook_parts.push(external_workbook_part);
                    }
                    crate::parts::pivot_table_cache_definition_part::RELATIONSHIP_TYPE => {
                        let target_path = crate::common::resolve_zip_file_path(
                            &format!("{}{}", child_parent_path, relationship.target),
                        );
                        let pivot_table_cache_definition_part = crate::parts::pivot_table_cache_definition_part::PivotTableCacheDefinitionPart::new_from_archive(
                            &child_parent_path,
                            &target_path,
                            &relationship.id,
                            file_path_set,
                            archive,
                        )?;
                        pivot_table_cache_definition_parts
                            .push(pivot_table_cache_definition_part);
                    }
                    crate::parts::worksheet_part::RELATIONSHIP_TYPE => {
                        let target_path = crate::common::resolve_zip_file_path(
                            &format!("{}{}", child_parent_path, relationship.target),
                        );
                        let worksheet_part = crate::parts::worksheet_part::WorksheetPart::new_from_archive(
                            &child_parent_path,
                            &target_path,
                            &relationship.id,
                            file_path_set,
                            archive,
                        )?;
                        worksheet_parts.push(worksheet_part);
                    }
                    crate::parts::excel_attached_toolbars_part::RELATIONSHIP_TYPE => {
                        let target_path = crate::common::resolve_zip_file_path(
                            &format!("{}{}", child_parent_path, relationship.target),
                        );
                        excel_attached_toolbars_part = Some(
                            crate::parts::excel_attached_toolbars_part::ExcelAttachedToolbarsPart::new_from_archive(
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
                    crate::parts::macro_sheet_part::RELATIONSHIP_TYPE => {
                        let target_path = crate::common::resolve_zip_file_path(
                            &format!("{}{}", child_parent_path, relationship.target),
                        );
                        let macro_sheet_part = crate::parts::macro_sheet_part::MacroSheetPart::new_from_archive(
                            &child_parent_path,
                            &target_path,
                            &relationship.id,
                            file_path_set,
                            archive,
                        )?;
                        macro_sheet_parts.push(macro_sheet_part);
                    }
                    crate::parts::international_macro_sheet_part::RELATIONSHIP_TYPE => {
                        let target_path = crate::common::resolve_zip_file_path(
                            &format!("{}{}", child_parent_path, relationship.target),
                        );
                        let international_macro_sheet_part = crate::parts::international_macro_sheet_part::InternationalMacroSheetPart::new_from_archive(
                            &child_parent_path,
                            &target_path,
                            &relationship.id,
                            file_path_set,
                            archive,
                        )?;
                        international_macro_sheet_parts
                            .push(international_macro_sheet_part);
                    }
                    crate::parts::custom_data_properties_part::RELATIONSHIP_TYPE => {
                        let target_path = crate::common::resolve_zip_file_path(
                            &format!("{}{}", child_parent_path, relationship.target),
                        );
                        let custom_data_properties_part = crate::parts::custom_data_properties_part::CustomDataPropertiesPart::new_from_archive(
                            &child_parent_path,
                            &target_path,
                            &relationship.id,
                            file_path_set,
                            archive,
                        )?;
                        custom_data_properties_parts.push(custom_data_properties_part);
                    }
                    crate::parts::slicer_cache_part::RELATIONSHIP_TYPE => {
                        let target_path = crate::common::resolve_zip_file_path(
                            &format!("{}{}", child_parent_path, relationship.target),
                        );
                        let slicer_cache_part = crate::parts::slicer_cache_part::SlicerCachePart::new_from_archive(
                            &child_parent_path,
                            &target_path,
                            &relationship.id,
                            file_path_set,
                            archive,
                        )?;
                        slicer_cache_parts.push(slicer_cache_part);
                    }
                    crate::parts::time_line_cache_part::RELATIONSHIP_TYPE => {
                        let target_path = crate::common::resolve_zip_file_path(
                            &format!("{}{}", child_parent_path, relationship.target),
                        );
                        let time_line_cache_part = crate::parts::time_line_cache_part::TimeLineCachePart::new_from_archive(
                            &child_parent_path,
                            &target_path,
                            &relationship.id,
                            file_path_set,
                            archive,
                        )?;
                        time_line_cache_parts.push(time_line_cache_part);
                    }
                    crate::parts::workbook_person_part::RELATIONSHIP_TYPE => {
                        let target_path = crate::common::resolve_zip_file_path(
                            &format!("{}{}", child_parent_path, relationship.target),
                        );
                        let workbook_person_part = crate::parts::workbook_person_part::WorkbookPersonPart::new_from_archive(
                            &child_parent_path,
                            &target_path,
                            &relationship.id,
                            file_path_set,
                            archive,
                        )?;
                        workbook_person_parts.push(workbook_person_part);
                    }
                    crate::parts::rd_rich_value_part::RELATIONSHIP_TYPE => {
                        let target_path = crate::common::resolve_zip_file_path(
                            &format!("{}{}", child_parent_path, relationship.target),
                        );
                        let rd_rich_value_part = crate::parts::rd_rich_value_part::RdRichValuePart::new_from_archive(
                            &child_parent_path,
                            &target_path,
                            &relationship.id,
                            file_path_set,
                            archive,
                        )?;
                        rd_rich_value_parts.push(rd_rich_value_part);
                    }
                    crate::parts::rd_rich_value_structure_part::RELATIONSHIP_TYPE => {
                        let target_path = crate::common::resolve_zip_file_path(
                            &format!("{}{}", child_parent_path, relationship.target),
                        );
                        let rd_rich_value_structure_part = crate::parts::rd_rich_value_structure_part::RdRichValueStructurePart::new_from_archive(
                            &child_parent_path,
                            &target_path,
                            &relationship.id,
                            file_path_set,
                            archive,
                        )?;
                        ct_rd_rich_value_structure_parts
                            .push(rd_rich_value_structure_part);
                    }
                    crate::parts::rd_array_part::RELATIONSHIP_TYPE => {
                        let target_path = crate::common::resolve_zip_file_path(
                            &format!("{}{}", child_parent_path, relationship.target),
                        );
                        let rd_array_part = crate::parts::rd_array_part::RdArrayPart::new_from_archive(
                            &child_parent_path,
                            &target_path,
                            &relationship.id,
                            file_path_set,
                            archive,
                        )?;
                        rd_array_parts.push(rd_array_part);
                    }
                    crate::parts::rich_styles_part::RELATIONSHIP_TYPE => {
                        let target_path = crate::common::resolve_zip_file_path(
                            &format!("{}{}", child_parent_path, relationship.target),
                        );
                        let rich_styles_part = crate::parts::rich_styles_part::RichStylesPart::new_from_archive(
                            &child_parent_path,
                            &target_path,
                            &relationship.id,
                            file_path_set,
                            archive,
                        )?;
                        rich_styles_parts.push(rich_styles_part);
                    }
                    crate::parts::rd_supporting_property_bag_part::RELATIONSHIP_TYPE => {
                        let target_path = crate::common::resolve_zip_file_path(
                            &format!("{}{}", child_parent_path, relationship.target),
                        );
                        let rd_supporting_property_bag_part = crate::parts::rd_supporting_property_bag_part::RdSupportingPropertyBagPart::new_from_archive(
                            &child_parent_path,
                            &target_path,
                            &relationship.id,
                            file_path_set,
                            archive,
                        )?;
                        rd_supporting_property_bag_parts
                            .push(rd_supporting_property_bag_part);
                    }
                    crate::parts::rd_supporting_property_bag_structure_part::RELATIONSHIP_TYPE => {
                        let target_path = crate::common::resolve_zip_file_path(
                            &format!("{}{}", child_parent_path, relationship.target),
                        );
                        let rd_supporting_property_bag_structure_part = crate::parts::rd_supporting_property_bag_structure_part::RdSupportingPropertyBagStructurePart::new_from_archive(
                            &child_parent_path,
                            &target_path,
                            &relationship.id,
                            file_path_set,
                            archive,
                        )?;
                        rd_supporting_property_bag_structure_parts
                            .push(rd_supporting_property_bag_structure_part);
                    }
                    crate::parts::rd_rich_value_types_part::RELATIONSHIP_TYPE => {
                        let target_path = crate::common::resolve_zip_file_path(
                            &format!("{}{}", child_parent_path, relationship.target),
                        );
                        let rd_rich_value_types_part = crate::parts::rd_rich_value_types_part::RdRichValueTypesPart::new_from_archive(
                            &child_parent_path,
                            &target_path,
                            &relationship.id,
                            file_path_set,
                            archive,
                        )?;
                        rd_rich_value_types_parts.push(rd_rich_value_types_part);
                    }
                    crate::parts::rd_rich_value_web_image_part::RELATIONSHIP_TYPE => {
                        let target_path = crate::common::resolve_zip_file_path(
                            &format!("{}{}", child_parent_path, relationship.target),
                        );
                        rd_rich_value_web_image_part = Some(
                            crate::parts::rd_rich_value_web_image_part::RdRichValueWebImagePart::new_from_archive(
                                &child_parent_path,
                                &target_path,
                                &relationship.id,
                                file_path_set,
                                archive,
                            )?,
                        );
                    }
                    crate::parts::feature_property_bags_part::RELATIONSHIP_TYPE => {
                        let target_path = crate::common::resolve_zip_file_path(
                            &format!("{}{}", child_parent_path, relationship.target),
                        );
                        feature_property_bags_part = Some(
                            crate::parts::feature_property_bags_part::FeaturePropertyBagsPart::new_from_archive(
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
            calculation_chain_part,
            cell_metadata_part,
            connections_part,
            custom_xml_mappings_part,
            shared_string_table_part,
            workbook_revision_header_part,
            workbook_user_data_part,
            workbook_styles_part,
            theme_part,
            thumbnail_part,
            volatile_dependencies_part,
            chartsheet_parts,
            dialogsheet_parts,
            external_workbook_parts,
            pivot_table_cache_definition_parts,
            worksheet_parts,
            excel_attached_toolbars_part,
            vba_project_part,
            macro_sheet_parts,
            international_macro_sheet_parts,
            custom_data_properties_parts,
            slicer_cache_parts,
            time_line_cache_parts,
            workbook_person_parts,
            rd_rich_value_parts,
            ct_rd_rich_value_structure_parts,
            rd_array_parts,
            rich_styles_parts,
            rd_supporting_property_bag_parts,
            rd_supporting_property_bag_structure_parts,
            rd_rich_value_types_parts,
            rd_rich_value_web_image_part,
            feature_property_bags_part,
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
        let workbook_part_dir_path = crate::common::resolve_zip_file_path(
            &format!("{}{}/", parent_path, "xl"),
        );
        if !workbook_part_dir_path.is_empty()
            && !entry_set.contains(&workbook_part_dir_path)
        {
            zip.add_directory(
                &workbook_part_dir_path,
                zip::write::SimpleFileOptions::default(),
            )?;
            entry_set.insert(workbook_part_dir_path);
        }
        if !entry_set.contains(&self.inner_path) {
            zip.start_file(&self.inner_path, options)?;
            zip.write_all(self.root_element.to_string()?.as_bytes())?;
            entry_set.insert(self.inner_path.to_string());
        }
        let child_parent_path = format!("{}{}", parent_path, "xl/");
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
        for custom_xml_part in &self.custom_xml_parts {
            custom_xml_part.save_zip(&child_parent_path, zip, entry_set)?;
        }
        if let Some(calculation_chain_part) = &self.calculation_chain_part {
            calculation_chain_part.save_zip(&child_parent_path, zip, entry_set)?;
        }
        if let Some(cell_metadata_part) = &self.cell_metadata_part {
            cell_metadata_part.save_zip(&child_parent_path, zip, entry_set)?;
        }
        if let Some(connections_part) = &self.connections_part {
            connections_part.save_zip(&child_parent_path, zip, entry_set)?;
        }
        if let Some(custom_xml_mappings_part) = &self.custom_xml_mappings_part {
            custom_xml_mappings_part.save_zip(&child_parent_path, zip, entry_set)?;
        }
        if let Some(shared_string_table_part) = &self.shared_string_table_part {
            shared_string_table_part.save_zip(&child_parent_path, zip, entry_set)?;
        }
        if let Some(workbook_revision_header_part) = &self.workbook_revision_header_part
        {
            workbook_revision_header_part.save_zip(&child_parent_path, zip, entry_set)?;
        }
        if let Some(workbook_user_data_part) = &self.workbook_user_data_part {
            workbook_user_data_part.save_zip(&child_parent_path, zip, entry_set)?;
        }
        if let Some(workbook_styles_part) = &self.workbook_styles_part {
            workbook_styles_part.save_zip(&child_parent_path, zip, entry_set)?;
        }
        if let Some(theme_part) = &self.theme_part {
            theme_part.save_zip(&child_parent_path, zip, entry_set)?;
        }
        if let Some(thumbnail_part) = &self.thumbnail_part {
            thumbnail_part.save_zip(&child_parent_path, zip, entry_set)?;
        }
        if let Some(volatile_dependencies_part) = &self.volatile_dependencies_part {
            volatile_dependencies_part.save_zip(&child_parent_path, zip, entry_set)?;
        }
        for chartsheet_part in &self.chartsheet_parts {
            chartsheet_part.save_zip(&child_parent_path, zip, entry_set)?;
        }
        for dialogsheet_part in &self.dialogsheet_parts {
            dialogsheet_part.save_zip(&child_parent_path, zip, entry_set)?;
        }
        for external_workbook_part in &self.external_workbook_parts {
            external_workbook_part.save_zip(&child_parent_path, zip, entry_set)?;
        }
        for pivot_table_cache_definition_part in &self.pivot_table_cache_definition_parts
        {
            pivot_table_cache_definition_part
                .save_zip(&child_parent_path, zip, entry_set)?;
        }
        for worksheet_part in &self.worksheet_parts {
            worksheet_part.save_zip(&child_parent_path, zip, entry_set)?;
        }
        if let Some(excel_attached_toolbars_part) = &self.excel_attached_toolbars_part {
            excel_attached_toolbars_part.save_zip(&child_parent_path, zip, entry_set)?;
        }
        if let Some(vba_project_part) = &self.vba_project_part {
            vba_project_part.save_zip(&child_parent_path, zip, entry_set)?;
        }
        for macro_sheet_part in &self.macro_sheet_parts {
            macro_sheet_part.save_zip(&child_parent_path, zip, entry_set)?;
        }
        for international_macro_sheet_part in &self.international_macro_sheet_parts {
            international_macro_sheet_part.save_zip(&child_parent_path, zip, entry_set)?;
        }
        for custom_data_properties_part in &self.custom_data_properties_parts {
            custom_data_properties_part.save_zip(&child_parent_path, zip, entry_set)?;
        }
        for slicer_cache_part in &self.slicer_cache_parts {
            slicer_cache_part.save_zip(&child_parent_path, zip, entry_set)?;
        }
        for time_line_cache_part in &self.time_line_cache_parts {
            time_line_cache_part.save_zip(&child_parent_path, zip, entry_set)?;
        }
        for workbook_person_part in &self.workbook_person_parts {
            workbook_person_part.save_zip(&child_parent_path, zip, entry_set)?;
        }
        for rd_rich_value_part in &self.rd_rich_value_parts {
            rd_rich_value_part.save_zip(&child_parent_path, zip, entry_set)?;
        }
        for rd_rich_value_structure_part in &self.ct_rd_rich_value_structure_parts {
            rd_rich_value_structure_part.save_zip(&child_parent_path, zip, entry_set)?;
        }
        for rd_array_part in &self.rd_array_parts {
            rd_array_part.save_zip(&child_parent_path, zip, entry_set)?;
        }
        for rich_styles_part in &self.rich_styles_parts {
            rich_styles_part.save_zip(&child_parent_path, zip, entry_set)?;
        }
        for rd_supporting_property_bag_part in &self.rd_supporting_property_bag_parts {
            rd_supporting_property_bag_part
                .save_zip(&child_parent_path, zip, entry_set)?;
        }
        for rd_supporting_property_bag_structure_part in &self
            .rd_supporting_property_bag_structure_parts
        {
            rd_supporting_property_bag_structure_part
                .save_zip(&child_parent_path, zip, entry_set)?;
        }
        for rd_rich_value_types_part in &self.rd_rich_value_types_parts {
            rd_rich_value_types_part.save_zip(&child_parent_path, zip, entry_set)?;
        }
        if let Some(rd_rich_value_web_image_part) = &self.rd_rich_value_web_image_part {
            rd_rich_value_web_image_part.save_zip(&child_parent_path, zip, entry_set)?;
        }
        if let Some(feature_property_bags_part) = &self.feature_property_bags_part {
            feature_property_bags_part.save_zip(&child_parent_path, zip, entry_set)?;
        }
        Ok(())
    }
}
