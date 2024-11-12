/// Defines the WordprocessingCanvas Class.
/// When the object is serialized out as xml, it's qualified name is wpc:wpc.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "wpc:wpc")]
pub struct WordprocessingCanvas {
    #[xml(
        child = "wpc:bg",
        child = "wpc:whole",
        child = "wps:wsp",
        child = "pic:pic",
        child = "w14:contentPart",
        child = "wpg:wgp",
        child = "wpc:graphicFrame",
        child = "wpc:extLst",
    )]
    pub children: Vec<WordprocessingCanvasChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum WordprocessingCanvasChildChoice {
    #[xml(tag = "wpc:bg")]
    WpcBg(BackgroundFormatting),
    #[xml(tag = "wpc:whole")]
    WpcWhole(WholeFormatting),
    #[xml(tag = "wps:wsp")]
    WpsWsp(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordprocessing_shape::WordprocessingShape,
    ),
    #[xml(tag = "pic:pic")]
    PicPic(crate::schemas::schemas_openxmlformats_org_drawingml_2006_picture::Picture),
    #[xml(tag = "w14:contentPart")]
    W14ContentPart(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::ContentPart,
    ),
    #[xml(tag = "wpg:wgp")]
    WpgWgp(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordprocessing_group::WordprocessingGroup,
    ),
    #[xml(tag = "wpc:graphicFrame")]
    WpcGraphicFrame(GraphicFrameType),
    #[xml(tag = "wpc:extLst")]
    WpcExtLst(OfficeArtExtensionList),
}
/// Defines the BackgroundFormatting Class.
/// When the object is serialized out as xml, it's qualified name is wpc:bg.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "wpc:bg")]
pub struct BackgroundFormatting {
    #[xml(
        child = "a:noFill",
        child = "a:solidFill",
        child = "a:gradFill",
        child = "a:blipFill",
        child = "a:pattFill",
        child = "a:grpFill",
        child = "a:effectLst",
        child = "a:effectDag",
    )]
    pub children: Vec<BackgroundFormattingChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum BackgroundFormattingChildChoice {
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
    #[xml(tag = "a:effectLst")]
    AEffectLst(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::EffectList,
    ),
    #[xml(tag = "a:effectDag")]
    AEffectDag(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::EffectDag,
    ),
}
/// Defines the WholeFormatting Class.
/// When the object is serialized out as xml, it's qualified name is wpc:whole.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "wpc:whole")]
pub struct WholeFormatting {
    #[xml(child = "a:ln", child = "a:effectLst", child = "a:effectDag")]
    pub children: Vec<WholeFormattingChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum WholeFormattingChildChoice {
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
}
/// Defines the GraphicFrameType Class.
/// When the object is serialized out as xml, it's qualified name is wpc:graphicFrame.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "wpc:graphicFrame")]
pub struct GraphicFrameType {
    /// _
    #[xml(child = "wpg:cNvPr")]
    pub non_visual_drawing_properties: crate::schemas::schemas_microsoft_com_office_word_2010_wordprocessing_group::NonVisualDrawingProperties,
    /// _
    #[xml(child = "wpg:cNvFrPr")]
    pub non_visual_graphic_frame_properties: crate::schemas::schemas_microsoft_com_office_word_2010_wordprocessing_group::NonVisualGraphicFrameProperties,
    /// _
    #[xml(child = "wpg:xfrm")]
    pub transform2_d: crate::schemas::schemas_microsoft_com_office_word_2010_wordprocessing_group::Transform2D,
    /// _
    #[xml(child = "a:graphic")]
    pub graphic: crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Graphic,
    /// _
    #[xml(child = "wpg:extLst")]
    pub office_art_extension_list: Option<
        crate::schemas::schemas_microsoft_com_office_word_2010_wordprocessing_group::OfficeArtExtensionList,
    >,
}
/// Defines the OfficeArtExtensionList Class.
/// When the object is serialized out as xml, it's qualified name is wpc:extLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "wpc:extLst")]
pub struct OfficeArtExtensionList {
    #[xml(child = "a:ext")]
    pub children: Vec<OfficeArtExtensionListChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum OfficeArtExtensionListChildChoice {
    #[xml(tag = "a:ext")]
    AExt(crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Extension),
}
