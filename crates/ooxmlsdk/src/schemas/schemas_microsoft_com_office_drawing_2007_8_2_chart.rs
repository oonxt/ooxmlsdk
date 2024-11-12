/// Defines the PivotOptions Class.
/// When the object is serialized out as xml, it's qualified name is c14:pivotOptions.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c14:pivotOptions")]
pub struct PivotOptions {
    /// _
    #[xml(child = "c14:dropZoneFilter")]
    pub drop_zone_filter: Option<DropZoneFilter>,
    /// _
    #[xml(child = "c14:dropZoneCategories")]
    pub drop_zone_categories: Option<DropZoneCategories>,
    /// _
    #[xml(child = "c14:dropZoneData")]
    pub drop_zone_data: Option<DropZoneData>,
    /// _
    #[xml(child = "c14:dropZoneSeries")]
    pub drop_zone_series: Option<DropZoneSeries>,
    /// _
    #[xml(child = "c14:dropZonesVisible")]
    pub drop_zones_visible: Option<DropZonesVisible>,
}
/// Defines the SketchOptions Class.
/// When the object is serialized out as xml, it's qualified name is c14:sketchOptions.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c14:sketchOptions")]
pub struct SketchOptions {
    /// _
    #[xml(child = "c14:inSketchMode")]
    pub in_sketch_mode: Option<InSketchMode>,
    /// _
    #[xml(child = "c14:showSketchBtn")]
    pub show_sketch_button: Option<ShowSketchButton>,
}
/// Defines the InvertSolidFillFormat Class.
/// When the object is serialized out as xml, it's qualified name is c14:invertSolidFillFmt.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c14:invertSolidFillFmt")]
pub struct InvertSolidFillFormat {
    /// _
    #[xml(child = "c14:spPr")]
    pub shape_properties: ShapeProperties,
}
/// Defines the Style Class.
/// When the object is serialized out as xml, it's qualified name is c14:style.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c14:style")]
pub struct Style {
    /// val
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: u8,
}
/// Defines the ShapeProperties Class.
/// When the object is serialized out as xml, it's qualified name is c14:spPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c14:spPr")]
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
/// Defines the DropZoneFilter Class.
/// When the object is serialized out as xml, it's qualified name is c14:dropZoneFilter.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c14:dropZoneFilter")]
pub struct DropZoneFilter {
    /// val
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: Option<bool>,
}
/// Defines the DropZoneCategories Class.
/// When the object is serialized out as xml, it's qualified name is c14:dropZoneCategories.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c14:dropZoneCategories")]
pub struct DropZoneCategories {
    /// val
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: Option<bool>,
}
/// Defines the DropZoneData Class.
/// When the object is serialized out as xml, it's qualified name is c14:dropZoneData.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c14:dropZoneData")]
pub struct DropZoneData {
    /// val
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: Option<bool>,
}
/// Defines the DropZoneSeries Class.
/// When the object is serialized out as xml, it's qualified name is c14:dropZoneSeries.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c14:dropZoneSeries")]
pub struct DropZoneSeries {
    /// val
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: Option<bool>,
}
/// Defines the DropZonesVisible Class.
/// When the object is serialized out as xml, it's qualified name is c14:dropZonesVisible.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c14:dropZonesVisible")]
pub struct DropZonesVisible {
    /// val
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: Option<bool>,
}
/// Defines the InSketchMode Class.
/// When the object is serialized out as xml, it's qualified name is c14:inSketchMode.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c14:inSketchMode")]
pub struct InSketchMode {
    /// val
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: Option<bool>,
}
/// Defines the BooleanFalseType Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct BooleanFalseType {
    /// val
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: Option<bool>,
}
/// Defines the ShowSketchButton Class.
/// When the object is serialized out as xml, it's qualified name is c14:showSketchBtn.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "c14:showSketchBtn")]
pub struct ShowSketchButton {
    /// val
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: Option<bool>,
}
