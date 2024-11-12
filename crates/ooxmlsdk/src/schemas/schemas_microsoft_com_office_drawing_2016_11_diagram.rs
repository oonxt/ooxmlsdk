#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum STorageType {
    #[default]
    SibTrans,
    ParTrans,
}
crate::__string_enum! {
    STorageType { SibTrans = "sibTrans", ParTrans = "parTrans", }
}
/// Defines the NumberDiagramInfoList Class.
/// When the object is serialized out as xml, it's qualified name is dgm1611:autoBuNodeInfoLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "dgm1611:autoBuNodeInfoLst")]
pub struct NumberDiagramInfoList {
    /// _
    #[xml(child = "dgm1611:autoBuNodeInfo")]
    pub dgm1611_auto_bu_node_info: Vec<NumberDiagramInfo>,
}
/// Defines the DiagramAutoBullet Class.
/// When the object is serialized out as xml, it's qualified name is dgm1611:buPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "dgm1611:buPr")]
pub struct DiagramAutoBullet {
    /// prefix
    /// Represents the following attribute in the schema: :prefix
    #[xml(attr = "prefix")]
    pub auto_bullet_prefix: Option<String>,
    /// leadZeros
    /// Represents the following attribute in the schema: :leadZeros
    #[xml(attr = "leadZeros")]
    pub lead_zeros: Option<bool>,
    #[xml(
        child = "a:buNone",
        child = "a:buAutoNum",
        child = "a:buChar",
        child = "a:buBlip",
    )]
    pub children: Vec<DiagramAutoBulletChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum DiagramAutoBulletChildChoice {
    #[xml(tag = "a:buNone")]
    ABuNone(crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::NoBullet),
    #[xml(tag = "a:buAutoNum")]
    ABuAutoNum(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::AutoNumberedBullet,
    ),
    #[xml(tag = "a:buChar")]
    ABuChar(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::CharacterBullet,
    ),
    #[xml(tag = "a:buBlip")]
    ABuBlip(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::PictureBullet,
    ),
}
/// Defines the NumberDiagramInfo Class.
/// When the object is serialized out as xml, it's qualified name is dgm1611:autoBuNodeInfo.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "dgm1611:autoBuNodeInfo")]
pub struct NumberDiagramInfo {
    /// lvl
    /// Represents the following attribute in the schema: :lvl
    #[xml(attr = "lvl")]
    pub lvl: i32,
    /// ptType
    /// Represents the following attribute in the schema: :ptType
    #[xml(attr = "ptType")]
    pub pt_type: STorageType,
    /// _
    #[xml(child = "dgm1611:buPr")]
    pub diagram_auto_bullet: DiagramAutoBullet,
}
