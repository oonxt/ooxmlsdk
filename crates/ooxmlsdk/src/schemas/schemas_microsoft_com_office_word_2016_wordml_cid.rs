/// Defines the CommentsIds Class.
/// When the object is serialized out as xml, it's qualified name is w16cid:commentsIds.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w16cid:commentsIds")]
pub struct CommentsIds {
    #[xml(attr = "xmlns", with = "comments_ids_xmlns_derive")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// _
    #[xml(child = "w16cid:commentId")]
    pub w16cid_comment_id: Vec<CommentId>,
}
mod comments_ids_xmlns_derive {
    pub fn from_xml(mode: &str) -> hard_xml::XmlResult<String> {
        Ok(mode.to_string())
    }
    pub fn to_xml(_: &String) -> hard_xml::XmlResult<&'static str> {
        Ok("http://schemas.microsoft.com/office/word/2016/wordml/cid")
    }
}
/// Defines the CommentId Class.
/// When the object is serialized out as xml, it's qualified name is w16cid:commentId.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w16cid:commentId")]
pub struct CommentId {
    /// paraId
    /// Represents the following attribute in the schema: w16cid:paraId
    #[xml(attr = "w16cid:paraId")]
    pub w16cid_para_id: String,
    /// durableId
    /// Represents the following attribute in the schema: w16cid:durableId
    #[xml(attr = "w16cid:durableId")]
    pub w16cid_durable_id: String,
}
