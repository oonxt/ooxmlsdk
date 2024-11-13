/// Defines the PersonList Class.
/// When the object is serialized out as xml, it's qualified name is xltc:personList.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xltc:personList")]
pub struct PersonList {
    #[xml(attr = "xmlns", with = "person_list_xmlns_derive")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// _
    #[xml(child = "xltc:person")]
    pub xltc_person: Vec<Person>,
    /// _
    #[xml(child = "xltc:extLst")]
    pub xltc_ext_lst: Option<ExtensionList>,
}
mod person_list_xmlns_derive {
    pub fn from_xml(mode: &str) -> hard_xml::XmlResult<String> {
        Ok(mode.to_string())
    }
    pub fn to_xml(_: &String) -> hard_xml::XmlResult<&'static str> {
        Ok("http://schemas.microsoft.com/office/spreadsheetml/2018/threadedcomments")
    }
}
/// Defines the ThreadedComments Class.
/// When the object is serialized out as xml, it's qualified name is xltc:ThreadedComments.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xltc:ThreadedComments")]
pub struct ThreadedComments {
    #[xml(attr = "xmlns", with = "threaded_comments_xmlns_derive")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// _
    #[xml(child = "xltc:threadedComment")]
    pub xltc_threaded_comment: Vec<ThreadedComment>,
    /// _
    #[xml(child = "xltc:extLst")]
    pub xltc_ext_lst: Option<ExtensionList>,
}
mod threaded_comments_xmlns_derive {
    pub fn from_xml(mode: &str) -> hard_xml::XmlResult<String> {
        Ok(mode.to_string())
    }
    pub fn to_xml(_: &String) -> hard_xml::XmlResult<&'static str> {
        Ok("http://schemas.microsoft.com/office/spreadsheetml/2018/threadedcomments")
    }
}
/// Defines the Person Class.
/// When the object is serialized out as xml, it's qualified name is xltc:person.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xltc:person")]
pub struct Person {
    /// displayName
    /// Represents the following attribute in the schema: :displayName
    #[xml(attr = "displayName")]
    pub display_name: String,
    /// id
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: String,
    /// userId
    /// Represents the following attribute in the schema: :userId
    #[xml(attr = "userId")]
    pub user_id: Option<String>,
    /// providerId
    /// Represents the following attribute in the schema: :providerId
    #[xml(attr = "providerId")]
    pub provider_id: Option<String>,
    /// _
    #[xml(child = "xltc:extLst")]
    pub extension_list: Option<ExtensionList>,
}
/// Defines the ExtensionList Class.
/// When the object is serialized out as xml, it's qualified name is xltc:extLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xltc:extLst")]
pub struct ExtensionList {
    #[xml(child = "x:ext")]
    pub children: Vec<ExtensionListChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum ExtensionListChildChoice {
    #[xml(tag = "x:ext")]
    XExt(crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Extension),
}
/// Defines the ThreadedComment Class.
/// When the object is serialized out as xml, it's qualified name is xltc:threadedComment.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xltc:threadedComment")]
pub struct ThreadedComment {
    /// ref
    /// Represents the following attribute in the schema: :ref
    #[xml(attr = "ref")]
    pub r#ref: Option<String>,
    /// dT
    /// Represents the following attribute in the schema: :dT
    #[xml(attr = "dT")]
    pub d_t: Option<String>,
    /// personId
    /// Represents the following attribute in the schema: :personId
    #[xml(attr = "personId")]
    pub person_id: String,
    /// id
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: String,
    /// parentId
    /// Represents the following attribute in the schema: :parentId
    #[xml(attr = "parentId")]
    pub parent_id: Option<String>,
    /// done
    /// Represents the following attribute in the schema: :done
    #[xml(attr = "done")]
    pub done: Option<bool>,
    /// _
    #[xml(child = "xltc:text")]
    pub threaded_comment_text: Option<ThreadedCommentText>,
    /// _
    #[xml(child = "xltc:mentions")]
    pub threaded_comment_mentions: Option<ThreadedCommentMentions>,
    /// _
    #[xml(child = "xltc:extLst")]
    pub extension_list: Option<ExtensionList>,
}
/// Defines the ThreadedCommentText Class.
/// When the object is serialized out as xml, it's qualified name is xltc:text.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xltc:text")]
pub struct ThreadedCommentText {
    #[xml(text)]
    pub child: String,
}
/// Defines the ThreadedCommentMentions Class.
/// When the object is serialized out as xml, it's qualified name is xltc:mentions.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xltc:mentions")]
pub struct ThreadedCommentMentions {
    /// _
    #[xml(child = "xltc:mention")]
    pub xltc_mention: Vec<Mention>,
}
/// Defines the Mention Class.
/// When the object is serialized out as xml, it's qualified name is xltc:mention.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xltc:mention")]
pub struct Mention {
    /// mentionpersonId
    /// Represents the following attribute in the schema: :mentionpersonId
    #[xml(attr = "mentionpersonId")]
    pub mentionperson_id: String,
    /// mentionId
    /// Represents the following attribute in the schema: :mentionId
    #[xml(attr = "mentionId")]
    pub mention_id: String,
    /// startIndex
    /// Represents the following attribute in the schema: :startIndex
    #[xml(attr = "startIndex")]
    pub start_index: i32,
    /// length
    /// Represents the following attribute in the schema: :length
    #[xml(attr = "length")]
    pub length: i32,
}
