/// Defines the DesignElement Class.
/// When the object is serialized out as xml, it's qualified name is p16:designElem.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p16:designElem")]
pub struct DesignElement {
    /// val
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: Option<bool>,
}
