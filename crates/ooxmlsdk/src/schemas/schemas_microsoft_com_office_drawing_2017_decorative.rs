/// Defines the Decorative Class.
/// When the object is serialized out as xml, it's qualified name is adec:decorative.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "adec:decorative")]
pub struct Decorative {
    /// val
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: Option<bool>,
}
