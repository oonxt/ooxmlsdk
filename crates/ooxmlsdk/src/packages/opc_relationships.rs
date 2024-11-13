use hard_xml::{XmlRead, XmlWrite};
use std::collections::HashMap;
use std::io::Write;
const XMLNS: &str = "http://schemas.openxmlformats.org/package/2006/relationships";

#[derive(Clone, Debug, XmlRead, XmlWrite)]
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
    TargetMode { External = "External", Internal = "Internal", }
}

impl Default for Relationships {
    fn default() -> Self {
        Self {
            xmlns: Some(XMLNS.to_string()),
            xmlns_map: HashMap::new(),
            mc_ignorable: None,
            relationship: vec![],
        }
    }
}