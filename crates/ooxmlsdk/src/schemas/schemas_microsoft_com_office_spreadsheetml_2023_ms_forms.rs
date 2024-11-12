/// Defines the Question Class.
/// When the object is serialized out as xml, it's qualified name is xlmsforms:question.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xlmsforms:question")]
pub struct Question {
    /// id
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: String,
    /// _
    #[xml(child = "xlmsforms:extLst")]
    pub extension_list: Option<ExtensionList>,
}
/// Defines the MsForm Class.
/// When the object is serialized out as xml, it's qualified name is xlmsforms:msForm.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xlmsforms:msForm")]
pub struct MsForm {
    /// id
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: String,
    /// isFormConnected
    /// Represents the following attribute in the schema: :isFormConnected
    #[xml(attr = "isFormConnected")]
    pub is_form_connected: Option<bool>,
    /// maxResponseId
    /// Represents the following attribute in the schema: :maxResponseId
    #[xml(attr = "maxResponseId")]
    pub max_response_id: Option<i32>,
    /// latestEventMarker
    /// Represents the following attribute in the schema: :latestEventMarker
    #[xml(attr = "latestEventMarker")]
    pub latest_event_marker: Option<String>,
    /// _
    #[xml(child = "xlmsforms:syncedQuestionId")]
    pub xlmsforms_synced_question_id: Vec<SyncedQuestionId>,
    /// _
    #[xml(child = "xlmsforms:extLst")]
    pub xlmsforms_ext_lst: Option<ExtensionList>,
}
/// Defines the SyncedQuestionId Class.
/// When the object is serialized out as xml, it's qualified name is xlmsforms:syncedQuestionId.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xlmsforms:syncedQuestionId")]
pub struct SyncedQuestionId {
    #[xml(text)]
    pub child: String,
}
/// Defines the ExtensionList Class.
/// When the object is serialized out as xml, it's qualified name is xlmsforms:extLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xlmsforms:extLst")]
pub struct ExtensionList {
    #[xml(child = "x:ext")]
    pub children: Vec<ExtensionListChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum ExtensionListChildChoice {
    #[xml(tag = "x:ext")]
    XExt(crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Extension),
}
