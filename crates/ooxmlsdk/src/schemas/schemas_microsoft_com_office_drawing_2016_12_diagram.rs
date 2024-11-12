/// Defines the ShapeProperties Class.
/// When the object is serialized out as xml, it's qualified name is dgm1612:spPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "dgm1612:spPr")]
pub struct ShapeProperties {
    /// Black and White Mode
    /// Represents the following attribute in the schema: :bwMode
    #[xml(attr = "bwMode")]
    pub black_white_mode: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::BlackWhiteModeValues,
    >,
    #[xml(
        child = "a:xfrm",
        child = "a:custGeom",
        child = "a:prstGeom",
        child = "a:noFill",
        child = "a:solidFill",
        child = "a:gradFill",
        child = "a:blipFill",
        child = "a:pattFill",
        child = "a:grpFill",
        child = "a:ln",
        child = "a:effectLst",
        child = "a:effectDag",
        child = "a:scene3d",
        child = "a:sp3d",
        child = "a:extLst",
    )]
    pub children: Vec<ShapePropertiesChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum ShapePropertiesChildChoice {
    #[xml(tag = "a:xfrm")]
    AXfrm(crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Transform2D),
    #[xml(tag = "a:custGeom")]
    ACustGeom(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::CustomGeometry,
    ),
    #[xml(tag = "a:prstGeom")]
    APrstGeom(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::PresetGeometry,
    ),
    #[xml(tag = "a:noFill")]
    ANoFill(crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::NoFill),
    #[xml(tag = "a:solidFill")]
    ASolidFill(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::SolidFill,
    ),
    #[xml(tag = "a:gradFill")]
    AGradFill(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::GradientFill,
    ),
    #[xml(tag = "a:blipFill")]
    ABlipFill(crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::BlipFill),
    #[xml(tag = "a:pattFill")]
    APattFill(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::PatternFill,
    ),
    #[xml(tag = "a:grpFill")]
    AGrpFill(crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::GroupFill),
    #[xml(tag = "a:ln")]
    ALn(crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Outline),
    #[xml(tag = "a:effectLst")]
    AEffectLst(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::EffectList,
    ),
    #[xml(tag = "a:effectDag")]
    AEffectDag(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::EffectDag,
    ),
    #[xml(tag = "a:scene3d")]
    AScene3d(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Scene3DType,
    ),
    #[xml(tag = "a:sp3d")]
    ASp3d(crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Shape3DType),
    #[xml(tag = "a:extLst")]
    AExtLst(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ShapePropertiesExtensionList,
    ),
}
/// Defines the TextListStyleType Class.
/// When the object is serialized out as xml, it's qualified name is dgm1612:lstStyle.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "dgm1612:lstStyle")]
pub struct TextListStyleType {
    ///Default Paragraph Style
    #[xml(child = "a:defPPr")]
    pub default_paragraph_properties: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::DefaultParagraphProperties,
    >,
    ///List Level 1 Text Style
    #[xml(child = "a:lvl1pPr")]
    pub level1_paragraph_properties: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Level1ParagraphProperties,
    >,
    ///List Level 2 Text Style
    #[xml(child = "a:lvl2pPr")]
    pub level2_paragraph_properties: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Level2ParagraphProperties,
    >,
    ///List Level 3 Text Style
    #[xml(child = "a:lvl3pPr")]
    pub level3_paragraph_properties: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Level3ParagraphProperties,
    >,
    ///List Level 4 Text Style
    #[xml(child = "a:lvl4pPr")]
    pub level4_paragraph_properties: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Level4ParagraphProperties,
    >,
    ///List Level 5 Text Style
    #[xml(child = "a:lvl5pPr")]
    pub level5_paragraph_properties: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Level5ParagraphProperties,
    >,
    ///List Level 6 Text Style
    #[xml(child = "a:lvl6pPr")]
    pub level6_paragraph_properties: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Level6ParagraphProperties,
    >,
    ///List Level 7 Text Style
    #[xml(child = "a:lvl7pPr")]
    pub level7_paragraph_properties: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Level7ParagraphProperties,
    >,
    ///List Level 8 Text Style
    #[xml(child = "a:lvl8pPr")]
    pub level8_paragraph_properties: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Level8ParagraphProperties,
    >,
    ///List Level 9 Text Style
    #[xml(child = "a:lvl9pPr")]
    pub level9_paragraph_properties: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Level9ParagraphProperties,
    >,
    /// _
    #[xml(child = "a:extLst")]
    pub extension_list: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ExtensionList,
    >,
}
