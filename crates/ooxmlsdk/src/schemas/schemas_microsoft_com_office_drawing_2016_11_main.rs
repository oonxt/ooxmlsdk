/// Defines the PictureAttributionSourceURL Class.
/// When the object is serialized out as xml, it's qualified name is a1611:picAttrSrcUrl.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a1611:picAttrSrcUrl")]
pub struct PictureAttributionSourceUrl {
    /// id
    /// Represents the following attribute in the schema: r:id
    #[xml(attr = "r:id")]
    pub r_id: String,
}
