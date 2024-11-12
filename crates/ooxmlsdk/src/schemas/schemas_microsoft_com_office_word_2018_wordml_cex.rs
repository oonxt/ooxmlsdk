/// Defines the CommentsExtensible Class.
/// When the object is serialized out as xml, it's qualified name is w16cex:commentsExtensible.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w16cex:commentsExtensible")]
pub struct CommentsExtensible {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// _
    #[xml(child = "w16cex:commentExtensible")]
    pub w16cex_comment_extensible: Vec<CommentExtensible>,
    /// _
    #[xml(child = "w16cex:extLst")]
    pub w16cex_ext_lst: Option<ExtensionList>,
}
/// Defines the CommentExtensible Class.
/// When the object is serialized out as xml, it's qualified name is w16cex:commentExtensible.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w16cex:commentExtensible")]
pub struct CommentExtensible {
    /// durableId
    /// Represents the following attribute in the schema: w16cex:durableId
    #[xml(attr = "w16cex:durableId")]
    pub w16cex_durable_id: String,
    /// dateUtc
    /// Represents the following attribute in the schema: w16cex:dateUtc
    #[xml(attr = "w16cex:dateUtc")]
    pub w16cex_date_utc: Option<String>,
    /// intelligentPlaceholder
    /// Represents the following attribute in the schema: w16cex:intelligentPlaceholder
    #[xml(attr = "w16cex:intelligentPlaceholder")]
    pub w16cex_intelligent_placeholder: Option<bool>,
    /// _
    #[xml(child = "w16cex:extLst")]
    pub extension_list: Option<ExtensionList>,
}
/// Defines the ExtensionList Class.
/// When the object is serialized out as xml, it's qualified name is w16cex:extLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w16cex:extLst")]
pub struct ExtensionList {
    /// _
    #[xml(child = "w16cur:ext")]
    pub w16cur_ext: Vec<
        crate::schemas::schemas_microsoft_com_office_word_2018_wordml::Extension,
    >,
}
