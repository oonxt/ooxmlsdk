/// Defines the Tasks Class.
/// When the object is serialized out as xml, it's qualified name is t:Tasks.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "t:Tasks")]
pub struct Tasks {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// _
    #[xml(child = "t:Task")]
    pub t_task: Vec<Task>,
    /// _
    #[xml(child = "t:extLst")]
    pub t_ext_lst: Option<ExtensionList>,
}
/// Defines the Task Class.
/// When the object is serialized out as xml, it's qualified name is t:Task.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "t:Task")]
pub struct Task {
    /// id
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: String,
    /// _
    #[xml(child = "t:Anchor")]
    pub task_anchor: Option<TaskAnchor>,
    /// _
    #[xml(child = "t:History")]
    pub task_history: Option<TaskHistory>,
    /// _
    #[xml(child = "t:extLst")]
    pub extension_list: Option<ExtensionList>,
}
/// Defines the ExtensionList Class.
/// When the object is serialized out as xml, it's qualified name is t:extLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "t:extLst")]
pub struct ExtensionList {
    /// _
    #[xml(child = "oel:ext")]
    pub oel_ext: Vec<
        crate::schemas::schemas_microsoft_com_office_2019_extlst::Extension,
    >,
}
/// Defines the TaskAnchor Class.
/// When the object is serialized out as xml, it's qualified name is t:Anchor.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "t:Anchor")]
pub struct TaskAnchor {
    /// _
    #[xml(child = "t:Comment")]
    pub comment_anchor: Option<CommentAnchor>,
    /// _
    #[xml(child = "t:extLst")]
    pub extension_list: Option<ExtensionList>,
}
/// Defines the TaskHistory Class.
/// When the object is serialized out as xml, it's qualified name is t:History.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "t:History")]
pub struct TaskHistory {
    /// _
    #[xml(child = "t:Event")]
    pub t_event: Vec<TaskHistoryEvent>,
}
/// Defines the TaskHistoryEvent Class.
/// When the object is serialized out as xml, it's qualified name is t:Event.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "t:Event")]
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
        child = "t:Attribution",
        child = "t:Anchor",
        child = "t:Assign",
        child = "t:Unassign",
        child = "t:Create",
        child = "t:SetTitle",
        child = "t:Schedule",
        child = "t:Progress",
        child = "t:Priority",
        child = "t:Delete",
        child = "t:Undelete",
        child = "t:UnassignAll",
        child = "t:Undo",
        child = "t:extLst",
    )]
    pub children: Vec<TaskHistoryEventChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum TaskHistoryEventChildChoice {
    #[xml(tag = "t:Attribution")]
    TAttribution(AttributionTaskUser),
    #[xml(tag = "t:Anchor")]
    TAnchor(TaskAnchor),
    #[xml(tag = "t:Assign")]
    TAssign(AssignTaskUser),
    #[xml(tag = "t:Unassign")]
    TUnassign(UnassignTaskUser),
    #[xml(tag = "t:Create")]
    TCreate(TaskCreateEventInfo),
    #[xml(tag = "t:SetTitle")]
    TSetTitle(TaskTitleEventInfo),
    #[xml(tag = "t:Schedule")]
    TSchedule(TaskScheduleEventInfo),
    #[xml(tag = "t:Progress")]
    TProgress(TaskProgressEventInfo),
    #[xml(tag = "t:Priority")]
    TPriority(TaskPriorityEventInfo),
    #[xml(tag = "t:Delete")]
    TDelete(TaskDeleteEventInfo),
    #[xml(tag = "t:Undelete")]
    TUndelete(TaskUndeleteEventInfo),
    #[xml(tag = "t:UnassignAll")]
    TUnassignAll(TaskUnassignAll),
    #[xml(tag = "t:Undo")]
    TUndo(TaskUndo),
    #[xml(tag = "t:extLst")]
    TExtLst(ExtensionList),
}
/// Defines the AttributionTaskUser Class.
/// When the object is serialized out as xml, it's qualified name is t:Attribution.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "t:Attribution")]
pub struct AttributionTaskUser {
    /// userId
    /// Represents the following attribute in the schema: :userId
    #[xml(attr = "userId")]
    pub user_id: String,
    /// userName
    /// Represents the following attribute in the schema: :userName
    #[xml(attr = "userName")]
    pub user_name: String,
    /// userProvider
    /// Represents the following attribute in the schema: :userProvider
    #[xml(attr = "userProvider")]
    pub user_provider: String,
}
/// Defines the AssignTaskUser Class.
/// When the object is serialized out as xml, it's qualified name is t:Assign.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "t:Assign")]
pub struct AssignTaskUser {
    /// userId
    /// Represents the following attribute in the schema: :userId
    #[xml(attr = "userId")]
    pub user_id: String,
    /// userName
    /// Represents the following attribute in the schema: :userName
    #[xml(attr = "userName")]
    pub user_name: String,
    /// userProvider
    /// Represents the following attribute in the schema: :userProvider
    #[xml(attr = "userProvider")]
    pub user_provider: String,
}
/// Defines the UnassignTaskUser Class.
/// When the object is serialized out as xml, it's qualified name is t:Unassign.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "t:Unassign")]
pub struct UnassignTaskUser {
    /// userId
    /// Represents the following attribute in the schema: :userId
    #[xml(attr = "userId")]
    pub user_id: String,
    /// userName
    /// Represents the following attribute in the schema: :userName
    #[xml(attr = "userName")]
    pub user_name: String,
    /// userProvider
    /// Represents the following attribute in the schema: :userProvider
    #[xml(attr = "userProvider")]
    pub user_provider: String,
}
/// Defines the OpenXmlTaskUserElement Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct OpenXmlTaskUserElement {
    /// userId
    /// Represents the following attribute in the schema: :userId
    #[xml(attr = "userId")]
    pub user_id: String,
    /// userName
    /// Represents the following attribute in the schema: :userName
    #[xml(attr = "userName")]
    pub user_name: String,
    /// userProvider
    /// Represents the following attribute in the schema: :userProvider
    #[xml(attr = "userProvider")]
    pub user_provider: String,
}
/// Defines the TaskCreateEventInfo Class.
/// When the object is serialized out as xml, it's qualified name is t:Create.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "t:Create")]
pub struct TaskCreateEventInfo {}
/// Defines the TaskTitleEventInfo Class.
/// When the object is serialized out as xml, it's qualified name is t:SetTitle.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "t:SetTitle")]
pub struct TaskTitleEventInfo {
    /// title
    /// Represents the following attribute in the schema: :title
    #[xml(attr = "title")]
    pub title: String,
}
/// Defines the TaskScheduleEventInfo Class.
/// When the object is serialized out as xml, it's qualified name is t:Schedule.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "t:Schedule")]
pub struct TaskScheduleEventInfo {
    /// startDate
    /// Represents the following attribute in the schema: :startDate
    #[xml(attr = "startDate")]
    pub start_date: Option<String>,
    /// dueDate
    /// Represents the following attribute in the schema: :dueDate
    #[xml(attr = "dueDate")]
    pub due_date: Option<String>,
}
/// Defines the TaskProgressEventInfo Class.
/// When the object is serialized out as xml, it's qualified name is t:Progress.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "t:Progress")]
pub struct TaskProgressEventInfo {
    /// percentComplete
    /// Represents the following attribute in the schema: :percentComplete
    #[xml(attr = "percentComplete")]
    pub percent_complete: i32,
}
/// Defines the TaskPriorityEventInfo Class.
/// When the object is serialized out as xml, it's qualified name is t:Priority.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "t:Priority")]
pub struct TaskPriorityEventInfo {
    /// value
    /// Represents the following attribute in the schema: :value
    #[xml(attr = "value")]
    pub value: i32,
}
/// Defines the TaskDeleteEventInfo Class.
/// When the object is serialized out as xml, it's qualified name is t:Delete.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "t:Delete")]
pub struct TaskDeleteEventInfo {}
/// Defines the TaskUndeleteEventInfo Class.
/// When the object is serialized out as xml, it's qualified name is t:Undelete.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "t:Undelete")]
pub struct TaskUndeleteEventInfo {}
/// Defines the TaskUnassignAll Class.
/// When the object is serialized out as xml, it's qualified name is t:UnassignAll.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "t:UnassignAll")]
pub struct TaskUnassignAll {}
/// Defines the TaskUndo Class.
/// When the object is serialized out as xml, it's qualified name is t:Undo.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "t:Undo")]
pub struct TaskUndo {
    /// id
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: String,
}
/// Defines the CommentAnchor Class.
/// When the object is serialized out as xml, it's qualified name is t:Comment.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "t:Comment")]
pub struct CommentAnchor {
    /// id
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: String,
}
