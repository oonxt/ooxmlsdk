/// Defines the TaskHistoryDetails Class.
/// When the object is serialized out as xml, it's qualified name is p1912:taskHistoryDetails.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p1912:taskHistoryDetails")]
pub struct TaskHistoryDetails {
    /// id
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: String,
    /// _
    #[xml(child = "p1912:history")]
    pub task_history: TaskHistory,
    /// _
    #[xml(child = "p1912:extLst")]
    pub extension_list: Option<ExtensionList>,
}
/// Defines the CommentAnchor Class.
/// When the object is serialized out as xml, it's qualified name is p1912:comment.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p1912:comment")]
pub struct CommentAnchor {
    /// id
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: String,
}
/// Defines the ExtensionList Class.
/// When the object is serialized out as xml, it's qualified name is p1912:extLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p1912:extLst")]
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
/// When the object is serialized out as xml, it's qualified name is p1912:atrbtn.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p1912:atrbtn")]
pub struct AtrbtnTaskAssignUnassignUser {
    /// authorId
    /// Represents the following attribute in the schema: :authorId
    #[xml(attr = "authorId")]
    pub author_id: String,
}
/// Defines the AsgnTaskAssignUnassignUser Class.
/// When the object is serialized out as xml, it's qualified name is p1912:asgn.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p1912:asgn")]
pub struct AsgnTaskAssignUnassignUser {
    /// authorId
    /// Represents the following attribute in the schema: :authorId
    #[xml(attr = "authorId")]
    pub author_id: String,
}
/// Defines the UnAsgnTaskAssignUnassignUser Class.
/// When the object is serialized out as xml, it's qualified name is p1912:unAsgn.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p1912:unAsgn")]
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
/// When the object is serialized out as xml, it's qualified name is p1912:anchr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p1912:anchr")]
pub struct TaskAnchor {
    /// _
    #[xml(child = "p1912:comment")]
    pub comment_anchor: CommentAnchor,
    /// _
    #[xml(child = "p1912:extLst")]
    pub extension_list: Option<ExtensionList>,
}
/// Defines the AddEmpty Class.
/// When the object is serialized out as xml, it's qualified name is p1912:add.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p1912:add")]
pub struct AddEmpty {}
/// Defines the UnasgnAllEmpty Class.
/// When the object is serialized out as xml, it's qualified name is p1912:unasgnAll.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p1912:unasgnAll")]
pub struct UnasgnAllEmpty {}
/// Defines the EmptyType Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct EmptyType {}
/// Defines the TaskTitleEventInfo Class.
/// When the object is serialized out as xml, it's qualified name is p1912:title.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p1912:title")]
pub struct TaskTitleEventInfo {
    /// val
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: String,
}
/// Defines the TaskScheduleEventInfo Class.
/// When the object is serialized out as xml, it's qualified name is p1912:date.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p1912:date")]
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
/// When the object is serialized out as xml, it's qualified name is p1912:pcntCmplt.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p1912:pcntCmplt")]
pub struct TaskProgressEventInfo {
    /// val
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: i32,
}
/// Defines the TaskPriorityRecord Class.
/// When the object is serialized out as xml, it's qualified name is p1912:pri.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p1912:pri")]
pub struct TaskPriorityRecord {
    /// val
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: i32,
}
/// Defines the TaskUndo Class.
/// When the object is serialized out as xml, it's qualified name is p1912:undo.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p1912:undo")]
pub struct TaskUndo {
    /// id
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: String,
}
/// Defines the TaskUnknownRecord Class.
/// When the object is serialized out as xml, it's qualified name is p1912:unknown.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p1912:unknown")]
pub struct TaskUnknownRecord {}
/// Defines the TaskHistoryEvent Class.
/// When the object is serialized out as xml, it's qualified name is p1912:event.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p1912:event")]
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
        child = "p1912:atrbtn",
        child = "p1912:anchr",
        child = "p1912:asgn",
        child = "p1912:unAsgn",
        child = "p1912:add",
        child = "p1912:title",
        child = "p1912:date",
        child = "p1912:pcntCmplt",
        child = "p1912:pri",
        child = "p1912:unasgnAll",
        child = "p1912:undo",
        child = "p1912:unknown",
        child = "p1912:extLst",
    )]
    pub children: Vec<TaskHistoryEventChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum TaskHistoryEventChildChoice {
    #[xml(tag = "p1912:atrbtn")]
    P1912Atrbtn(AtrbtnTaskAssignUnassignUser),
    #[xml(tag = "p1912:anchr")]
    P1912Anchr(TaskAnchor),
    #[xml(tag = "p1912:asgn")]
    P1912Asgn(AsgnTaskAssignUnassignUser),
    #[xml(tag = "p1912:unAsgn")]
    P1912UnAsgn(UnAsgnTaskAssignUnassignUser),
    #[xml(tag = "p1912:add")]
    P1912Add(AddEmpty),
    #[xml(tag = "p1912:title")]
    P1912Title(TaskTitleEventInfo),
    #[xml(tag = "p1912:date")]
    P1912Date(TaskScheduleEventInfo),
    #[xml(tag = "p1912:pcntCmplt")]
    P1912PcntCmplt(TaskProgressEventInfo),
    #[xml(tag = "p1912:pri")]
    P1912Pri(TaskPriorityRecord),
    #[xml(tag = "p1912:unasgnAll")]
    P1912UnasgnAll(UnasgnAllEmpty),
    #[xml(tag = "p1912:undo")]
    P1912Undo(TaskUndo),
    #[xml(tag = "p1912:unknown")]
    P1912Unknown(TaskUnknownRecord),
    #[xml(tag = "p1912:extLst")]
    P1912ExtLst(ExtensionList),
}
/// Defines the TaskHistory Class.
/// When the object is serialized out as xml, it's qualified name is p1912:history.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p1912:history")]
pub struct TaskHistory {
    /// _
    #[xml(child = "p1912:event")]
    pub p1912_event: Vec<TaskHistoryEvent>,
}
