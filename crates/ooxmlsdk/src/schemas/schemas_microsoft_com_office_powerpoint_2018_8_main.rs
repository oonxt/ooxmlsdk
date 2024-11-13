#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum CommentStatus {
    #[default]
    Active,
    Resolved,
    Closed,
}
crate::__string_enum! {
    CommentStatus { Active = "active", Resolved = "resolved", Closed = "closed", }
}
/// Defines the CommentUnknownAnchor Class.
/// When the object is serialized out as xml, it's qualified name is p188:unknownAnchor.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p188:unknownAnchor")]
pub struct CommentUnknownAnchor {}
/// Defines the TextBodyType Class.
/// When the object is serialized out as xml, it's qualified name is p188:txBody.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p188:txBody")]
pub struct TextBodyType {
    ///Body Properties
    #[xml(child = "a:bodyPr")]
    pub body_properties: crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::BodyProperties,
    ///Text List Styles
    #[xml(child = "a:lstStyle")]
    pub list_style: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ListStyle,
    >,
    /// _
    #[xml(child = "a:p")]
    pub a_p: Vec<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Paragraph,
    >,
}
/// Defines the CommentPropertiesExtensionList Class.
/// When the object is serialized out as xml, it's qualified name is p188:extLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p188:extLst")]
pub struct CommentPropertiesExtensionList {
    #[xml(child = "p:ext")]
    pub children: Vec<CommentPropertiesExtensionListChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum CommentPropertiesExtensionListChildChoice {
    #[xml(tag = "p:ext")]
    PExt(
        crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::CommentPropertiesExtension,
    ),
}
/// Defines the AuthorList Class.
/// When the object is serialized out as xml, it's qualified name is p188:authorLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p188:authorLst")]
pub struct AuthorList {
    #[xml(attr = "xmlns", with = "author_list_xmlns_derive")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// _
    #[xml(child = "p188:author")]
    pub p188_author: Vec<Author>,
}
mod author_list_xmlns_derive {
    pub fn from_xml(mode: &str) -> hard_xml::XmlResult<String> {
        Ok(mode.to_string())
    }
    pub fn to_xml(_: &String) -> hard_xml::XmlResult<&'static str> {
        Ok("http://schemas.microsoft.com/office/powerpoint/2018/8/main")
    }
}
/// Defines the CommentList Class.
/// When the object is serialized out as xml, it's qualified name is p188:cmLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p188:cmLst")]
pub struct CommentList {
    #[xml(attr = "xmlns", with = "comment_list_xmlns_derive")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// _
    #[xml(child = "p188:cm")]
    pub p188_cm: Vec<Comment>,
}
mod comment_list_xmlns_derive {
    pub fn from_xml(mode: &str) -> hard_xml::XmlResult<String> {
        Ok(mode.to_string())
    }
    pub fn to_xml(_: &String) -> hard_xml::XmlResult<&'static str> {
        Ok("http://schemas.microsoft.com/office/powerpoint/2018/8/main")
    }
}
/// Defines the CommentRelationship Class.
/// When the object is serialized out as xml, it's qualified name is p188:commentRel.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p188:commentRel")]
pub struct CommentRelationship {
    /// id
    /// Represents the following attribute in the schema: r:id
    #[xml(attr = "r:id")]
    pub r_id: String,
}
/// Defines the ExtensionList Class.
/// When the object is serialized out as xml, it's qualified name is p188:extLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p188:extLst")]
pub struct ExtensionList {
    #[xml(child = "p:ext")]
    pub children: Vec<ExtensionListChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum ExtensionListChildChoice {
    #[xml(tag = "p:ext")]
    PExt(crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::Extension),
}
/// Defines the Author Class.
/// When the object is serialized out as xml, it's qualified name is p188:author.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p188:author")]
pub struct Author {
    /// id
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: String,
    /// name
    /// Represents the following attribute in the schema: :name
    #[xml(attr = "name")]
    pub name: String,
    /// initials
    /// Represents the following attribute in the schema: :initials
    #[xml(attr = "initials")]
    pub initials: Option<String>,
    /// userId
    /// Represents the following attribute in the schema: :userId
    #[xml(attr = "userId")]
    pub user_id: String,
    /// providerId
    /// Represents the following attribute in the schema: :providerId
    #[xml(attr = "providerId")]
    pub provider_id: String,
    /// _
    #[xml(child = "p188:extLst")]
    pub extension_list: Option<ExtensionList>,
}
/// Defines the CommentReply Class.
/// When the object is serialized out as xml, it's qualified name is p188:reply.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p188:reply")]
pub struct CommentReply {
    /// id
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: String,
    /// authorId
    /// Represents the following attribute in the schema: :authorId
    #[xml(attr = "authorId")]
    pub author_id: String,
    /// status
    /// Represents the following attribute in the schema: :status
    #[xml(attr = "status")]
    pub status: Option<CommentStatus>,
    /// created
    /// Represents the following attribute in the schema: :created
    #[xml(attr = "created")]
    pub created: String,
    /// tags
    /// Represents the following attribute in the schema: :tags
    #[xml(attr = "tags")]
    pub tags: Option<String>,
    /// likes
    /// Represents the following attribute in the schema: :likes
    #[xml(attr = "likes")]
    pub likes: Option<String>,
    #[xml(child = "p188:txBody", child = "p188:extLst")]
    pub children: Vec<CommentReplyChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum CommentReplyChildChoice {
    #[xml(tag = "p188:txBody")]
    P188TxBody(TextBodyType),
    #[xml(tag = "p188:extLst")]
    P188ExtLst(CommentPropertiesExtensionList),
}
/// Defines the Point2DType Class.
/// When the object is serialized out as xml, it's qualified name is p188:pos.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p188:pos")]
pub struct Point2DType {
    /// X-Axis Coordinate
    /// Represents the following attribute in the schema: :x
    #[xml(attr = "x")]
    pub x: i32,
    /// Y-Axis Coordinate
    /// Represents the following attribute in the schema: :y
    #[xml(attr = "y")]
    pub y: i32,
}
/// Defines the CommentReplyList Class.
/// When the object is serialized out as xml, it's qualified name is p188:replyLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p188:replyLst")]
pub struct CommentReplyList {
    /// _
    #[xml(child = "p188:reply")]
    pub p188_reply: Vec<CommentReply>,
}
/// Defines the Comment Class.
/// When the object is serialized out as xml, it's qualified name is p188:cm.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p188:cm")]
pub struct Comment {
    /// id
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: String,
    /// authorId
    /// Represents the following attribute in the schema: :authorId
    #[xml(attr = "authorId")]
    pub author_id: String,
    /// status
    /// Represents the following attribute in the schema: :status
    #[xml(attr = "status")]
    pub status: Option<CommentStatus>,
    /// created
    /// Represents the following attribute in the schema: :created
    #[xml(attr = "created")]
    pub created: String,
    /// tags
    /// Represents the following attribute in the schema: :tags
    #[xml(attr = "tags")]
    pub tags: Option<String>,
    /// likes
    /// Represents the following attribute in the schema: :likes
    #[xml(attr = "likes")]
    pub likes: Option<String>,
    /// startDate
    /// Represents the following attribute in the schema: :startDate
    #[xml(attr = "startDate")]
    pub start_date: Option<String>,
    /// dueDate
    /// Represents the following attribute in the schema: :dueDate
    #[xml(attr = "dueDate")]
    pub due_date: Option<String>,
    /// assignedTo
    /// Represents the following attribute in the schema: :assignedTo
    #[xml(attr = "assignedTo")]
    pub assigned_to: Option<String>,
    /// complete
    /// Represents the following attribute in the schema: :complete
    #[xml(attr = "complete")]
    pub complete: Option<i32>,
    /// priority
    /// Represents the following attribute in the schema: :priority
    #[xml(attr = "priority")]
    pub priority: Option<u32>,
    /// title
    /// Represents the following attribute in the schema: :title
    #[xml(attr = "title")]
    pub title: Option<String>,
    #[xml(
        child = "pc:sldMkLst",
        child = "pc:sldLayoutMkLst",
        child = "pc:sldMasterMkLst",
        child = "oac:deMkLst",
        child = "oac:txBodyMkLst",
        child = "oac:txMkLst",
        child = "oac:tcMkLst",
        child = "oac:trMkLst",
        child = "oac:gridColMkLst",
        child = "p188:unknownAnchor",
        child = "p188:pos",
        child = "p188:replyLst",
        child = "p188:txBody",
        child = "p188:extLst",
    )]
    pub children: Vec<CommentChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum CommentChildChoice {
    #[xml(tag = "pc:sldMkLst")]
    PcSldMkLst(
        crate::schemas::schemas_microsoft_com_office_powerpoint_2013_main_command::SlideMonikerList,
    ),
    #[xml(tag = "pc:sldLayoutMkLst")]
    PcSldLayoutMkLst(
        crate::schemas::schemas_microsoft_com_office_powerpoint_2013_main_command::SlideLayoutMonikerList,
    ),
    #[xml(tag = "pc:sldMasterMkLst")]
    PcSldMasterMkLst(
        crate::schemas::schemas_microsoft_com_office_powerpoint_2013_main_command::MainMasterMonikerList,
    ),
    #[xml(tag = "oac:deMkLst")]
    OacDeMkLst(
        crate::schemas::schemas_microsoft_com_office_drawing_2013_main_command::DeMkLstDrawingElementMonikerList,
    ),
    #[xml(tag = "oac:txBodyMkLst")]
    OacTxBodyMkLst(
        crate::schemas::schemas_microsoft_com_office_drawing_2013_main_command::TextBodyMonikerList,
    ),
    #[xml(tag = "oac:txMkLst")]
    OacTxMkLst(
        crate::schemas::schemas_microsoft_com_office_drawing_2013_main_command::TextCharRangeMonikerList,
    ),
    #[xml(tag = "oac:tcMkLst")]
    OacTcMkLst(
        crate::schemas::schemas_microsoft_com_office_drawing_2013_main_command::TableCellMonikerList,
    ),
    #[xml(tag = "oac:trMkLst")]
    OacTrMkLst(
        crate::schemas::schemas_microsoft_com_office_drawing_2013_main_command::TableRowMonikerList,
    ),
    #[xml(tag = "oac:gridColMkLst")]
    OacGridColMkLst(
        crate::schemas::schemas_microsoft_com_office_drawing_2013_main_command::TableColumnMonikerList,
    ),
    #[xml(tag = "p188:unknownAnchor")]
    P188UnknownAnchor(CommentUnknownAnchor),
    #[xml(tag = "p188:pos")]
    P188Pos(Point2DType),
    #[xml(tag = "p188:replyLst")]
    P188ReplyLst(CommentReplyList),
    #[xml(tag = "p188:txBody")]
    P188TxBody(TextBodyType),
    #[xml(tag = "p188:extLst")]
    P188ExtLst(CommentPropertiesExtensionList),
}
