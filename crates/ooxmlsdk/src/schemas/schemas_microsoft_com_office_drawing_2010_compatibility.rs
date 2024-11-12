/// Defines the CompatibilityShape Class.
/// When the object is serialized out as xml, it's qualified name is com14:compatSp.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "com14:compatSp")]
pub struct CompatibilityShape {
    /// spid
    /// Represents the following attribute in the schema: :spid
    #[xml(attr = "spid")]
    pub shape_id: String,
}
