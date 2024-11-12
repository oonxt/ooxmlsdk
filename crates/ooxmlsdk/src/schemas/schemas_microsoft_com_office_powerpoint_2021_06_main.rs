/// Defines the TaskHistoryDetails Class.
/// When the object is serialized out as xml, it's qualified name is p216:taskHistoryDetails.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p216:taskHistoryDetails")]
pub struct TaskHistoryDetails {
    /// id
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: String,
    /// _
    #[xml(child = "p216:history")]
    pub task_history: TaskHistory,
    /// _
    #[xml(child = "p216:extLst")]
    pub extension_list: Option<ExtensionList>,
}
/// Defines the CommentAnchor Class.
/// When the object is serialized out as xml, it's qualified name is p216:comment.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p216:comment")]
pub struct CommentAnchor {
    /// id
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: String,
}
/// Defines the ExtensionList Class.
/// When the object is serialized out as xml, it's qualified name is p216:extLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p216:extLst")]
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
/// When the object is serialized out as xml, it's qualified name is p216:atrbtn.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p216:atrbtn")]
pub struct AtrbtnTaskAssignUnassignUser {
    /// authorId
    /// Represents the following attribute in the schema: :authorId
    #[xml(attr = "authorId")]
    pub author_id: String,
}
/// Defines the AsgnTaskAssignUnassignUser Class.
/// When the object is serialized out as xml, it's qualified name is p216:asgn.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p216:asgn")]
pub struct AsgnTaskAssignUnassignUser {
    /// authorId
    /// Represents the following attribute in the schema: :authorId
    #[xml(attr = "authorId")]
    pub author_id: String,
}
/// Defines the UnAsgnTaskAssignUnassignUser Class.
/// When the object is serialized out as xml, it's qualified name is p216:unAsgn.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p216:unAsgn")]
pub struct UnAsgnTaskAssignUnassignUser {
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
/// When the object is serialized out as xml, it's qualified name is p216:anchr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p216:anchr")]
pub struct TaskAnchor {
    /// _
    #[xml(child = "p216:comment")]
    pub comment_anchor: CommentAnchor,
    /// _
    #[xml(child = "p216:extLst")]
    pub extension_list: Option<ExtensionList>,
}
/// Defines the AddEmpty Class.
/// When the object is serialized out as xml, it's qualified name is p216:add.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p216:add")]
pub struct AddEmpty {}
/// Defines the UnasgnAllEmpty Class.
/// When the object is serialized out as xml, it's qualified name is p216:unasgnAll.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p216:unasgnAll")]
pub struct UnasgnAllEmpty {}
/// Defines the EmptyType Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct EmptyType {}
/// Defines the TaskTitleEventInfo Class.
/// When the object is serialized out as xml, it's qualified name is p216:title.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p216:title")]
pub struct TaskTitleEventInfo {
    /// val
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: String,
}
/// Defines the TaskScheduleEventInfo Class.
/// When the object is serialized out as xml, it's qualified name is p216:date.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p216:date")]
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
/// When the object is serialized out as xml, it's qualified name is p216:pcntCmplt.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p216:pcntCmplt")]
pub struct TaskProgressEventInfo {
    /// val
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: i32,
}
/// Defines the TaskPriorityRecord Class.
/// When the object is serialized out as xml, it's qualified name is p216:pri.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p216:pri")]
pub struct TaskPriorityRecord {
    /// val
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: i32,
}
/// Defines the TaskUndo Class.
/// When the object is serialized out as xml, it's qualified name is p216:undo.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p216:undo")]
pub struct TaskUndo {
    /// id
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: String,
}
/// Defines the TaskUnknownRecord Class.
/// When the object is serialized out as xml, it's qualified name is p216:unknown.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p216:unknown")]
pub struct TaskUnknownRecord {}
/// Defines the TaskHistoryEvent Class.
/// When the object is serialized out as xml, it's qualified name is p216:event.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p216:event")]
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
        child = "p216:atrbtn",
        child = "p216:anchr",
        child = "p216:asgn",
        child = "p216:unAsgn",
        child = "p216:add",
        child = "p216:title",
        child = "p216:date",
        child = "p216:pcntCmplt",
        child = "p216:pri",
        child = "p216:unasgnAll",
        child = "p216:undo",
        child = "p216:unknown",
        child = "p216:extLst",
    )]
    pub children: Vec<TaskHistoryEventChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum TaskHistoryEventChildChoice {
    #[xml(tag = "p216:atrbtn")]
    P216Atrbtn(AtrbtnTaskAssignUnassignUser),
    #[xml(tag = "p216:anchr")]
    P216Anchr(TaskAnchor),
    #[xml(tag = "p216:asgn")]
    P216Asgn(AsgnTaskAssignUnassignUser),
    #[xml(tag = "p216:unAsgn")]
    P216UnAsgn(UnAsgnTaskAssignUnassignUser),
    #[xml(tag = "p216:add")]
    P216Add(AddEmpty),
    #[xml(tag = "p216:title")]
    P216Title(TaskTitleEventInfo),
    #[xml(tag = "p216:date")]
    P216Date(TaskScheduleEventInfo),
    #[xml(tag = "p216:pcntCmplt")]
    P216PcntCmplt(TaskProgressEventInfo),
    #[xml(tag = "p216:pri")]
    P216Pri(TaskPriorityRecord),
    #[xml(tag = "p216:unasgnAll")]
    P216UnasgnAll(UnasgnAllEmpty),
    #[xml(tag = "p216:undo")]
    P216Undo(TaskUndo),
    #[xml(tag = "p216:unknown")]
    P216Unknown(TaskUnknownRecord),
    #[xml(tag = "p216:extLst")]
    P216ExtLst(ExtensionList),
}
/// Defines the TaskHistory Class.
/// When the object is serialized out as xml, it's qualified name is p216:history.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p216:history")]
pub struct TaskHistory {
    /// _
    #[xml(child = "p216:event")]
    pub p216_event: Vec<TaskHistoryEvent>,
}
