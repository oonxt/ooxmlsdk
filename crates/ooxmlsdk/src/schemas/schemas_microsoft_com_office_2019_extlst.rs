/// Defines the Extension Class.
/// When the object is serialized out as xml, it's qualified name is oel:ext.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "oel:ext")]
pub struct Extension {
    /// uri
    /// Represents the following attribute in the schema: :uri
    #[xml(attr = "uri")]
    pub uri: Option<String>,
}
