use hard_xml::{XmlRead, XmlResult, XmlWrite, XmlWriter};
use std::io::Write;
use crate::common::SCHEMA_XML;

#[derive(Clone, Debug, Default, XmlRead)]
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

const SCHEMA_CORE_PROPERTIES: &str = "http://schemas.openxmlformats.org/package/2006/metadata/core-properties";
impl XmlWrite for CoreProperties {
    fn to_writer<W: Write>(&self, writer: &mut XmlWriter<W>) -> XmlResult<()> {
        let _ = write!(writer.inner, "{}", SCHEMA_XML);
        writer.write_element_start("cp:coreProperties")?;
        writer.write_attribute("xmlns", SCHEMA_CORE_PROPERTIES)?;
        writer.write_prefix::<String>("xmlns", &self.xmlns_map)?;
        if let Some(mc_ignorable) = &self.mc_ignorable {
            writer.write_attribute("mc:Ignorable", mc_ignorable)?;
        }
        writer.write_element_end_open()?;
        if let Some(v) = &self.category {
            writer.write_flatten_text("cp:category", v, false)?;
        }
        if let Some(v) = &self.content_status {
            writer.write_flatten_text("cp:contentStatus", v, false)?;
        }
        if let Some(v) = &self.created {
            v.to_writer(writer)?;
        }
        if let Some(v) = &self.creator {
            writer.write_flatten_text("dc:creator", v, false)?;
        }
        if let Some(v) = &self.description {
            writer.write_flatten_text("dc:description", v, false)?;
        }
        if let Some(v) = &self.identifier {
            writer.write_flatten_text("dc:identifier", v, false)?;
        }
        if let Some(v) = &self.keywords {
            writer.write_flatten_text("cp:keywords", v, false)?;
        }
        if let Some(v) = &self.language {
            writer.write_flatten_text("dc:language", v, false)?;
        }
        if let Some(v) = &self.last_modified_by {
            writer.write_flatten_text("cp:lastModifiedBy", v, false)?;
        }
        if let Some(v) = &self.last_printed {
            writer.write_flatten_text("cp:lastPrinted", v, false)?;
        }
        if let Some(v) = &self.modified {
            v.to_writer(writer)?;
        }
        if let Some(v) = &self.revision {
            writer.write_flatten_text("cp:revision", v, false)?;
        }
        if let Some(v) = &self.subject {
            writer.write_flatten_text("dc:subject", v, false)?;
        }
        if let Some(v) = &self.title {
            writer.write_flatten_text("dc:title", v, false)?;
        }
        if let Some(v) = &self.version {
            writer.write_flatten_text("cp:version", v, false)?;
        }
        writer.write_element_end_close("cp:coreProperties")?;
        Ok(())
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