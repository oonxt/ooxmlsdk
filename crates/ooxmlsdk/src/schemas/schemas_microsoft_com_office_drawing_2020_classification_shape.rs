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
/// When the object is serialized out as xml, it's qualified name is aclsh:classification.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "aclsh:classification")]
pub struct ClassificationOutcome {
    /// classificationOutcomeType
    /// Represents the following attribute in the schema: :classificationOutcomeType
    #[xml(attr = "classificationOutcomeType")]
    pub classification_outcome_type: Option<ClassificationOutcomeType>,
}
