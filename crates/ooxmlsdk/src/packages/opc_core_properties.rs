use hard_xml::{XmlRead, XmlResult, XmlWrite, XmlWriter};
use std::io::Write;

const SCHEMA_CORE_PROPERTIES: &str = "http://schemas.openxmlformats.org/package/2006/metadata/core-properties";

#[derive(Clone, Debug, Default, XmlRead, XmlWrite)]
#[xml(tag = "cp:coreProperties")]
pub struct CoreProperties {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    #[xml(flatten_text = "cp:category")]
    pub category: Option<String>,
    #[xml(flatten_text = "cp:contentStatus")]
    pub content_status: Option<String>,
    #[xml(child = "dcterms:created")]
    pub created: Option<DctermsCreated>,
    #[xml(flatten_text = "dc:creator")]
    pub creator: Option<String>,
    #[xml(flatten_text = "dc:description")]
    pub description: Option<String>,
    #[xml(flatten_text = "dc:identifier")]
    pub identifier: Option<String>,
    #[xml(flatten_text = "cp:keywords")]
    pub keywords: Option<String>,
    #[xml(flatten_text = "dc:language")]
    pub language: Option<String>,
    #[xml(flatten_text = "cp:lastModifiedBy")]
    pub last_modified_by: Option<String>,
    #[xml(flatten_text = "cp:lastPrinted")]
    pub last_printed: Option<String>,
    #[xml(child = "dcterms:modified")]
    pub modified: Option<DctermsModified>,
    #[xml(flatten_text = "cp:revision")]
    pub revision: Option<String>,
    #[xml(flatten_text = "dc:subject")]
    pub subject: Option<String>,
    #[xml(flatten_text = "dc:title")]
    pub title: Option<String>,
    #[xml(flatten_text = "cp:version")]
    pub version: Option<String>,
}

impl Default for CoreProperties {
    fn default() -> Self {
        Self {
            xmlns: Some(SCHEMA_CORE_PROPERTIES.to_string()),
            ..Default::default()
        }
    }
}

#[derive(Clone, Debug, Default, XmlRead, XmlWrite)]
#[xml(tag = "dcterms:created")]
pub struct DctermsCreated {
    #[xml(attr = "xsi:type")]
    pub xsi_type: Option<String>,
    #[xml(text)]
    pub child: String,
}

#[derive(Clone, Debug, Default, XmlRead, XmlWrite)]
#[xml(tag = "dcterms:modified")]
pub struct DctermsModified {
    #[xml(attr = "xsi:type")]
    pub xsi_type: Option<String>,
    #[xml(text)]
    pub child: String,
}