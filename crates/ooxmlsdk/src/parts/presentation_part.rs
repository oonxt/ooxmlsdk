use hard_xml::{XmlRead, XmlWrite};
pub const RELATIONSHIP_TYPE: &str = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument";
#[derive(Clone, Debug, Default)]
pub struct PresentationPart {
    pub r_id: String,
    pub relationships: Option<crate::packages::opc_relationships::Relationships>,
    pub rels_path: String,
    pub inner_path: String,
    pub root_element: crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::Presentation,
    pub custom_xml_parts: Vec<crate::parts::custom_xml_part::CustomXmlPart>,
    pub font_parts: Vec<crate::parts::font_part::FontPart>,
    pub presentation_properties_part: Option<
        crate::parts::presentation_properties_part::PresentationPropertiesPart,
    >,
    pub table_styles_part: Option<crate::parts::table_styles_part::TableStylesPart>,
    pub theme_part: Option<crate::parts::theme_part::ThemePart>,
    pub view_properties_part: Option<
        crate::parts::view_properties_part::ViewPropertiesPart,
    >,
    pub notes_master_part: Option<crate::parts::notes_master_part::NotesMasterPart>,
    pub slide_parts: Vec<crate::parts::slide_part::SlidePart>,
    pub slide_master_parts: Vec<crate::parts::slide_master_part::SlideMasterPart>,
    pub user_defined_tags_part: Option<
        crate::parts::user_defined_tags_part::UserDefinedTagsPart,
    >,
    pub comment_authors_part: Option<
        crate::parts::comment_authors_part::CommentAuthorsPart,
    >,
    pub handout_master_part: Option<
        crate::parts::handout_master_part::HandoutMasterPart,
    >,
    pub legacy_diagram_text_info_part: Option<
        crate::parts::legacy_diagram_text_info_part::LegacyDiagramTextInfoPart,
    >,
    pub vba_project_part: Option<crate::parts::vba_project_part::VbaProjectPart>,
    pub comment_parts: Vec<
        crate::parts::power_point_comment_part::PowerPointCommentPart,
    >,
    pub authors_part: Option<
        crate::parts::power_point_authors_part::PowerPointAuthorsPart,
    >,
}
impl PresentationPart {
    #[allow(unused_variables)]
    pub(crate) fn new_from_archive(
        parent_path: &str,
        path: &str,
        r_id: &str,
        file_path_set: &std::collections::HashSet<String>,
        archive: &mut zip::ZipArchive<std::io::BufReader<std::fs::File>>,
    ) -> Result<Self, crate::common::SdkError> {
        let mut rels_path = "".to_string();
        let child_parent_path = format!("{}{}", parent_path, "ppt/");
        let part_target_str = if path.ends_with(".xml") {
            &path[path
                .rfind('/')
                .ok_or_else(|| crate::common::SdkError::CommonError(path.to_string()))?
                + 1..path.len()]
        } else {
            ""
        };
        let presentation_part_rels_path = crate::common::resolve_zip_file_path(
            &format!("{}_rels/{}.rels", child_parent_path, part_target_str),
        );
        let relationships = if let Some(file_path) = file_path_set
            .get(&presentation_part_rels_path)
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
            crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::Presentation::from_str(
                &crate::common::read_string(&mut archive.by_name(path)?)?,
            )?,
        );
        let mut custom_xml_parts: Vec<crate::parts::custom_xml_part::CustomXmlPart> = vec![];
        let mut font_parts: Vec<crate::parts::font_part::FontPart> = vec![];
        let mut presentation_properties_part: Option<
            crate::parts::presentation_properties_part::PresentationPropertiesPart,
        > = None;
        let mut table_styles_part: Option<
            crate::parts::table_styles_part::TableStylesPart,
        > = None;
        let mut theme_part: Option<crate::parts::theme_part::ThemePart> = None;
        let mut view_properties_part: Option<
            crate::parts::view_properties_part::ViewPropertiesPart,
        > = None;
        let mut notes_master_part: Option<
            crate::parts::notes_master_part::NotesMasterPart,
        > = None;
        let mut slide_parts: Vec<crate::parts::slide_part::SlidePart> = vec![];
        let mut slide_master_parts: Vec<
            crate::parts::slide_master_part::SlideMasterPart,
        > = vec![];
        let mut user_defined_tags_part: Option<
            crate::parts::user_defined_tags_part::UserDefinedTagsPart,
        > = None;
        let mut comment_authors_part: Option<
            crate::parts::comment_authors_part::CommentAuthorsPart,
        > = None;
        let mut handout_master_part: Option<
            crate::parts::handout_master_part::HandoutMasterPart,
        > = None;
        let mut legacy_diagram_text_info_part: Option<
            crate::parts::legacy_diagram_text_info_part::LegacyDiagramTextInfoPart,
        > = None;
        let mut vba_project_part: Option<
            crate::parts::vba_project_part::VbaProjectPart,
        > = None;
        let mut comment_parts: Vec<
            crate::parts::power_point_comment_part::PowerPointCommentPart,
        > = vec![];
        let mut authors_part: Option<
            crate::parts::power_point_authors_part::PowerPointAuthorsPart,
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
                    crate::parts::font_part::RELATIONSHIP_TYPE => {
                        let target_path = crate::common::resolve_zip_file_path(
                            &format!("{}{}", child_parent_path, relationship.target),
                        );
                        let font_part = crate::parts::font_part::FontPart::new_from_archive(
                            &child_parent_path,
                            &target_path,
                            &relationship.id,
                            file_path_set,
                            archive,
                        )?;
                        font_parts.push(font_part);
                    }
                    crate::parts::presentation_properties_part::RELATIONSHIP_TYPE => {
                        let target_path = crate::common::resolve_zip_file_path(
                            &format!("{}{}", child_parent_path, relationship.target),
                        );
                        presentation_properties_part = Some(
                            crate::parts::presentation_properties_part::PresentationPropertiesPart::new_from_archive(
                                &child_parent_path,
                                &target_path,
                                &relationship.id,
                                file_path_set,
                                archive,
                            )?,
                        );
                    }
                    crate::parts::table_styles_part::RELATIONSHIP_TYPE => {
                        let target_path = crate::common::resolve_zip_file_path(
                            &format!("{}{}", child_parent_path, relationship.target),
                        );
                        table_styles_part = Some(
                            crate::parts::table_styles_part::TableStylesPart::new_from_archive(
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
                    crate::parts::view_properties_part::RELATIONSHIP_TYPE => {
                        let target_path = crate::common::resolve_zip_file_path(
                            &format!("{}{}", child_parent_path, relationship.target),
                        );
                        view_properties_part = Some(
                            crate::parts::view_properties_part::ViewPropertiesPart::new_from_archive(
                                &child_parent_path,
                                &target_path,
                                &relationship.id,
                                file_path_set,
                                archive,
                            )?,
                        );
                    }
                    crate::parts::notes_master_part::RELATIONSHIP_TYPE => {
                        let target_path = crate::common::resolve_zip_file_path(
                            &format!("{}{}", child_parent_path, relationship.target),
                        );
                        notes_master_part = Some(
                            crate::parts::notes_master_part::NotesMasterPart::new_from_archive(
                                &child_parent_path,
                                &target_path,
                                &relationship.id,
                                file_path_set,
                                archive,
                            )?,
                        );
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
                    crate::parts::slide_master_part::RELATIONSHIP_TYPE => {
                        let target_path = crate::common::resolve_zip_file_path(
                            &format!("{}{}", child_parent_path, relationship.target),
                        );
                        let slide_master_part = crate::parts::slide_master_part::SlideMasterPart::new_from_archive(
                            &child_parent_path,
                            &target_path,
                            &relationship.id,
                            file_path_set,
                            archive,
                        )?;
                        slide_master_parts.push(slide_master_part);
                    }
                    crate::parts::user_defined_tags_part::RELATIONSHIP_TYPE => {
                        let target_path = crate::common::resolve_zip_file_path(
                            &format!("{}{}", child_parent_path, relationship.target),
                        );
                        user_defined_tags_part = Some(
                            crate::parts::user_defined_tags_part::UserDefinedTagsPart::new_from_archive(
                                &child_parent_path,
                                &target_path,
                                &relationship.id,
                                file_path_set,
                                archive,
                            )?,
                        );
                    }
                    crate::parts::comment_authors_part::RELATIONSHIP_TYPE => {
                        let target_path = crate::common::resolve_zip_file_path(
                            &format!("{}{}", child_parent_path, relationship.target),
                        );
                        comment_authors_part = Some(
                            crate::parts::comment_authors_part::CommentAuthorsPart::new_from_archive(
                                &child_parent_path,
                                &target_path,
                                &relationship.id,
                                file_path_set,
                                archive,
                            )?,
                        );
                    }
                    crate::parts::handout_master_part::RELATIONSHIP_TYPE => {
                        let target_path = crate::common::resolve_zip_file_path(
                            &format!("{}{}", child_parent_path, relationship.target),
                        );
                        handout_master_part = Some(
                            crate::parts::handout_master_part::HandoutMasterPart::new_from_archive(
                                &child_parent_path,
                                &target_path,
                                &relationship.id,
                                file_path_set,
                                archive,
                            )?,
                        );
                    }
                    crate::parts::legacy_diagram_text_info_part::RELATIONSHIP_TYPE => {
                        let target_path = crate::common::resolve_zip_file_path(
                            &format!("{}{}", child_parent_path, relationship.target),
                        );
                        legacy_diagram_text_info_part = Some(
                            crate::parts::legacy_diagram_text_info_part::LegacyDiagramTextInfoPart::new_from_archive(
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
                    crate::parts::power_point_authors_part::RELATIONSHIP_TYPE => {
                        let target_path = crate::common::resolve_zip_file_path(
                            &format!("{}{}", child_parent_path, relationship.target),
                        );
                        authors_part = Some(
                            crate::parts::power_point_authors_part::PowerPointAuthorsPart::new_from_archive(
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
            font_parts,
            presentation_properties_part,
            table_styles_part,
            theme_part,
            view_properties_part,
            notes_master_part,
            slide_parts,
            slide_master_parts,
            user_defined_tags_part,
            comment_authors_part,
            handout_master_part,
            legacy_diagram_text_info_part,
            vba_project_part,
            comment_parts,
            authors_part,
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
        let presentation_part_dir_path = crate::common::resolve_zip_file_path(
            &format!("{}{}/", parent_path, "ppt"),
        );
        if !presentation_part_dir_path.is_empty()
            && !entry_set.contains(&presentation_part_dir_path)
        {
            zip.add_directory(
                &presentation_part_dir_path,
                zip::write::SimpleFileOptions::default(),
            )?;
            entry_set.insert(presentation_part_dir_path);
        }
        if !entry_set.contains(&self.inner_path) {
            zip.start_file(&self.inner_path, options)?;
            zip.write_all(crate::common::SCHEMA_XML.as_bytes())?;
            zip.write_all(self.root_element.to_string()?.as_bytes())?;
            entry_set.insert(self.inner_path.to_string());
        }
        let child_parent_path = format!("{}{}", parent_path, "ppt/");
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
        for font_part in &self.font_parts {
            font_part.save_zip(&child_parent_path, zip, entry_set)?;
        }
        if let Some(presentation_properties_part) = &self.presentation_properties_part {
            presentation_properties_part.save_zip(&child_parent_path, zip, entry_set)?;
        }
        if let Some(table_styles_part) = &self.table_styles_part {
            table_styles_part.save_zip(&child_parent_path, zip, entry_set)?;
        }
        if let Some(theme_part) = &self.theme_part {
            theme_part.save_zip(&child_parent_path, zip, entry_set)?;
        }
        if let Some(view_properties_part) = &self.view_properties_part {
            view_properties_part.save_zip(&child_parent_path, zip, entry_set)?;
        }
        if let Some(notes_master_part) = &self.notes_master_part {
            notes_master_part.save_zip(&child_parent_path, zip, entry_set)?;
        }
        for slide_part in &self.slide_parts {
            slide_part.save_zip(&child_parent_path, zip, entry_set)?;
        }
        for slide_master_part in &self.slide_master_parts {
            slide_master_part.save_zip(&child_parent_path, zip, entry_set)?;
        }
        if let Some(user_defined_tags_part) = &self.user_defined_tags_part {
            user_defined_tags_part.save_zip(&child_parent_path, zip, entry_set)?;
        }
        if let Some(comment_authors_part) = &self.comment_authors_part {
            comment_authors_part.save_zip(&child_parent_path, zip, entry_set)?;
        }
        if let Some(handout_master_part) = &self.handout_master_part {
            handout_master_part.save_zip(&child_parent_path, zip, entry_set)?;
        }
        if let Some(legacy_diagram_text_info_part) = &self.legacy_diagram_text_info_part
        {
            legacy_diagram_text_info_part.save_zip(&child_parent_path, zip, entry_set)?;
        }
        if let Some(vba_project_part) = &self.vba_project_part {
            vba_project_part.save_zip(&child_parent_path, zip, entry_set)?;
        }
        for power_point_comment_part in &self.comment_parts {
            power_point_comment_part.save_zip(&child_parent_path, zip, entry_set)?;
        }
        if let Some(authors_part) = &self.authors_part {
            authors_part.save_zip(&child_parent_path, zip, entry_set)?;
        }
        Ok(())
    }
}
