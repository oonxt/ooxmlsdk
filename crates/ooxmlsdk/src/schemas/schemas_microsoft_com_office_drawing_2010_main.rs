/// Defines the CameraTool Class.
/// When the object is serialized out as xml, it's qualified name is a14:cameraTool.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a14:cameraTool")]
pub struct CameraTool {
    /// cellRange
    /// Represents the following attribute in the schema: :cellRange
    #[xml(attr = "cellRange")]
    pub cell_range: Option<String>,
    /// spid
    /// Represents the following attribute in the schema: :spid
    #[xml(attr = "spid")]
    pub shape_id: Option<String>,
}
/// Defines the CompatExtension Class.
/// When the object is serialized out as xml, it's qualified name is a14:compatExt.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a14:compatExt")]
pub struct CompatExtension {
    /// spid
    /// Represents the following attribute in the schema: :spid
    #[xml(attr = "spid")]
    pub shape_id: Option<String>,
}
/// Defines the IsCanvas Class.
/// When the object is serialized out as xml, it's qualified name is a14:isCanvas.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a14:isCanvas")]
pub struct IsCanvas {
    /// val
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: bool,
}
/// Defines the GvmlContentPart Class.
/// When the object is serialized out as xml, it's qualified name is a14:contentPart.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a14:contentPart")]
pub struct GvmlContentPart {
    /// bwMode
    /// Represents the following attribute in the schema: :bwMode
    #[xml(attr = "bwMode")]
    pub black_white_mode: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::BlackWhiteModeValues,
    >,
    /// id
    /// Represents the following attribute in the schema: r:id
    #[xml(attr = "r:id")]
    pub relationship_id: String,
    /// _
    #[xml(child = "a14:nvContentPartPr")]
    pub non_visual_content_part_properties: Option<NonVisualContentPartProperties>,
    /// _
    #[xml(child = "a14:xfrm")]
    pub transform2_d: Option<Transform2D>,
    /// _
    #[xml(child = "a14:extLst")]
    pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the ShadowObscured Class.
/// When the object is serialized out as xml, it's qualified name is a14:shadowObscured.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a14:shadowObscured")]
pub struct ShadowObscured {
    /// val
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: Option<bool>,
}
/// Defines the HiddenFillProperties Class.
/// When the object is serialized out as xml, it's qualified name is a14:hiddenFill.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a14:hiddenFill")]
pub struct HiddenFillProperties {
    #[xml(
        child = "a:noFill",
        child = "a:solidFill",
        child = "a:gradFill",
        child = "a:blipFill",
        child = "a:pattFill",
        child = "a:grpFill",
    )]
    pub children: Vec<HiddenFillPropertiesChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum HiddenFillPropertiesChildChoice {
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
}
/// Defines the HiddenLineProperties Class.
/// When the object is serialized out as xml, it's qualified name is a14:hiddenLine.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a14:hiddenLine")]
pub struct HiddenLineProperties {
    /// line width
    /// Represents the following attribute in the schema: :w
    #[xml(attr = "w")]
    pub width: Option<i32>,
    /// line cap
    /// Represents the following attribute in the schema: :cap
    #[xml(attr = "cap")]
    pub cap_type: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::LineCapValues,
    >,
    /// compound line type
    /// Represents the following attribute in the schema: :cmpd
    #[xml(attr = "cmpd")]
    pub compound_line_type: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::CompoundLineValues,
    >,
    /// pen alignment
    /// Represents the following attribute in the schema: :algn
    #[xml(attr = "algn")]
    pub alignment: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::PenAlignmentValues,
    >,
    #[xml(
        child = "a:noFill",
        child = "a:solidFill",
        child = "a:gradFill",
        child = "a:pattFill",
        child = "a:prstDash",
        child = "a:custDash",
        child = "a:round",
        child = "a:bevel",
        child = "a:miter",
        child = "a:headEnd",
        child = "a:tailEnd",
        child = "a:extLst",
    )]
    pub children: Vec<HiddenLinePropertiesChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum HiddenLinePropertiesChildChoice {
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
    #[xml(tag = "a:pattFill")]
    APattFill(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::PatternFill,
    ),
    #[xml(tag = "a:prstDash")]
    APrstDash(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::PresetDash,
    ),
    #[xml(tag = "a:custDash")]
    ACustDash(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::CustomDash,
    ),
    #[xml(tag = "a:round")]
    ARound(crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Round),
    #[xml(tag = "a:bevel")]
    ABevel(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::LineJoinBevel,
    ),
    #[xml(tag = "a:miter")]
    AMiter(crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Miter),
    #[xml(tag = "a:headEnd")]
    AHeadEnd(crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::HeadEnd),
    #[xml(tag = "a:tailEnd")]
    ATailEnd(crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::TailEnd),
    #[xml(tag = "a:extLst")]
    AExtLst(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::LinePropertiesExtensionList,
    ),
}
/// Defines the HiddenEffectsProperties Class.
/// When the object is serialized out as xml, it's qualified name is a14:hiddenEffects.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a14:hiddenEffects")]
pub struct HiddenEffectsProperties {
    #[xml(child = "a:effectLst", child = "a:effectDag")]
    pub children: Vec<HiddenEffectsPropertiesChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum HiddenEffectsPropertiesChildChoice {
    #[xml(tag = "a:effectLst")]
    AEffectLst(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::EffectList,
    ),
    #[xml(tag = "a:effectDag")]
    AEffectDag(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::EffectDag,
    ),
}
/// Defines the HiddenScene3D Class.
/// When the object is serialized out as xml, it's qualified name is a14:hiddenScene3d.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a14:hiddenScene3d")]
pub struct HiddenScene3D {
    ///Camera
    #[xml(child = "a:camera")]
    pub camera: crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Camera,
    ///Light Rig
    #[xml(child = "a:lightRig")]
    pub light_rig: crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::LightRig,
    ///Backdrop Plane
    #[xml(child = "a:backdrop")]
    pub backdrop: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Backdrop,
    >,
    /// _
    #[xml(child = "a:extLst")]
    pub extension_list: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ExtensionList,
    >,
}
/// Defines the HiddenShape3D Class.
/// When the object is serialized out as xml, it's qualified name is a14:hiddenSp3d.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a14:hiddenSp3d")]
pub struct HiddenShape3D {
    /// Shape Depth
    /// Represents the following attribute in the schema: :z
    #[xml(attr = "z")]
    pub z: Option<i32>,
    /// Extrusion Height
    /// Represents the following attribute in the schema: :extrusionH
    #[xml(attr = "extrusionH")]
    pub extrusion_height: Option<i32>,
    /// Contour Width
    /// Represents the following attribute in the schema: :contourW
    #[xml(attr = "contourW")]
    pub contour_width: Option<i32>,
    /// Preset Material Type
    /// Represents the following attribute in the schema: :prstMaterial
    #[xml(attr = "prstMaterial")]
    pub preset_material: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::PresetMaterialTypeValues,
    >,
    ///Top Bevel
    #[xml(child = "a:bevelT")]
    pub bevel_top: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::BevelTop,
    >,
    ///Bottom Bevel
    #[xml(child = "a:bevelB")]
    pub bevel_bottom: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::BevelBottom,
    >,
    ///Extrusion Color
    #[xml(child = "a:extrusionClr")]
    pub extrusion_color: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ExtrusionColor,
    >,
    ///Contour Color
    #[xml(child = "a:contourClr")]
    pub contour_color: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ContourColor,
    >,
    /// _
    #[xml(child = "a:extLst")]
    pub extension_list: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ExtensionList,
    >,
}
/// Defines the ImageProperties Class.
/// When the object is serialized out as xml, it's qualified name is a14:imgProps.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a14:imgProps")]
pub struct ImageProperties {
    /// _
    #[xml(child = "a14:imgLayer")]
    pub image_layer: ImageLayer,
}
/// Defines the UseLocalDpi Class.
/// When the object is serialized out as xml, it's qualified name is a14:useLocalDpi.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a14:useLocalDpi")]
pub struct UseLocalDpi {
    /// val
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: Option<bool>,
}
/// Defines the TextMath Class.
/// When the object is serialized out as xml, it's qualified name is a14:m.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a14:m")]
pub struct TextMath {}
/// Defines the OfficeArtExtensionList Class.
/// When the object is serialized out as xml, it's qualified name is a14:extLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a14:extLst")]
pub struct OfficeArtExtensionList {
    #[xml(child = "a:ext")]
    pub children: Vec<OfficeArtExtensionListChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum OfficeArtExtensionListChildChoice {
    #[xml(tag = "a:ext")]
    AExt(crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Extension),
}
/// Defines the ContentPartLocks Class.
/// When the object is serialized out as xml, it's qualified name is a14:cpLocks.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a14:cpLocks")]
pub struct ContentPartLocks {
    /// Disallow Shape Grouping
    /// Represents the following attribute in the schema: :noGrp
    #[xml(attr = "noGrp")]
    pub no_grouping: Option<bool>,
    /// Disallow Shape Selection
    /// Represents the following attribute in the schema: :noSelect
    #[xml(attr = "noSelect")]
    pub no_selection: Option<bool>,
    /// Disallow Shape Rotation
    /// Represents the following attribute in the schema: :noRot
    #[xml(attr = "noRot")]
    pub no_rotation: Option<bool>,
    /// Disallow Aspect Ratio Change
    /// Represents the following attribute in the schema: :noChangeAspect
    #[xml(attr = "noChangeAspect")]
    pub no_change_aspect: Option<bool>,
    /// Disallow Shape Movement
    /// Represents the following attribute in the schema: :noMove
    #[xml(attr = "noMove")]
    pub no_move: Option<bool>,
    /// Disallow Shape Resize
    /// Represents the following attribute in the schema: :noResize
    #[xml(attr = "noResize")]
    pub no_resize: Option<bool>,
    /// Disallow Shape Point Editing
    /// Represents the following attribute in the schema: :noEditPoints
    #[xml(attr = "noEditPoints")]
    pub no_edit_points: Option<bool>,
    /// Disallow Showing Adjust Handles
    /// Represents the following attribute in the schema: :noAdjustHandles
    #[xml(attr = "noAdjustHandles")]
    pub no_adjust_handles: Option<bool>,
    /// Disallow Arrowhead Changes
    /// Represents the following attribute in the schema: :noChangeArrowheads
    #[xml(attr = "noChangeArrowheads")]
    pub no_change_arrowheads: Option<bool>,
    /// Disallow Shape Type Change
    /// Represents the following attribute in the schema: :noChangeShapeType
    #[xml(attr = "noChangeShapeType")]
    pub no_change_shape_type: Option<bool>,
    /// _
    #[xml(child = "a14:extLst")]
    pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the ForegroundMark Class.
/// When the object is serialized out as xml, it's qualified name is a14:foregroundMark.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a14:foregroundMark")]
pub struct ForegroundMark {
    /// x1
    /// Represents the following attribute in the schema: :x1
    #[xml(attr = "x1")]
    pub first_x_coordinate: i32,
    /// y1
    /// Represents the following attribute in the schema: :y1
    #[xml(attr = "y1")]
    pub first_y_coordinate: i32,
    /// x2
    /// Represents the following attribute in the schema: :x2
    #[xml(attr = "x2")]
    pub second_x_coordinate: i32,
    /// y2
    /// Represents the following attribute in the schema: :y2
    #[xml(attr = "y2")]
    pub second_y_coordinate: i32,
}
/// Defines the BackgroundMark Class.
/// When the object is serialized out as xml, it's qualified name is a14:backgroundMark.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a14:backgroundMark")]
pub struct BackgroundMark {
    /// x1
    /// Represents the following attribute in the schema: :x1
    #[xml(attr = "x1")]
    pub first_x_coordinate: i32,
    /// y1
    /// Represents the following attribute in the schema: :y1
    #[xml(attr = "y1")]
    pub first_y_coordinate: i32,
    /// x2
    /// Represents the following attribute in the schema: :x2
    #[xml(attr = "x2")]
    pub second_x_coordinate: i32,
    /// y2
    /// Represents the following attribute in the schema: :y2
    #[xml(attr = "y2")]
    pub second_y_coordinate: i32,
}
/// Defines the ArtisticBlur Class.
/// When the object is serialized out as xml, it's qualified name is a14:artisticBlur.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a14:artisticBlur")]
pub struct ArtisticBlur {
    /// radius
    /// Represents the following attribute in the schema: :radius
    #[xml(attr = "radius")]
    pub radius: Option<i32>,
}
/// Defines the ArtisticCement Class.
/// When the object is serialized out as xml, it's qualified name is a14:artisticCement.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a14:artisticCement")]
pub struct ArtisticCement {
    /// trans
    /// Represents the following attribute in the schema: :trans
    #[xml(attr = "trans")]
    pub transparancy: Option<i32>,
    /// crackSpacing
    /// Represents the following attribute in the schema: :crackSpacing
    #[xml(attr = "crackSpacing")]
    pub crack_spacing: Option<i32>,
}
/// Defines the ArtisticChalkSketch Class.
/// When the object is serialized out as xml, it's qualified name is a14:artisticChalkSketch.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a14:artisticChalkSketch")]
pub struct ArtisticChalkSketch {
    /// trans
    /// Represents the following attribute in the schema: :trans
    #[xml(attr = "trans")]
    pub transparancy: Option<i32>,
    /// pressure
    /// Represents the following attribute in the schema: :pressure
    #[xml(attr = "pressure")]
    pub pressure: Option<i32>,
}
/// Defines the ArtisticCrisscrossEtching Class.
/// When the object is serialized out as xml, it's qualified name is a14:artisticCrisscrossEtching.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a14:artisticCrisscrossEtching")]
pub struct ArtisticCrisscrossEtching {
    /// trans
    /// Represents the following attribute in the schema: :trans
    #[xml(attr = "trans")]
    pub transparancy: Option<i32>,
    /// pressure
    /// Represents the following attribute in the schema: :pressure
    #[xml(attr = "pressure")]
    pub pressure: Option<i32>,
}
/// Defines the ArtisticCutout Class.
/// When the object is serialized out as xml, it's qualified name is a14:artisticCutout.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a14:artisticCutout")]
pub struct ArtisticCutout {
    /// trans
    /// Represents the following attribute in the schema: :trans
    #[xml(attr = "trans")]
    pub transparancy: Option<i32>,
    /// numberOfShades
    /// Represents the following attribute in the schema: :numberOfShades
    #[xml(attr = "numberOfShades")]
    pub number_of_shades: Option<i32>,
}
/// Defines the ArtisticFilmGrain Class.
/// When the object is serialized out as xml, it's qualified name is a14:artisticFilmGrain.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a14:artisticFilmGrain")]
pub struct ArtisticFilmGrain {
    /// trans
    /// Represents the following attribute in the schema: :trans
    #[xml(attr = "trans")]
    pub transparancy: Option<i32>,
    /// grainSize
    /// Represents the following attribute in the schema: :grainSize
    #[xml(attr = "grainSize")]
    pub grain_size: Option<i32>,
}
/// Defines the ArtisticGlass Class.
/// When the object is serialized out as xml, it's qualified name is a14:artisticGlass.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a14:artisticGlass")]
pub struct ArtisticGlass {
    /// trans
    /// Represents the following attribute in the schema: :trans
    #[xml(attr = "trans")]
    pub transparancy: Option<i32>,
    /// scaling
    /// Represents the following attribute in the schema: :scaling
    #[xml(attr = "scaling")]
    pub scaling: Option<i32>,
}
/// Defines the ArtisticGlowDiffused Class.
/// When the object is serialized out as xml, it's qualified name is a14:artisticGlowDiffused.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a14:artisticGlowDiffused")]
pub struct ArtisticGlowDiffused {
    /// trans
    /// Represents the following attribute in the schema: :trans
    #[xml(attr = "trans")]
    pub transparancy: Option<i32>,
    /// intensity
    /// Represents the following attribute in the schema: :intensity
    #[xml(attr = "intensity")]
    pub intensity: Option<i32>,
}
/// Defines the ArtisticGlowEdges Class.
/// When the object is serialized out as xml, it's qualified name is a14:artisticGlowEdges.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a14:artisticGlowEdges")]
pub struct ArtisticGlowEdges {
    /// trans
    /// Represents the following attribute in the schema: :trans
    #[xml(attr = "trans")]
    pub transparancy: Option<i32>,
    /// smoothness
    /// Represents the following attribute in the schema: :smoothness
    #[xml(attr = "smoothness")]
    pub smoothness: Option<i32>,
}
/// Defines the ArtisticLightScreen Class.
/// When the object is serialized out as xml, it's qualified name is a14:artisticLightScreen.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a14:artisticLightScreen")]
pub struct ArtisticLightScreen {
    /// trans
    /// Represents the following attribute in the schema: :trans
    #[xml(attr = "trans")]
    pub transparancy: Option<i32>,
    /// gridSize
    /// Represents the following attribute in the schema: :gridSize
    #[xml(attr = "gridSize")]
    pub grid_size: Option<i32>,
}
/// Defines the ArtisticLineDrawing Class.
/// When the object is serialized out as xml, it's qualified name is a14:artisticLineDrawing.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a14:artisticLineDrawing")]
pub struct ArtisticLineDrawing {
    /// trans
    /// Represents the following attribute in the schema: :trans
    #[xml(attr = "trans")]
    pub transparancy: Option<i32>,
    /// pencilSize
    /// Represents the following attribute in the schema: :pencilSize
    #[xml(attr = "pencilSize")]
    pub pencil_size: Option<i32>,
}
/// Defines the ArtisticMarker Class.
/// When the object is serialized out as xml, it's qualified name is a14:artisticMarker.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a14:artisticMarker")]
pub struct ArtisticMarker {
    /// trans
    /// Represents the following attribute in the schema: :trans
    #[xml(attr = "trans")]
    pub transparancy: Option<i32>,
    /// size
    /// Represents the following attribute in the schema: :size
    #[xml(attr = "size")]
    pub size: Option<i32>,
}
/// Defines the ArtisticMosaicBubbles Class.
/// When the object is serialized out as xml, it's qualified name is a14:artisticMosiaicBubbles.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a14:artisticMosiaicBubbles")]
pub struct ArtisticMosaicBubbles {
    /// trans
    /// Represents the following attribute in the schema: :trans
    #[xml(attr = "trans")]
    pub transparancy: Option<i32>,
    /// pressure
    /// Represents the following attribute in the schema: :pressure
    #[xml(attr = "pressure")]
    pub pressure: Option<i32>,
}
/// Defines the ArtisticPaintStrokes Class.
/// When the object is serialized out as xml, it's qualified name is a14:artisticPaintStrokes.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a14:artisticPaintStrokes")]
pub struct ArtisticPaintStrokes {
    /// trans
    /// Represents the following attribute in the schema: :trans
    #[xml(attr = "trans")]
    pub transparancy: Option<i32>,
    /// intensity
    /// Represents the following attribute in the schema: :intensity
    #[xml(attr = "intensity")]
    pub intensity: Option<i32>,
}
/// Defines the ArtisticPaintBrush Class.
/// When the object is serialized out as xml, it's qualified name is a14:artisticPaintBrush.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a14:artisticPaintBrush")]
pub struct ArtisticPaintBrush {
    /// trans
    /// Represents the following attribute in the schema: :trans
    #[xml(attr = "trans")]
    pub transparancy: Option<i32>,
    /// brushSize
    /// Represents the following attribute in the schema: :brushSize
    #[xml(attr = "brushSize")]
    pub brush_size: Option<i32>,
}
/// Defines the ArtisticPastelsSmooth Class.
/// When the object is serialized out as xml, it's qualified name is a14:artisticPastelsSmooth.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a14:artisticPastelsSmooth")]
pub struct ArtisticPastelsSmooth {
    /// trans
    /// Represents the following attribute in the schema: :trans
    #[xml(attr = "trans")]
    pub transparancy: Option<i32>,
    /// scaling
    /// Represents the following attribute in the schema: :scaling
    #[xml(attr = "scaling")]
    pub brush_size: Option<i32>,
}
/// Defines the ArtisticPencilGrayscale Class.
/// When the object is serialized out as xml, it's qualified name is a14:artisticPencilGrayscale.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a14:artisticPencilGrayscale")]
pub struct ArtisticPencilGrayscale {
    /// trans
    /// Represents the following attribute in the schema: :trans
    #[xml(attr = "trans")]
    pub transparancy: Option<i32>,
    /// pencilSize
    /// Represents the following attribute in the schema: :pencilSize
    #[xml(attr = "pencilSize")]
    pub brush_size: Option<i32>,
}
/// Defines the ArtisticPencilSketch Class.
/// When the object is serialized out as xml, it's qualified name is a14:artisticPencilSketch.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a14:artisticPencilSketch")]
pub struct ArtisticPencilSketch {
    /// trans
    /// Represents the following attribute in the schema: :trans
    #[xml(attr = "trans")]
    pub transparancy: Option<i32>,
    /// pressure
    /// Represents the following attribute in the schema: :pressure
    #[xml(attr = "pressure")]
    pub pressure: Option<i32>,
}
/// Defines the ArtisticPhotocopy Class.
/// When the object is serialized out as xml, it's qualified name is a14:artisticPhotocopy.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a14:artisticPhotocopy")]
pub struct ArtisticPhotocopy {
    /// trans
    /// Represents the following attribute in the schema: :trans
    #[xml(attr = "trans")]
    pub transparancy: Option<i32>,
    /// detail
    /// Represents the following attribute in the schema: :detail
    #[xml(attr = "detail")]
    pub detail: Option<i32>,
}
/// Defines the ArtisticPlasticWrap Class.
/// When the object is serialized out as xml, it's qualified name is a14:artisticPlasticWrap.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a14:artisticPlasticWrap")]
pub struct ArtisticPlasticWrap {
    /// trans
    /// Represents the following attribute in the schema: :trans
    #[xml(attr = "trans")]
    pub transparancy: Option<i32>,
    /// smoothness
    /// Represents the following attribute in the schema: :smoothness
    #[xml(attr = "smoothness")]
    pub smoothness: Option<i32>,
}
/// Defines the ArtisticTexturizer Class.
/// When the object is serialized out as xml, it's qualified name is a14:artisticTexturizer.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a14:artisticTexturizer")]
pub struct ArtisticTexturizer {
    /// trans
    /// Represents the following attribute in the schema: :trans
    #[xml(attr = "trans")]
    pub transparancy: Option<i32>,
    /// scaling
    /// Represents the following attribute in the schema: :scaling
    #[xml(attr = "scaling")]
    pub scaling: Option<i32>,
}
/// Defines the ArtisticWatercolorSponge Class.
/// When the object is serialized out as xml, it's qualified name is a14:artisticWatercolorSponge.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a14:artisticWatercolorSponge")]
pub struct ArtisticWatercolorSponge {
    /// trans
    /// Represents the following attribute in the schema: :trans
    #[xml(attr = "trans")]
    pub transparancy: Option<i32>,
    /// brushSize
    /// Represents the following attribute in the schema: :brushSize
    #[xml(attr = "brushSize")]
    pub brush_size: Option<i32>,
}
/// Defines the BackgroundRemoval Class.
/// When the object is serialized out as xml, it's qualified name is a14:backgroundRemoval.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a14:backgroundRemoval")]
pub struct BackgroundRemoval {
    /// t
    /// Represents the following attribute in the schema: :t
    #[xml(attr = "t")]
    pub marquee_top: i32,
    /// b
    /// Represents the following attribute in the schema: :b
    #[xml(attr = "b")]
    pub marquee_bottom: i32,
    /// l
    /// Represents the following attribute in the schema: :l
    #[xml(attr = "l")]
    pub marquee_left: i32,
    /// r
    /// Represents the following attribute in the schema: :r
    #[xml(attr = "r")]
    pub marquee_right: i32,
    /// _
    #[xml(child = "a14:foregroundMark")]
    pub a14_foreground_mark: Vec<ForegroundMark>,
    /// _
    #[xml(child = "a14:backgroundMark")]
    pub a14_background_mark: Vec<BackgroundMark>,
}
/// Defines the BrightnessContrast Class.
/// When the object is serialized out as xml, it's qualified name is a14:brightnessContrast.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a14:brightnessContrast")]
pub struct BrightnessContrast {
    /// bright
    /// Represents the following attribute in the schema: :bright
    #[xml(attr = "bright")]
    pub bright: Option<i32>,
    /// contrast
    /// Represents the following attribute in the schema: :contrast
    #[xml(attr = "contrast")]
    pub contrast: Option<i32>,
}
/// Defines the ColorTemperature Class.
/// When the object is serialized out as xml, it's qualified name is a14:colorTemperature.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a14:colorTemperature")]
pub struct ColorTemperature {
    /// colorTemp
    /// Represents the following attribute in the schema: :colorTemp
    #[xml(attr = "colorTemp")]
    pub color_temperature_value: Option<i32>,
}
/// Defines the Saturation Class.
/// When the object is serialized out as xml, it's qualified name is a14:saturation.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a14:saturation")]
pub struct Saturation {
    /// sat
    /// Represents the following attribute in the schema: :sat
    #[xml(attr = "sat")]
    pub saturation_amount: Option<i32>,
}
/// Defines the SharpenSoften Class.
/// When the object is serialized out as xml, it's qualified name is a14:sharpenSoften.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a14:sharpenSoften")]
pub struct SharpenSoften {
    /// amount
    /// Represents the following attribute in the schema: :amount
    #[xml(attr = "amount")]
    pub amount: Option<i32>,
}
/// Defines the ImageEffect Class.
/// When the object is serialized out as xml, it's qualified name is a14:imgEffect.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a14:imgEffect")]
pub struct ImageEffect {
    /// visible
    /// Represents the following attribute in the schema: :visible
    #[xml(attr = "visible")]
    pub visible: Option<bool>,
    #[xml(
        child = "a14:artisticBlur",
        child = "a14:artisticCement",
        child = "a14:artisticChalkSketch",
        child = "a14:artisticCrisscrossEtching",
        child = "a14:artisticCutout",
        child = "a14:artisticFilmGrain",
        child = "a14:artisticGlass",
        child = "a14:artisticGlowDiffused",
        child = "a14:artisticGlowEdges",
        child = "a14:artisticLightScreen",
        child = "a14:artisticLineDrawing",
        child = "a14:artisticMarker",
        child = "a14:artisticMosiaicBubbles",
        child = "a14:artisticPaintStrokes",
        child = "a14:artisticPaintBrush",
        child = "a14:artisticPastelsSmooth",
        child = "a14:artisticPencilGrayscale",
        child = "a14:artisticPencilSketch",
        child = "a14:artisticPhotocopy",
        child = "a14:artisticPlasticWrap",
        child = "a14:artisticTexturizer",
        child = "a14:artisticWatercolorSponge",
        child = "a14:backgroundRemoval",
        child = "a14:brightnessContrast",
        child = "a14:colorTemperature",
        child = "a14:saturation",
        child = "a14:sharpenSoften",
    )]
    pub children: Vec<ImageEffectChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum ImageEffectChildChoice {
    #[xml(tag = "a14:artisticBlur")]
    A14ArtisticBlur(ArtisticBlur),
    #[xml(tag = "a14:artisticCement")]
    A14ArtisticCement(ArtisticCement),
    #[xml(tag = "a14:artisticChalkSketch")]
    A14ArtisticChalkSketch(ArtisticChalkSketch),
    #[xml(tag = "a14:artisticCrisscrossEtching")]
    A14ArtisticCrisscrossEtching(ArtisticCrisscrossEtching),
    #[xml(tag = "a14:artisticCutout")]
    A14ArtisticCutout(ArtisticCutout),
    #[xml(tag = "a14:artisticFilmGrain")]
    A14ArtisticFilmGrain(ArtisticFilmGrain),
    #[xml(tag = "a14:artisticGlass")]
    A14ArtisticGlass(ArtisticGlass),
    #[xml(tag = "a14:artisticGlowDiffused")]
    A14ArtisticGlowDiffused(ArtisticGlowDiffused),
    #[xml(tag = "a14:artisticGlowEdges")]
    A14ArtisticGlowEdges(ArtisticGlowEdges),
    #[xml(tag = "a14:artisticLightScreen")]
    A14ArtisticLightScreen(ArtisticLightScreen),
    #[xml(tag = "a14:artisticLineDrawing")]
    A14ArtisticLineDrawing(ArtisticLineDrawing),
    #[xml(tag = "a14:artisticMarker")]
    A14ArtisticMarker(ArtisticMarker),
    #[xml(tag = "a14:artisticMosiaicBubbles")]
    A14ArtisticMosiaicBubbles(ArtisticMosaicBubbles),
    #[xml(tag = "a14:artisticPaintStrokes")]
    A14ArtisticPaintStrokes(ArtisticPaintStrokes),
    #[xml(tag = "a14:artisticPaintBrush")]
    A14ArtisticPaintBrush(ArtisticPaintBrush),
    #[xml(tag = "a14:artisticPastelsSmooth")]
    A14ArtisticPastelsSmooth(ArtisticPastelsSmooth),
    #[xml(tag = "a14:artisticPencilGrayscale")]
    A14ArtisticPencilGrayscale(ArtisticPencilGrayscale),
    #[xml(tag = "a14:artisticPencilSketch")]
    A14ArtisticPencilSketch(ArtisticPencilSketch),
    #[xml(tag = "a14:artisticPhotocopy")]
    A14ArtisticPhotocopy(ArtisticPhotocopy),
    #[xml(tag = "a14:artisticPlasticWrap")]
    A14ArtisticPlasticWrap(ArtisticPlasticWrap),
    #[xml(tag = "a14:artisticTexturizer")]
    A14ArtisticTexturizer(ArtisticTexturizer),
    #[xml(tag = "a14:artisticWatercolorSponge")]
    A14ArtisticWatercolorSponge(ArtisticWatercolorSponge),
    #[xml(tag = "a14:backgroundRemoval")]
    A14BackgroundRemoval(BackgroundRemoval),
    #[xml(tag = "a14:brightnessContrast")]
    A14BrightnessContrast(BrightnessContrast),
    #[xml(tag = "a14:colorTemperature")]
    A14ColorTemperature(ColorTemperature),
    #[xml(tag = "a14:saturation")]
    A14Saturation(Saturation),
    #[xml(tag = "a14:sharpenSoften")]
    A14SharpenSoften(SharpenSoften),
}
/// Defines the ImageLayer Class.
/// When the object is serialized out as xml, it's qualified name is a14:imgLayer.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a14:imgLayer")]
pub struct ImageLayer {
    /// embed
    /// Represents the following attribute in the schema: r:embed
    #[xml(attr = "r:embed")]
    pub embed: Option<String>,
    /// _
    #[xml(child = "a14:imgEffect")]
    pub a14_img_effect: Vec<ImageEffect>,
}
/// Defines the NonVisualDrawingProperties Class.
/// When the object is serialized out as xml, it's qualified name is a14:cNvPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a14:cNvPr")]
pub struct NonVisualDrawingProperties {
    /// Application defined unique identifier.
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: u32,
    /// Name compatible with Object Model (non-unique).
    /// Represents the following attribute in the schema: :name
    #[xml(attr = "name")]
    pub name: String,
    /// Description of the drawing element.
    /// Represents the following attribute in the schema: :descr
    #[xml(attr = "descr")]
    pub description: Option<String>,
    /// Flag determining to show or hide this element.
    /// Represents the following attribute in the schema: :hidden
    #[xml(attr = "hidden")]
    pub hidden: Option<bool>,
    /// Title
    /// Represents the following attribute in the schema: :title
    #[xml(attr = "title")]
    pub title: Option<String>,
    ///Hyperlink associated with clicking or selecting the element.
    #[xml(child = "a:hlinkClick")]
    pub hyperlink_on_click: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::HyperlinkOnClick,
    >,
    ///Hyperlink associated with hovering over the element.
    #[xml(child = "a:hlinkHover")]
    pub hyperlink_on_hover: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::HyperlinkOnHover,
    >,
    ///Future extension
    #[xml(child = "a:extLst")]
    pub non_visual_drawing_properties_extension_list: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::NonVisualDrawingPropertiesExtensionList,
    >,
}
/// Defines the NonVisualInkContentPartProperties Class.
/// When the object is serialized out as xml, it's qualified name is a14:cNvContentPartPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a14:cNvContentPartPr")]
pub struct NonVisualInkContentPartProperties {
    /// isComment
    /// Represents the following attribute in the schema: :isComment
    #[xml(attr = "isComment")]
    pub is_comment: Option<bool>,
    /// _
    #[xml(child = "a14:cpLocks")]
    pub content_part_locks: Option<ContentPartLocks>,
    /// _
    #[xml(child = "a14:extLst")]
    pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the NonVisualContentPartProperties Class.
/// When the object is serialized out as xml, it's qualified name is a14:nvContentPartPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a14:nvContentPartPr")]
pub struct NonVisualContentPartProperties {
    /// _
    #[xml(child = "a14:cNvPr")]
    pub non_visual_drawing_properties: NonVisualDrawingProperties,
    /// _
    #[xml(child = "a14:cNvContentPartPr")]
    pub non_visual_ink_content_part_properties: Option<
        NonVisualInkContentPartProperties,
    >,
}
/// Defines the Transform2D Class.
/// When the object is serialized out as xml, it's qualified name is a14:xfrm.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a14:xfrm")]
pub struct Transform2D {
    /// Rotation
    /// Represents the following attribute in the schema: :rot
    #[xml(attr = "rot")]
    pub rotation: Option<i32>,
    /// Horizontal Flip
    /// Represents the following attribute in the schema: :flipH
    #[xml(attr = "flipH")]
    pub horizontal_flip: Option<bool>,
    /// Vertical Flip
    /// Represents the following attribute in the schema: :flipV
    #[xml(attr = "flipV")]
    pub vertical_flip: Option<bool>,
    ///Offset
    #[xml(child = "a:off")]
    pub offset: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Offset,
    >,
    ///Extents
    #[xml(child = "a:ext")]
    pub extents: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Extents,
    >,
}
