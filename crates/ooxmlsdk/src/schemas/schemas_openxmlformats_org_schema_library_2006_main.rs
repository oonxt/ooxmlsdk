/// Embedded Custom XML Schema Supplementary Data.
/// When the object is serialized out as xml, it's qualified name is sl:schemaLibrary.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "sl:schemaLibrary")]
pub struct SchemaLibrary {
    /// _
    #[xml(child = "sl:schema")]
    pub sl_schema: Vec<Schema>,
}
/// Custom XML Schema Reference.
/// When the object is serialized out as xml, it's qualified name is sl:schema.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "sl:schema")]
pub struct Schema {
    /// Custom XML Schema Namespace
    /// Represents the following attribute in the schema: sl:uri
    #[xml(attr = "sl:uri")]
    pub uri: Option<String>,
    /// Resource File Location
    /// Represents the following attribute in the schema: sl:manifestLocation
    #[xml(attr = "sl:manifestLocation")]
    pub manifest_location: Option<String>,
    /// Custom XML Schema Location
    /// Represents the following attribute in the schema: sl:schemaLocation
    #[xml(attr = "sl:schemaLocation")]
    pub schema_location: Option<String>,
}
