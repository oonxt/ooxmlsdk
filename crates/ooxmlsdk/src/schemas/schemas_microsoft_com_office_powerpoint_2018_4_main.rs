#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum ClassificationOutcomeType {
    #[default]
    None,
    Hdr,
    Ftr,
    Watermark,
}
crate::__string_enum! {
    ClassificationOutcomeType { None = "none", Hdr = "hdr", Ftr = "ftr", Watermark =
    "watermark", }
}
/// Defines the ClassificationOutcome Class.
/// When the object is serialized out as xml, it's qualified name is p184:classification.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p184:classification")]
pub struct ClassificationOutcome {
    /// val
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: Option<ClassificationOutcomeType>,
}
