/// Locked Canvas Container.
/// When the object is serialized out as xml, it's qualified name is lc:lockedCanvas.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "lc:lockedCanvas")]
pub struct LockedCanvas {
    #[xml(
        child = "a:nvGrpSpPr",
        child = "a:grpSpPr",
        child = "a:txSp",
        child = "a:sp",
        child = "a:cxnSp",
        child = "a:pic",
        child = "a14:contentPart",
        child = "a:graphicFrame",
        child = "a:grpSp",
        child = "a:extLst",
    )]
    pub children: Vec<LockedCanvasChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum LockedCanvasChildChoice {
    #[xml(tag = "a:nvGrpSpPr")]
    ANvGrpSpPr(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::NonVisualGroupShapeProperties,
    ),
    #[xml(tag = "a:grpSpPr")]
    AGrpSpPr(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::VisualGroupShapeProperties,
    ),
    #[xml(tag = "a:txSp")]
    ATxSp(crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::TextShape),
    #[xml(tag = "a:sp")]
    ASp(crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Shape),
    #[xml(tag = "a:cxnSp")]
    ACxnSp(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ConnectionShape,
    ),
    #[xml(tag = "a:pic")]
    APic(crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Picture),
    #[xml(tag = "a14:contentPart")]
    A14ContentPart(
        crate::schemas::schemas_microsoft_com_office_drawing_2010_main::GvmlContentPart,
    ),
    #[xml(tag = "a:graphicFrame")]
    AGraphicFrame(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::GraphicFrame,
    ),
    #[xml(tag = "a:grpSp")]
    AGrpSp(crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::GroupShape),
    #[xml(tag = "a:extLst")]
    AExtLst(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::GvmlGroupShapeExtensionList,
    ),
}
