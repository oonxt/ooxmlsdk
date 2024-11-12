/// Custom XML Data Properties.
/// When the object is serialized out as xml, it's qualified name is ds:datastoreItem.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "ds:datastoreItem")]
pub struct DataStoreItem {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// Custom XML Data ID
    /// Represents the following attribute in the schema: ds:itemID
    #[xml(attr = "ds:itemID")]
    pub item_id: String,
    ///Set of Associated XML Schemas
    #[xml(child = "ds:schemaRefs")]
    pub schema_references: Option<SchemaReferences>,
}
/// Associated XML Schema.
/// When the object is serialized out as xml, it's qualified name is ds:schemaRef.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "ds:schemaRef")]
pub struct SchemaReference {
    /// Target Namespace of Associated XML Schema
    /// Represents the following attribute in the schema: ds:uri
    #[xml(attr = "ds:uri")]
    pub uri: String,
}
/// Set of Associated XML Schemas.
/// When the object is serialized out as xml, it's qualified name is ds:schemaRefs.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "ds:schemaRefs")]
pub struct SchemaReferences {
    /// _
    #[xml(child = "ds:schemaRef")]
    pub ds_schema_ref: Vec<SchemaReference>,
}
