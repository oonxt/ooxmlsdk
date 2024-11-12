/// Defines the CommentV2MonikerList Class.
/// When the object is serialized out as xml, it's qualified name is pc2:cmMkLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "pc2:cmMkLst")]
pub struct CommentV2MonikerList {
    /// _
    #[xml(child = "pc:sldMkLst")]
    pub slide_moniker_list: crate::schemas::schemas_microsoft_com_office_powerpoint_2013_main_command::SlideMonikerList,
    /// _
    #[xml(child = "pc2:cmMK")]
    pub comment_v2_moniker: CommentV2Moniker,
}
/// Defines the CommentReplyV2MonikerList Class.
/// When the object is serialized out as xml, it's qualified name is pc2:cmRplyMkLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "pc2:cmRplyMkLst")]
pub struct CommentReplyV2MonikerList {
    /// _
    #[xml(child = "pc2:cmMkLst")]
    pub comment_v2_moniker_list: CommentV2MonikerList,
    /// _
    #[xml(child = "pc2:cmRplyMk")]
    pub comment_reply_v2_moniker: CommentReplyV2Moniker,
}
/// Defines the CommentV2Moniker Class.
/// When the object is serialized out as xml, it's qualified name is pc2:cmMK.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "pc2:cmMK")]
pub struct CommentV2Moniker {
    /// id
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: String,
}
/// Defines the CommentReplyV2Moniker Class.
/// When the object is serialized out as xml, it's qualified name is pc2:cmRplyMk.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "pc2:cmRplyMk")]
pub struct CommentReplyV2Moniker {
    /// id
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: String,
}
