/// Defines the SymEx Class.
/// When the object is serialized out as xml, it's qualified name is w16se:symEx.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w16se:symEx")]
pub struct SymEx {
    /// font
    /// Represents the following attribute in the schema: w16se:font
    #[xml(attr = "w16se:font")]
    pub w16se_font: Option<String>,
    /// char
    /// Represents the following attribute in the schema: w16se:char
    #[xml(attr = "w16se:char")]
    pub w16se_char: Option<String>,
}
