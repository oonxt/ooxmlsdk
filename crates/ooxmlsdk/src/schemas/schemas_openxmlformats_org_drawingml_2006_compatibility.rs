/// Legacy Drawing Object.
/// When the object is serialized out as xml, it's qualified name is comp:legacyDrawing.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "comp:legacyDrawing")]
pub struct LegacyDrawing {
    /// Shape ID
    /// Represents the following attribute in the schema: :spid
    #[xml(attr = "spid")]
    pub shape_id: String,
}
