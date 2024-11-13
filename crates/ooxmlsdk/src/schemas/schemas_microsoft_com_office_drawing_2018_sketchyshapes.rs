/// Defines the LineSketchNoneEmpty Class.
/// When the object is serialized out as xml, it's qualified name is ask:lineSketchNone.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "ask:lineSketchNone")]
pub struct LineSketchNoneEmpty {}
/// Defines the LineSketchCurvedEmpty Class.
/// When the object is serialized out as xml, it's qualified name is ask:lineSketchCurved.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "ask:lineSketchCurved")]
pub struct LineSketchCurvedEmpty {}
/// Defines the LineSketchFreehandEmpty Class.
/// When the object is serialized out as xml, it's qualified name is ask:lineSketchFreehand.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "ask:lineSketchFreehand")]
pub struct LineSketchFreehandEmpty {}
/// Defines the LineSketchScribbleEmpty Class.
/// When the object is serialized out as xml, it's qualified name is ask:lineSketchScribble.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "ask:lineSketchScribble")]
pub struct LineSketchScribbleEmpty {}
/// Defines the OpenXmlEmptyElement Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct OpenXmlEmptyElement {}
/// Defines the LineSketchStyleProperties Class.
/// When the object is serialized out as xml, it's qualified name is ask:lineSketchStyleProps.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "ask:lineSketchStyleProps")]
pub struct LineSketchStyleProperties {
    /// sd
    /// Represents the following attribute in the schema: :sd
    #[xml(attr = "sd")]
    pub sd: Option<u32>,
    #[xml(
        child = "a:custGeom",
        child = "a:prstGeom",
        child = "ask:type",
        child = "ask:seed",
        child = "ask:extLst",
    )]
    pub children: Vec<LineSketchStylePropertiesChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum LineSketchStylePropertiesChildChoice {
    #[xml(tag = "a:custGeom")]
    ACustGeom(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::CustomGeometry,
    ),
    #[xml(tag = "a:prstGeom")]
    APrstGeom(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::PresetGeometry,
    ),
    #[xml(tag = "ask:type")]
    AskType(LineSketchTypeProperties),
    #[xml(tag = "ask:seed")]
    AskSeed(LineSketchSeed),
    #[xml(tag = "ask:extLst")]
    AskExtLst(OfficeArtExtensionList),
}
/// Defines the LineSketchTypeProperties Class.
/// When the object is serialized out as xml, it's qualified name is ask:type.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "ask:type")]
pub struct LineSketchTypeProperties {
    #[xml(
        child = "ask:lineSketchNone",
        child = "ask:lineSketchCurved",
        child = "ask:lineSketchFreehand",
        child = "ask:lineSketchScribble",
    )]
    pub children: Vec<LineSketchTypePropertiesChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum LineSketchTypePropertiesChildChoice {
    #[xml(tag = "ask:lineSketchNone")]
    AskLineSketchNone(LineSketchNoneEmpty),
    #[xml(tag = "ask:lineSketchCurved")]
    AskLineSketchCurved(LineSketchCurvedEmpty),
    #[xml(tag = "ask:lineSketchFreehand")]
    AskLineSketchFreehand(LineSketchFreehandEmpty),
    #[xml(tag = "ask:lineSketchScribble")]
    AskLineSketchScribble(LineSketchScribbleEmpty),
}
/// Defines the LineSketchSeed Class.
/// When the object is serialized out as xml, it's qualified name is ask:seed.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "ask:seed")]
pub struct LineSketchSeed {
    #[xml(text)]
    pub child: u32,
}
/// Defines the OfficeArtExtensionList Class.
/// When the object is serialized out as xml, it's qualified name is ask:extLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "ask:extLst")]
pub struct OfficeArtExtensionList {
    #[xml(child = "a:ext")]
    pub children: Vec<OfficeArtExtensionListChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum OfficeArtExtensionListChildChoice {
    #[xml(tag = "a:ext")]
    AExt(crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Extension),
}
