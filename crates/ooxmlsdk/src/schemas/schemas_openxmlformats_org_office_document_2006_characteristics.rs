#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum RelationValues {
    #[default]
    GreaterThanOrEqualTo,
    LessThanOrEqualTo,
    GreaterThan,
    LessThan,
    EqualTo,
}
crate::__string_enum! {
    RelationValues { GreaterThanOrEqualTo = "ge", LessThanOrEqualTo = "le", GreaterThan =
    "gt", LessThan = "lt", EqualTo = "eq", }
}
/// Set of Additional Characteristics.
/// When the object is serialized out as xml, it's qualified name is ac:additionalCharacteristics.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "ac:additionalCharacteristics")]
pub struct AdditionalCharacteristicsInfo {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// _
    #[xml(child = "ac:characteristic")]
    pub ac_characteristic: Vec<Characteristic>,
}
/// Single Characteristic.
/// When the object is serialized out as xml, it's qualified name is ac:characteristic.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "ac:characteristic")]
pub struct Characteristic {
    /// Name of Characteristic
    /// Represents the following attribute in the schema: :name
    #[xml(attr = "name")]
    pub name: String,
    /// Relationship of Value to Name
    /// Represents the following attribute in the schema: :relation
    #[xml(attr = "relation")]
    pub relation: RelationValues,
    /// Characteristic Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: String,
    /// Characteristic Grammar
    /// Represents the following attribute in the schema: :vocabulary
    #[xml(attr = "vocabulary")]
    pub vocabulary: Option<String>,
}
