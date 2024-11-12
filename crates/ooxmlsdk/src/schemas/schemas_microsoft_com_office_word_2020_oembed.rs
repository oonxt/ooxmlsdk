/// Defines the OEmbed Class.
/// When the object is serialized out as xml, it's qualified name is woe:oembed.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "woe:oembed")]
pub struct OEmbed {
    /// oEmbedUrl
    /// Represents the following attribute in the schema: :oEmbedUrl
    #[xml(attr = "oEmbedUrl")]
    pub o_embed_url: String,
    /// mediaType
    /// Represents the following attribute in the schema: :mediaType
    #[xml(attr = "mediaType")]
    pub media_type: String,
    /// picLocksAutoForOEmbed
    /// Represents the following attribute in the schema: :picLocksAutoForOEmbed
    #[xml(attr = "picLocksAutoForOEmbed")]
    pub pic_locks_auto_for_o_embed: Option<bool>,
}
