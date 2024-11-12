/// Defines the ReadonlyRecommended Class.
/// When the object is serialized out as xml, it's qualified name is p1710:readonlyRecommended.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p1710:readonlyRecommended")]
pub struct ReadonlyRecommended {
    /// val
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: bool,
}
