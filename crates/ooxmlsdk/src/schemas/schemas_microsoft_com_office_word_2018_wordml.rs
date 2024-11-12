/// Defines the Extension Class.
/// When the object is serialized out as xml, it's qualified name is w16cur:ext.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w16cur:ext")]
pub struct Extension {
    /// uri
    /// Represents the following attribute in the schema: w16cur:uri
    #[xml(attr = "w16cur:uri")]
    pub w16cur_uri: Option<String>,
}
