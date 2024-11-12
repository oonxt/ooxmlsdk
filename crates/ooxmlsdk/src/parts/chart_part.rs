use hard_xml::{XmlRead, XmlWrite};
pub const RELATIONSHIP_TYPE: &str = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/chart";
#[derive(Clone, Debug, Default)]
pub struct ChartPart {
    pub r_id: String,
    pub relationships: Option<crate::packages::opc_relationships::Relationships>,
    pub rels_path: String,
    pub inner_path: String,
    pub root_element: crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::ChartSpace,
    pub chart_drawing_part: Option<Box<crate::parts::chart_drawing_part::ChartDrawingPart>>,
    pub embedded_package_part: Option<
        crate::parts::embedded_package_part::EmbeddedPackagePart,
    >,
    pub image_parts: Vec<crate::parts::image_part::ImagePart>,
    pub theme_override_part: Option<
        crate::parts::theme_override_part::ThemeOverridePart,
    >,
    pub chart_style_parts: Vec<crate::parts::chart_style_part::ChartStylePart>,
    pub chart_color_style_parts: Vec<
        crate::parts::chart_color_style_part::ChartColorStylePart,
    >,
}
impl ChartPart {
    #[allow(unused_variables)]
    pub(crate) fn new_from_archive(
        parent_path: &str,
        path: &str,
        r_id: &str,
        file_path_set: &std::collections::HashSet<String>,
        archive: &mut zip::ZipArchive<std::io::BufReader<std::fs::File>>,
    ) -> Result<Self, crate::common::SdkError> {
        let mut rels_path = "".to_string();
        let child_parent_path = format!("{}{}", parent_path, "charts/");
        let part_target_str = if path.ends_with(".xml") {
            &path[path
                .rfind('/')
                .ok_or_else(|| crate::common::SdkError::CommonError(path.to_string()))?
                + 1..path.len()]
        } else {
            ""
        };
        let chart_part_rels_path = crate::common::resolve_zip_file_path(
            &format!("{}_rels/{}.rels", child_parent_path, part_target_str),
        );
        let relationships = if let Some(file_path) = file_path_set
            .get(&chart_part_rels_path)
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
            crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::ChartSpace::from_str(
                &crate::common::read_string(&mut archive.by_name(path)?)?,
            )?,
        );
        let mut chart_drawing_part: Option<
            Box<crate::parts::chart_drawing_part::ChartDrawingPart>,
        > = None;
        let mut embedded_package_part: Option<
            crate::parts::embedded_package_part::EmbeddedPackagePart,
        > = None;
        let mut image_parts: Vec<crate::parts::image_part::ImagePart> = vec![];
        let mut theme_override_part: Option<
            crate::parts::theme_override_part::ThemeOverridePart,
        > = None;
        let mut chart_style_parts: Vec<crate::parts::chart_style_part::ChartStylePart> = vec![];
        let mut chart_color_style_parts: Vec<
            crate::parts::chart_color_style_part::ChartColorStylePart,
        > = vec![];
        if let Some(relationships) = &relationships {
            for relationship in &relationships.relationship {
                #[allow(clippy::single_match)]
                match relationship.r#type.as_str() {
                    crate::parts::chart_drawing_part::RELATIONSHIP_TYPE => {
                        let target_path = crate::common::resolve_zip_file_path(
                            &format!("{}{}", child_parent_path, relationship.target),
                        );
                        chart_drawing_part = Some(
                            Box::new(crate::parts::chart_drawing_part::ChartDrawingPart::new_from_archive(
                                &child_parent_path,
                                &target_path,
                                &relationship.id,
                                file_path_set,
                                archive,
                            )?,)
                        );
                    }
                    crate::parts::embedded_package_part::RELATIONSHIP_TYPE => {
                        let target_path = crate::common::resolve_zip_file_path(
                            &format!("{}{}", child_parent_path, relationship.target),
                        );
                        embedded_package_part = Some(
                            crate::parts::embedded_package_part::EmbeddedPackagePart::new_from_archive(
                                &child_parent_path,
                                &target_path,
                                &relationship.id,
                                file_path_set,
                                archive,
                            )?,
                        );
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
                    crate::parts::chart_style_part::RELATIONSHIP_TYPE => {
                        let target_path = crate::common::resolve_zip_file_path(
                            &format!("{}{}", child_parent_path, relationship.target),
                        );
                        let chart_style_part = crate::parts::chart_style_part::ChartStylePart::new_from_archive(
                            &child_parent_path,
                            &target_path,
                            &relationship.id,
                            file_path_set,
                            archive,
                        )?;
                        chart_style_parts.push(chart_style_part);
                    }
                    crate::parts::chart_color_style_part::RELATIONSHIP_TYPE => {
                        let target_path = crate::common::resolve_zip_file_path(
                            &format!("{}{}", child_parent_path, relationship.target),
                        );
                        let chart_color_style_part = crate::parts::chart_color_style_part::ChartColorStylePart::new_from_archive(
                            &child_parent_path,
                            &target_path,
                            &relationship.id,
                            file_path_set,
                            archive,
                        )?;
                        chart_color_style_parts.push(chart_color_style_part);
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
            chart_drawing_part,
            embedded_package_part,
            image_parts,
            theme_override_part,
            chart_style_parts,
            chart_color_style_parts,
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
        let chart_part_dir_path = crate::common::resolve_zip_file_path(
            &format!("{}{}/", parent_path, "charts"),
        );
        if !chart_part_dir_path.is_empty() && !entry_set.contains(&chart_part_dir_path) {
            zip.add_directory(
                &chart_part_dir_path,
                zip::write::SimpleFileOptions::default(),
            )?;
            entry_set.insert(chart_part_dir_path);
        }
        if !entry_set.contains(&self.inner_path) {
            zip.start_file(&self.inner_path, options)?;
            zip.write_all(self.root_element.to_string()?.as_bytes())?;
            entry_set.insert(self.inner_path.to_string());
        }
        let child_parent_path = format!("{}{}", parent_path, "charts/");
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
        if let Some(chart_drawing_part) = &self.chart_drawing_part {
            chart_drawing_part.save_zip(&child_parent_path, zip, entry_set)?;
        }
        if let Some(embedded_package_part) = &self.embedded_package_part {
            embedded_package_part.save_zip(&child_parent_path, zip, entry_set)?;
        }
        for image_part in &self.image_parts {
            image_part.save_zip(&child_parent_path, zip, entry_set)?;
        }
        if let Some(theme_override_part) = &self.theme_override_part {
            theme_override_part.save_zip(&child_parent_path, zip, entry_set)?;
        }
        for chart_style_part in &self.chart_style_parts {
            chart_style_part.save_zip(&child_parent_path, zip, entry_set)?;
        }
        for chart_color_style_part in &self.chart_color_style_parts {
            chart_color_style_part.save_zip(&child_parent_path, zip, entry_set)?;
        }
        Ok(())
    }
}
