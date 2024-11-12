/// Defines the CalcFeatures Class.
/// When the object is serialized out as xml, it's qualified name is xcalcf:calcFeatures.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xcalcf:calcFeatures")]
pub struct CalcFeatures {
    /// _
    #[xml(child = "xcalcf:feature")]
    pub xcalcf_feature: Vec<CalcFeature>,
}
/// Defines the CalcFeature Class.
/// When the object is serialized out as xml, it's qualified name is xcalcf:feature.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xcalcf:feature")]
pub struct CalcFeature {
    /// name
    /// Represents the following attribute in the schema: :name
    #[xml(attr = "name")]
    pub name: String,
}
