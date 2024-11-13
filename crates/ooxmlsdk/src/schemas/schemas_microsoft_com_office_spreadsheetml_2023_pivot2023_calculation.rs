#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum AggregationType {
    #[default]
    DistinctCount,
    Median,
}
crate::__string_enum! {
    AggregationType { DistinctCount = "distinctCount", Median = "median", }
}
/// Defines the AggregationInfo Class.
/// When the object is serialized out as xml, it's qualified name is xlpcalc:aggregationInfo.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xlpcalc:aggregationInfo")]
pub struct AggregationInfo {
    /// aggregationType
    /// Represents the following attribute in the schema: :aggregationType
    #[xml(attr = "aggregationType")]
    pub aggregation_type: AggregationType,
    /// sourceField
    /// Represents the following attribute in the schema: :sourceField
    #[xml(attr = "sourceField")]
    pub source_field: u32,
}
/// Defines the FeatureSupport Class.
/// When the object is serialized out as xml, it's qualified name is xlpcalc:featureSupportInfo.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xlpcalc:featureSupportInfo")]
pub struct FeatureSupport {
    /// featureName
    /// Represents the following attribute in the schema: :featureName
    #[xml(attr = "featureName")]
    pub feature_name: String,
}
