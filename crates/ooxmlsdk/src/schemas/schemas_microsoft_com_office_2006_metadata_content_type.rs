/// Defines the ContentTypeSchema Class.
/// When the object is serialized out as xml, it's qualified name is ct:contentTypeSchema.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "ct:contentTypeSchema")]
pub struct ContentTypeSchema {
    /// _
    /// Represents the following attribute in the schema: ct:_
    #[xml(attr = "ct:_")]
    pub under_score: Option<String>,
    /// _
    /// Represents the following attribute in the schema: ma:_
    #[xml(attr = "ma:_")]
    pub reserved_attribute_string: Option<String>,
    /// contentTypeName
    /// Represents the following attribute in the schema: ma:contentTypeName
    #[xml(attr = "ma:contentTypeName")]
    pub content_type_name: Option<String>,
    /// contentTypeID
    /// Represents the following attribute in the schema: ma:contentTypeID
    #[xml(attr = "ma:contentTypeID")]
    pub content_type_id: Option<String>,
    /// contentTypeVersion
    /// Represents the following attribute in the schema: ma:contentTypeVersion
    #[xml(attr = "ma:contentTypeVersion")]
    pub content_type_version: Option<i32>,
    /// contentTypeDescription
    /// Represents the following attribute in the schema: ma:contentTypeDescription
    #[xml(attr = "ma:contentTypeDescription")]
    pub content_type_description: Option<String>,
    /// contentTypeScope
    /// Represents the following attribute in the schema: ma:contentTypeScope
    #[xml(attr = "ma:contentTypeScope")]
    pub content_type_scope: Option<String>,
    /// versionID
    /// Represents the following attribute in the schema: ma:versionID
    #[xml(attr = "ma:versionID")]
    pub version_id: Option<String>,
}
