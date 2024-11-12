/// Defines the MultiLvlStrData Class.
/// When the object is serialized out as xml, it's qualified name is c16ac:multiLvlStrLit.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c16ac:multiLvlStrLit")]
pub struct MultiLvlStrData {
    /// _
    #[xml(child = "c:ptCount")]
    pub point_count: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::PointCount,
    >,
    /// _
    #[xml(child = "c:lvl")]
    pub c_lvl: Vec<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::Level,
    >,
    /// _
    #[xml(child = "c:extLst")]
    pub c_ext_lst: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::ExtensionList,
    >,
}
