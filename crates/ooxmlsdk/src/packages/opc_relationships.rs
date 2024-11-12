use hard_xml::{XmlRead, XmlResult, XmlWrite, XmlWriter};
use std::io::Write;
use crate::common::SCHEMA_XML;

#[derive(Clone, Debug, Default, XmlRead)]
#[xml(tag = "Relationships")]
pub struct Relationships {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    #[xml(child = "Relationship")]
    pub relationship: Vec<Relationship>,
}

const XMLNS: &str = "http://schemas.openxmlformats.org/package/2006/relationships";
impl XmlWrite for Relationships {
    fn to_writer<W: Write>(&self, writer: &mut XmlWriter<W>) -> XmlResult<()> {
        let _ = write!(writer.inner, "{}", SCHEMA_XML);
        writer.write_element_start("Relationships")?;
        writer.write_attribute("xmlns", XMLNS)?;
        writer.write_prefix::<String>("xmlns", &self.xmlns_map)?;
        if let Some(mc_ignorable) = &self.mc_ignorable {
            writer.write_attribute("mc:Ignorable", mc_ignorable)?;
        }
        if self.relationship.len() == 0usize {
            writer.write_element_end_empty()?;
        } else {
            writer.write_element_end_open()?;
            for ele in &self.relationship {
                ele.to_writer(writer)?;
            }
            writer.write_element_end_close("Relationships")?;
        }
        Ok(())
    }
}
#[derive(Clone, Debug, Default, XmlRead, XmlWrite)]
#[xml(tag = "Relationship")]
pub struct Relationship {
    #[xml(attr = "TargetMode")]
    pub target_mode: Option<TargetMode>,
    #[xml(attr = "Target")]
    pub target: String,
    #[xml(attr = "Type")]
    pub r#type: String,
    #[xml(attr = "Id")]
    pub id: String,
}
crate::__define_enum! {
    TargetMode { External = "external", Internal = "internal", }
}
