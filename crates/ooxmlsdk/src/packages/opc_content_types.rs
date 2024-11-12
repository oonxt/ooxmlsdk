use hard_xml::{XmlRead, XmlReader, XmlResult, XmlWrite, XmlWriter};
use std::io::{Read, Write};
use crate::common::SCHEMA_XML;

#[derive(Clone, Debug, Default, hard_xml::XmlRead)]
#[xml(tag = "Types")]
pub struct Types {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "md:Ignorable")]
    pub mc_ignorable: Option<String>,
    #[xml(child = "Default", child = "Override")]
    pub children: Vec<TypesChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlRead, hard_xml::XmlWrite)]
pub enum TypesChildChoice {
    #[xml(tag = "Default")]
    Default(Default),
    #[xml(tag = "Override")]
    Override(Override),
}
#[derive(Clone, Debug, Default, hard_xml::XmlRead, hard_xml::XmlWrite)]
#[xml(tag = "Default")]
pub struct Default {
    #[xml(attr = "Extension")]
    pub extension: String,
    #[xml(attr = "ContentType")]
    pub content_type: String,
}
#[derive(Clone, Debug, Default, hard_xml::XmlRead, hard_xml::XmlWrite)]
#[xml(tag = "Override")]
pub struct Override {
    #[xml(attr = "ContentType")]
    pub content_type: String,
    #[xml(attr = "PartName")]
    pub part_name: String,
}
pub const SCHEMA_CONTENT_TYPES: &str = "http://schemas.openxmlformats.org/package/2006/content-types";
impl hard_xml::XmlWrite for Types {
    fn to_writer<W: Write>(&self, writer: &mut XmlWriter<W>) -> XmlResult<()> {
        writer.write_element_start("Types")?;
        writer.write_attribute("xmlns", SCHEMA_CONTENT_TYPES)?;
        writer.write_prefix::<String>("xmlns", &self.xmlns_map)?;
        if let Some(mc_ignorable) = &self.mc_ignorable {
            writer.write_attribute("md:Ignorable", mc_ignorable)?;
        }
        if self.children.len() == 0usize {
            writer.write_element_end_empty()?;
        } else {
            writer.write_element_end_open()?;
            for ele in &self.children {
                ele.to_writer(writer)?;
            }
            writer.write_element_end_close("Types")?;
        }
        Ok(())
    }
}

#[test]
fn test_types () {

    let txt = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types"><Default Extension="rels" ContentType="application/vnd.openxmlformats-package.relationships+xml"/><Default Extension="xml" ContentType="application/xml"/><Override PartName="/word/document.xml" ContentType="application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml"/><Override PartName="/word/numbering.xml" ContentType="application/vnd.openxmlformats-officedocument.wordprocessingml.numbering+xml"/><Override PartName="/word/styles.xml" ContentType="application/vnd.openxmlformats-officedocument.wordprocessingml.styles+xml"/><Override PartName="/word/settings.xml" ContentType="application/vnd.openxmlformats-officedocument.wordprocessingml.settings+xml"/><Override PartName="/word/webSettings.xml" ContentType="application/vnd.openxmlformats-officedocument.wordprocessingml.webSettings+xml"/><Override PartName="/word/fontTable.xml" ContentType="application/vnd.openxmlformats-officedocument.wordprocessingml.fontTable+xml"/><Override PartName="/word/theme/theme1.xml" ContentType="application/vnd.openxmlformats-officedocument.theme+xml"/><Override PartName="/docProps/core.xml" ContentType="application/vnd.openxmlformats-package.core-properties+xml"/><Override PartName="/docProps/app.xml" ContentType="application/vnd.openxmlformats-officedocument.extended-properties+xml"/></Types>
"#;
    let types = Types::from_str(txt).unwrap();

}