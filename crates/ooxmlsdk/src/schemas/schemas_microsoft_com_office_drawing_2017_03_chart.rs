/// Defines the DataDisplayOptions16 Class.
/// When the object is serialized out as xml, it's qualified name is c16r3:dataDisplayOptions16.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c16r3:dataDisplayOptions16")]
pub struct DataDisplayOptions16 {
    /// _
    #[xml(child = "c16r3:dispNaAsBlank")]
    pub boolean_false: Option<BooleanFalse>,
}
/// Defines the BooleanFalse Class.
/// When the object is serialized out as xml, it's qualified name is c16r3:dispNaAsBlank.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c16r3:dispNaAsBlank")]
pub struct BooleanFalse {
    /// val
    /// Represents the following attribute in the schema: c16r3:val
    #[xml(attr = "c16r3:val")]
    pub c16r3_val: Option<bool>,
}
