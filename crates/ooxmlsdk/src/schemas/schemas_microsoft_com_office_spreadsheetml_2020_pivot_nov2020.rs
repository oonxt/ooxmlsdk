/// Defines the Xsdboolean Class.
/// When the object is serialized out as xml, it's qualified name is xxpim:implicitMeasureSupport.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xxpim:implicitMeasureSupport")]
pub struct Xsdboolean {
    #[xml(text)]
    pub child: bool,
}
/// Defines the Ignorable Class.
/// When the object is serialized out as xml, it's qualified name is xxpim:ignorableAfterVersion.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xxpim:ignorableAfterVersion")]
pub struct Ignorable {
    /// version
    /// Represents the following attribute in the schema: :version
    #[xml(attr = "version")]
    pub version: u8,
}
/// Defines the DataFieldFutureData Class.
/// When the object is serialized out as xml, it's qualified name is xxpim:dataFieldFutureData.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xxpim:dataFieldFutureData")]
pub struct DataFieldFutureData {
    /// version
    /// Represents the following attribute in the schema: :version
    #[xml(attr = "version")]
    pub version: u8,
    /// sourceField
    /// Represents the following attribute in the schema: :sourceField
    #[xml(attr = "sourceField")]
    pub source_field: i32,
}
