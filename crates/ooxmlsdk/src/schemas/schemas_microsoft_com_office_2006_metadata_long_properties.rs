/// Defines the LongProperties Class.
/// When the object is serialized out as xml, it's qualified name is lp:LongProperties.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "lp:LongProperties")]
pub struct LongProperties {
    /// _
    #[xml(child = "lp:LongProp")]
    pub lp_long_prop: Vec<LongProperty>,
}
/// Defines the LongProperty Class.
/// When the object is serialized out as xml, it's qualified name is lp:LongProp.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "lp:LongProp")]
pub struct LongProperty {
    /// name
    /// Represents the following attribute in the schema: :name
    #[xml(attr = "name")]
    pub name: Option<String>,
    #[xml(text)]
    pub child: String,
}
