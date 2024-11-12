#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum HyperlinkColorEnum {
    #[default]
    HLink,
    Tx,
}
crate::__string_enum! {
    HyperlinkColorEnum { HLink = "hlink", Tx = "tx", }
}
/// Defines the HyperlinkColor Class.
/// When the object is serialized out as xml, it's qualified name is ahyp:hlinkClr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "ahyp:hlinkClr")]
pub struct HyperlinkColor {
    /// val
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: HyperlinkColorEnum,
}
