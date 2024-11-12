/// Defines the ImageFormula Class.
/// When the object is serialized out as xml, it's qualified name is aif:imageFormula.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "aif:imageFormula")]
pub struct ImageFormula {
    /// formula
    /// Represents the following attribute in the schema: :formula
    #[xml(attr = "formula")]
    pub formula: Option<String>,
}
