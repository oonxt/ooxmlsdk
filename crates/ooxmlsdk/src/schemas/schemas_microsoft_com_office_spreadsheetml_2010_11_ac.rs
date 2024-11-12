/// Defines the AbsolutePath Class.
/// When the object is serialized out as xml, it's qualified name is x15ac:absPath.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x15ac:absPath")]
pub struct AbsolutePath {
    /// url
    /// Represents the following attribute in the schema: :url
    #[xml(attr = "url")]
    pub url: String,
}
