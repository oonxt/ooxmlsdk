/// Defines the TaskDetails Class.
/// When the object is serialized out as xml, it's qualified name is p228:taskDetails.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p228:taskDetails")]
pub struct TaskDetails {
    /// deleted
    /// Represents the following attribute in the schema: :deleted
    #[xml(attr = "deleted")]
    pub deleted: Option<bool>,
    /// inactive
    /// Represents the following attribute in the schema: :inactive
    #[xml(attr = "inactive")]
    pub inactive: Option<bool>,
    /// _
    #[xml(child = "p228:history")]
    pub task_history: TaskHistory,
    /// _
    #[xml(child = "p228:extLst")]
    pub extension_list: Option<ExtensionList>,
}
/// Defines the CommentAnchor Class.
/// When the object is serialized out as xml, it's qualified name is p228:comment.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p228:comment")]
pub struct CommentAnchor {
    /// id
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: String,
}
/// Defines the ExtensionList Class.
/// When the object is serialized out as xml, it's qualified name is p228:extLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p228:extLst")]
pub struct ExtensionList {
    #[xml(child = "p:ext")]
    pub children: Vec<ExtensionListChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum ExtensionListChildChoice {
    #[xml(tag = "p:ext")]
    PExt(crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::Extension),
}
/// Defines the AtrbtnTaskAssignUnassignUser Class.
/// When the object is serialized out as xml, it's qualified name is p228:atrbtn.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p228:atrbtn")]
pub struct AtrbtnTaskAssignUnassignUser {
    /// authorId
    /// Represents the following attribute in the schema: :authorId
    #[xml(attr = "authorId")]
    pub author_id: String,
}
/// Defines the AsgnTaskAssignUnassignUser Class.
/// When the object is serialized out as xml, it's qualified name is p228:asgn.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p228:asgn")]
pub struct AsgnTaskAssignUnassignUser {
    /// authorId
    /// Represents the following attribute in the schema: :authorId
    #[xml(attr = "authorId")]
    pub author_id: String,
}
/// Defines the OpenXmlTaskAssignUnassignUserElement Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct OpenXmlTaskAssignUnassignUserElement {
    /// authorId
    /// Represents the following attribute in the schema: :authorId
    #[xml(attr = "authorId")]
    pub author_id: String,
}
/// Defines the TaskAnchor Class.
/// When the object is serialized out as xml, it's qualified name is p228:anchr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p228:anchr")]
pub struct TaskAnchor {
    /// _
    #[xml(child = "p228:comment")]
    pub comment_anchor: CommentAnchor,
    /// _
    #[xml(child = "p228:extLst")]
    pub extension_list: Option<ExtensionList>,
}
/// Defines the AddEmpty Class.
/// When the object is serialized out as xml, it's qualified name is p228:add.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p228:add")]
pub struct AddEmpty {}
/// Defines the UnasgnAllEmpty Class.
/// When the object is serialized out as xml, it's qualified name is p228:unasgnAll.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p228:unasgnAll")]
pub struct UnasgnAllEmpty {}
/// Defines the EmptyType Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct EmptyType {}
/// Defines the TaskTitleEventInfo Class.
/// When the object is serialized out as xml, it's qualified name is p228:title.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p228:title")]
pub struct TaskTitleEventInfo {
    /// val
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: String,
}
/// Defines the TaskScheduleEventInfo Class.
/// When the object is serialized out as xml, it's qualified name is p228:date.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p228:date")]
pub struct TaskScheduleEventInfo {
    /// stDt
    /// Represents the following attribute in the schema: :stDt
    #[xml(attr = "stDt")]
    pub st_dt: Option<String>,
    /// endDt
    /// Represents the following attribute in the schema: :endDt
    #[xml(attr = "endDt")]
    pub end_dt: Option<String>,
}
/// Defines the TaskProgressEventInfo Class.
/// When the object is serialized out as xml, it's qualified name is p228:pcntCmplt.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p228:pcntCmplt")]
pub struct TaskProgressEventInfo {
    /// val
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: i32,
}
/// Defines the TaskUndo Class.
/// When the object is serialized out as xml, it's qualified name is p228:undo.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p228:undo")]
pub struct TaskUndo {
    /// id
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: String,
}
/// Defines the TaskUnknownRecord Class.
/// When the object is serialized out as xml, it's qualified name is p228:unknown.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p228:unknown")]
pub struct TaskUnknownRecord {}
/// Defines the TaskHistoryEvent Class.
/// When the object is serialized out as xml, it's qualified name is p228:event.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p228:event")]
pub struct TaskHistoryEvent {
    /// time
    /// Represents the following attribute in the schema: :time
    #[xml(attr = "time")]
    pub time: String,
    /// id
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: String,
    #[xml(
        child = "p228:atrbtn",
        child = "p228:anchr",
        child = "p228:asgn",
        child = "p228:add",
        child = "p228:title",
        child = "p228:date",
        child = "p228:pcntCmplt",
        child = "p228:unasgnAll",
        child = "p228:undo",
        child = "p228:unknown",
        child = "p228:extLst",
    )]
    pub children: Vec<TaskHistoryEventChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum TaskHistoryEventChildChoice {
    #[xml(tag = "p228:atrbtn")]
    P228Atrbtn(AtrbtnTaskAssignUnassignUser),
    #[xml(tag = "p228:anchr")]
    P228Anchr(TaskAnchor),
    #[xml(tag = "p228:asgn")]
    P228Asgn(AsgnTaskAssignUnassignUser),
    #[xml(tag = "p228:add")]
    P228Add(AddEmpty),
    #[xml(tag = "p228:title")]
    P228Title(TaskTitleEventInfo),
    #[xml(tag = "p228:date")]
    P228Date(TaskScheduleEventInfo),
    #[xml(tag = "p228:pcntCmplt")]
    P228PcntCmplt(TaskProgressEventInfo),
    #[xml(tag = "p228:unasgnAll")]
    P228UnasgnAll(UnasgnAllEmpty),
    #[xml(tag = "p228:undo")]
    P228Undo(TaskUndo),
    #[xml(tag = "p228:unknown")]
    P228Unknown(TaskUnknownRecord),
    #[xml(tag = "p228:extLst")]
    P228ExtLst(ExtensionList),
}
/// Defines the TaskHistory Class.
/// When the object is serialized out as xml, it's qualified name is p228:history.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p228:history")]
pub struct TaskHistory {
    /// _
    #[xml(child = "p228:event")]
    pub p228_event: Vec<TaskHistoryEvent>,
}
