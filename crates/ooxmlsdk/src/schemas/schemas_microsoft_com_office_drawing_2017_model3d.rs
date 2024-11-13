/// Defines the Model3D Class.
/// When the object is serialized out as xml, it's qualified name is am3d:model3d.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "am3d:model3d")]
pub struct Model3D {
    /// Embedded Picture Reference
    /// Represents the following attribute in the schema: r:embed
    #[xml(attr = "r:embed")]
    pub embed: Option<String>,
    /// Linked Picture Reference
    /// Represents the following attribute in the schema: r:link
    #[xml(attr = "r:link")]
    pub link: Option<String>,
    #[xml(
        child = "am3d:spPr",
        child = "am3d:camera",
        child = "am3d:trans",
        child = "am3d:attrSrcUrl",
        child = "am3d:raster",
        child = "am3d:extLst",
        child = "am3d:objViewport",
        child = "am3d:winViewport",
        child = "am3d:ambientLight",
        child = "am3d:ptLight",
        child = "am3d:spotLight",
        child = "am3d:dirLight",
        child = "am3d:unkLight",
    )]
    pub children: Vec<Model3DChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum Model3DChildChoice {
    #[xml(tag = "am3d:spPr")]
    Am3dSpPr(ShapeProperties),
    #[xml(tag = "am3d:camera")]
    Am3dCamera(Model3DCamera),
    #[xml(tag = "am3d:trans")]
    Am3dTrans(Model3DTransform),
    #[xml(tag = "am3d:attrSrcUrl")]
    Am3dAttrSrcUrl(PictureAttributionSourceUrl),
    #[xml(tag = "am3d:raster")]
    Am3dRaster(Model3DRaster),
    #[xml(tag = "am3d:extLst")]
    Am3dExtLst(Model3DExtensionList),
    #[xml(tag = "am3d:objViewport")]
    Am3dObjViewport(ObjectViewport),
    #[xml(tag = "am3d:winViewport")]
    Am3dWinViewport(WindowViewport),
    #[xml(tag = "am3d:ambientLight")]
    Am3dAmbientLight(AmbientLight),
    #[xml(tag = "am3d:ptLight")]
    Am3dPtLight(PointLight),
    #[xml(tag = "am3d:spotLight")]
    Am3dSpotLight(SpotLight),
    #[xml(tag = "am3d:dirLight")]
    Am3dDirLight(DirectionalLight),
    #[xml(tag = "am3d:unkLight")]
    Am3dUnkLight(UnknownLight),
}
/// Defines the SxRatio Class.
/// When the object is serialized out as xml, it's qualified name is am3d:sx.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "am3d:sx")]
pub struct SxRatio {
    /// Numerator
    /// Represents the following attribute in the schema: :n
    #[xml(attr = "n")]
    pub numerator: i32,
    /// Denominator
    /// Represents the following attribute in the schema: :d
    #[xml(attr = "d")]
    pub denominator: i32,
}
/// Defines the SyRatio Class.
/// When the object is serialized out as xml, it's qualified name is am3d:sy.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "am3d:sy")]
pub struct SyRatio {
    /// Numerator
    /// Represents the following attribute in the schema: :n
    #[xml(attr = "n")]
    pub numerator: i32,
    /// Denominator
    /// Represents the following attribute in the schema: :d
    #[xml(attr = "d")]
    pub denominator: i32,
}
/// Defines the SzRatio Class.
/// When the object is serialized out as xml, it's qualified name is am3d:sz.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "am3d:sz")]
pub struct SzRatio {
    /// Numerator
    /// Represents the following attribute in the schema: :n
    #[xml(attr = "n")]
    pub numerator: i32,
    /// Denominator
    /// Represents the following attribute in the schema: :d
    #[xml(attr = "d")]
    pub denominator: i32,
}
/// Defines the RatioType Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct RatioType {
    /// Numerator
    /// Represents the following attribute in the schema: :n
    #[xml(attr = "n")]
    pub numerator: i32,
    /// Denominator
    /// Represents the following attribute in the schema: :d
    #[xml(attr = "d")]
    pub denominator: i32,
}
/// Defines the MeterPerModelUnitPositiveRatio Class.
/// When the object is serialized out as xml, it's qualified name is am3d:meterPerModelUnit.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "am3d:meterPerModelUnit")]
pub struct MeterPerModelUnitPositiveRatio {
    /// n
    /// Represents the following attribute in the schema: :n
    #[xml(attr = "n")]
    pub n: u64,
    /// d
    /// Represents the following attribute in the schema: :d
    #[xml(attr = "d")]
    pub d: u64,
}
/// Defines the SzPositiveRatio Class.
/// When the object is serialized out as xml, it's qualified name is am3d:sz.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "am3d:sz")]
pub struct SzPositiveRatio {
    /// n
    /// Represents the following attribute in the schema: :n
    #[xml(attr = "n")]
    pub n: u64,
    /// d
    /// Represents the following attribute in the schema: :d
    #[xml(attr = "d")]
    pub d: u64,
}
/// Defines the IlluminancePositiveRatio Class.
/// When the object is serialized out as xml, it's qualified name is am3d:illuminance.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "am3d:illuminance")]
pub struct IlluminancePositiveRatio {
    /// n
    /// Represents the following attribute in the schema: :n
    #[xml(attr = "n")]
    pub n: u64,
    /// d
    /// Represents the following attribute in the schema: :d
    #[xml(attr = "d")]
    pub d: u64,
}
/// Defines the IntensityPositiveRatio Class.
/// When the object is serialized out as xml, it's qualified name is am3d:intensity.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "am3d:intensity")]
pub struct IntensityPositiveRatio {
    /// n
    /// Represents the following attribute in the schema: :n
    #[xml(attr = "n")]
    pub n: u64,
    /// d
    /// Represents the following attribute in the schema: :d
    #[xml(attr = "d")]
    pub d: u64,
}
/// Defines the OpenXmlPositiveRatioElement Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct OpenXmlPositiveRatioElement {
    /// n
    /// Represents the following attribute in the schema: :n
    #[xml(attr = "n")]
    pub n: u64,
    /// d
    /// Represents the following attribute in the schema: :d
    #[xml(attr = "d")]
    pub d: u64,
}
/// Defines the PreTransVector3D Class.
/// When the object is serialized out as xml, it's qualified name is am3d:preTrans.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "am3d:preTrans")]
pub struct PreTransVector3D {
    /// Distance along X-axis in 3D
    /// Represents the following attribute in the schema: :dx
    #[xml(attr = "dx")]
    pub dx: i32,
    /// Distance along Y-axis in 3D
    /// Represents the following attribute in the schema: :dy
    #[xml(attr = "dy")]
    pub dy: i32,
    /// Distance along Z-axis in 3D
    /// Represents the following attribute in the schema: :dz
    #[xml(attr = "dz")]
    pub dz: i32,
}
/// Defines the PostTransVector3D Class.
/// When the object is serialized out as xml, it's qualified name is am3d:postTrans.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "am3d:postTrans")]
pub struct PostTransVector3D {
    /// Distance along X-axis in 3D
    /// Represents the following attribute in the schema: :dx
    #[xml(attr = "dx")]
    pub dx: i32,
    /// Distance along Y-axis in 3D
    /// Represents the following attribute in the schema: :dy
    #[xml(attr = "dy")]
    pub dy: i32,
    /// Distance along Z-axis in 3D
    /// Represents the following attribute in the schema: :dz
    #[xml(attr = "dz")]
    pub dz: i32,
}
/// Defines the UpVector3D Class.
/// When the object is serialized out as xml, it's qualified name is am3d:up.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "am3d:up")]
pub struct UpVector3D {
    /// Distance along X-axis in 3D
    /// Represents the following attribute in the schema: :dx
    #[xml(attr = "dx")]
    pub dx: i32,
    /// Distance along Y-axis in 3D
    /// Represents the following attribute in the schema: :dy
    #[xml(attr = "dy")]
    pub dy: i32,
    /// Distance along Z-axis in 3D
    /// Represents the following attribute in the schema: :dz
    #[xml(attr = "dz")]
    pub dz: i32,
}
/// Defines the Vector3DType Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct Vector3DType {
    /// Distance along X-axis in 3D
    /// Represents the following attribute in the schema: :dx
    #[xml(attr = "dx")]
    pub dx: i32,
    /// Distance along Y-axis in 3D
    /// Represents the following attribute in the schema: :dy
    #[xml(attr = "dy")]
    pub dy: i32,
    /// Distance along Z-axis in 3D
    /// Represents the following attribute in the schema: :dz
    #[xml(attr = "dz")]
    pub dz: i32,
}
/// Defines the Scale3D Class.
/// When the object is serialized out as xml, it's qualified name is am3d:scale.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "am3d:scale")]
pub struct Scale3D {
    /// _
    #[xml(child = "am3d:sx")]
    pub sx_ratio: SxRatio,
    /// _
    #[xml(child = "am3d:sy")]
    pub sy_ratio: SyRatio,
    /// _
    #[xml(child = "am3d:sz")]
    pub sz_ratio: SzRatio,
}
/// Defines the Rotate3D Class.
/// When the object is serialized out as xml, it's qualified name is am3d:rot.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "am3d:rot")]
pub struct Rotate3D {
    /// ax
    /// Represents the following attribute in the schema: :ax
    #[xml(attr = "ax")]
    pub ax: Option<i32>,
    /// ay
    /// Represents the following attribute in the schema: :ay
    #[xml(attr = "ay")]
    pub ay: Option<i32>,
    /// az
    /// Represents the following attribute in the schema: :az
    #[xml(attr = "az")]
    pub az: Option<i32>,
}
/// Defines the OfficeArtExtensionList Class.
/// When the object is serialized out as xml, it's qualified name is am3d:extLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "am3d:extLst")]
pub struct OfficeArtExtensionList {
    #[xml(child = "a:ext")]
    pub children: Vec<OfficeArtExtensionListChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum OfficeArtExtensionListChildChoice {
    #[xml(tag = "a:ext")]
    AExt(crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Extension),
}
/// Defines the PosPoint3D Class.
/// When the object is serialized out as xml, it's qualified name is am3d:pos.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "am3d:pos")]
pub struct PosPoint3D {
    /// X-Coordinate in 3D
    /// Represents the following attribute in the schema: :x
    #[xml(attr = "x")]
    pub x: i32,
    /// Y-Coordinate in 3D
    /// Represents the following attribute in the schema: :y
    #[xml(attr = "y")]
    pub y: i32,
    /// Z-Coordinate in 3D
    /// Represents the following attribute in the schema: :z
    #[xml(attr = "z")]
    pub z: i32,
}
/// Defines the LookAtPoint3D Class.
/// When the object is serialized out as xml, it's qualified name is am3d:lookAt.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "am3d:lookAt")]
pub struct LookAtPoint3D {
    /// X-Coordinate in 3D
    /// Represents the following attribute in the schema: :x
    #[xml(attr = "x")]
    pub x: i32,
    /// Y-Coordinate in 3D
    /// Represents the following attribute in the schema: :y
    #[xml(attr = "y")]
    pub y: i32,
    /// Z-Coordinate in 3D
    /// Represents the following attribute in the schema: :z
    #[xml(attr = "z")]
    pub z: i32,
}
/// Defines the OpenXmlPoint3DElement Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct OpenXmlPoint3DElement {
    /// X-Coordinate in 3D
    /// Represents the following attribute in the schema: :x
    #[xml(attr = "x")]
    pub x: i32,
    /// Y-Coordinate in 3D
    /// Represents the following attribute in the schema: :y
    #[xml(attr = "y")]
    pub y: i32,
    /// Z-Coordinate in 3D
    /// Represents the following attribute in the schema: :z
    #[xml(attr = "z")]
    pub z: i32,
}
/// Defines the OrthographicProjection Class.
/// When the object is serialized out as xml, it's qualified name is am3d:orthographic.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "am3d:orthographic")]
pub struct OrthographicProjection {
    /// _
    #[xml(child = "am3d:sz")]
    pub sz_positive_ratio: SzPositiveRatio,
    /// _
    #[xml(child = "am3d:extLst")]
    pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the PerspectiveProjection Class.
/// When the object is serialized out as xml, it's qualified name is am3d:perspective.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "am3d:perspective")]
pub struct PerspectiveProjection {
    /// fov
    /// Represents the following attribute in the schema: :fov
    #[xml(attr = "fov")]
    pub fov: i32,
    /// _
    #[xml(child = "am3d:extLst")]
    pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the Blip Class.
/// When the object is serialized out as xml, it's qualified name is am3d:blip.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "am3d:blip")]
pub struct Blip {
    /// Embedded Picture Reference
    /// Represents the following attribute in the schema: r:embed
    #[xml(attr = "r:embed")]
    pub embed: Option<String>,
    /// Linked Picture Reference
    /// Represents the following attribute in the schema: r:link
    #[xml(attr = "r:link")]
    pub link: Option<String>,
    /// Compression state for blips.
    /// Represents the following attribute in the schema: :cstate
    #[xml(attr = "cstate")]
    pub compression_state: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::BlipCompressionValues,
    >,
    #[xml(
        child = "a:alphaBiLevel",
        child = "a:alphaCeiling",
        child = "a:alphaFloor",
        child = "a:alphaInv",
        child = "a:alphaMod",
        child = "a:alphaModFix",
        child = "a:alphaRepl",
        child = "a:biLevel",
        child = "a:blur",
        child = "a:clrChange",
        child = "a:clrRepl",
        child = "a:duotone",
        child = "a:fillOverlay",
        child = "a:grayscl",
        child = "a:hsl",
        child = "a:lum",
        child = "a:tint",
        child = "a:extLst",
    )]
    pub children: Vec<BlipChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum BlipChildChoice {
    #[xml(tag = "a:alphaBiLevel")]
    AAlphaBiLevel(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::AlphaBiLevel,
    ),
    #[xml(tag = "a:alphaCeiling")]
    AAlphaCeiling(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::AlphaCeiling,
    ),
    #[xml(tag = "a:alphaFloor")]
    AAlphaFloor(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::AlphaFloor,
    ),
    #[xml(tag = "a:alphaInv")]
    AAlphaInv(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::AlphaInverse,
    ),
    #[xml(tag = "a:alphaMod")]
    AAlphaMod(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::AlphaModulationEffect,
    ),
    #[xml(tag = "a:alphaModFix")]
    AAlphaModFix(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::AlphaModulationFixed,
    ),
    #[xml(tag = "a:alphaRepl")]
    AAlphaRepl(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::AlphaReplace,
    ),
    #[xml(tag = "a:biLevel")]
    ABiLevel(crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::BiLevel),
    #[xml(tag = "a:blur")]
    ABlur(crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Blur),
    #[xml(tag = "a:clrChange")]
    AClrChange(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ColorChange,
    ),
    #[xml(tag = "a:clrRepl")]
    AClrRepl(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ColorReplacement,
    ),
    #[xml(tag = "a:duotone")]
    ADuotone(crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Duotone),
    #[xml(tag = "a:fillOverlay")]
    AFillOverlay(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::FillOverlay,
    ),
    #[xml(tag = "a:grayscl")]
    AGrayscl(crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Grayscale),
    #[xml(tag = "a:hsl")]
    AHsl(crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Hsl),
    #[xml(tag = "a:lum")]
    ALum(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::LuminanceEffect,
    ),
    #[xml(tag = "a:tint")]
    ATint(crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::TintEffect),
    #[xml(tag = "a:extLst")]
    AExtLst(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::BlipExtensionList,
    ),
}
/// Defines the ColorType Class.
/// When the object is serialized out as xml, it's qualified name is am3d:clr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "am3d:clr")]
pub struct ColorType {
    #[xml(
        child = "a:scrgbClr",
        child = "a:srgbClr",
        child = "a:hslClr",
        child = "a:sysClr",
        child = "a:schemeClr",
        child = "a:prstClr",
    )]
    pub children: Vec<ColorTypeChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum ColorTypeChildChoice {
    #[xml(tag = "a:scrgbClr")]
    AScrgbClr(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::RgbColorModelPercentage,
    ),
    #[xml(tag = "a:srgbClr")]
    ASrgbClr(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::RgbColorModelHex,
    ),
    #[xml(tag = "a:hslClr")]
    AHslClr(crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::HslColor),
    #[xml(tag = "a:sysClr")]
    ASysClr(crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::SystemColor),
    #[xml(tag = "a:schemeClr")]
    ASchemeClr(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::SchemeColor,
    ),
    #[xml(tag = "a:prstClr")]
    APrstClr(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::PresetColor,
    ),
}
/// Defines the Model3DExtension Class.
/// When the object is serialized out as xml, it's qualified name is am3d:ext.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "am3d:ext")]
pub struct Model3DExtension {
    /// URI
    /// Represents the following attribute in the schema: :uri
    #[xml(attr = "uri")]
    pub uri: String,
    #[xml(child = "a3danim:embedAnim", child = "a3danim:posterFrame")]
    pub children: Vec<Model3DExtensionChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum Model3DExtensionChildChoice {
    #[xml(tag = "a3danim:embedAnim")]
    A3danimEmbedAnim(
        crate::schemas::schemas_microsoft_com_office_drawing_2018_animation_model3d::EmbeddedAnimation,
    ),
    #[xml(tag = "a3danim:posterFrame")]
    A3danimPosterFrame(
        crate::schemas::schemas_microsoft_com_office_drawing_2018_animation_model3d::PosterFrame,
    ),
}
/// Defines the ShapeProperties Class.
/// When the object is serialized out as xml, it's qualified name is am3d:spPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "am3d:spPr")]
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
/// Defines the Model3DCamera Class.
/// When the object is serialized out as xml, it's qualified name is am3d:camera.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "am3d:camera")]
pub struct Model3DCamera {
    #[xml(
        child = "am3d:pos",
        child = "am3d:up",
        child = "am3d:lookAt",
        child = "am3d:orthographic",
        child = "am3d:perspective",
        child = "am3d:extLst",
    )]
    pub children: Vec<Model3DCameraChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum Model3DCameraChildChoice {
    #[xml(tag = "am3d:pos")]
    Am3dPos(PosPoint3D),
    #[xml(tag = "am3d:up")]
    Am3dUp(UpVector3D),
    #[xml(tag = "am3d:lookAt")]
    Am3dLookAt(LookAtPoint3D),
    #[xml(tag = "am3d:orthographic")]
    Am3dOrthographic(OrthographicProjection),
    #[xml(tag = "am3d:perspective")]
    Am3dPerspective(PerspectiveProjection),
    #[xml(tag = "am3d:extLst")]
    Am3dExtLst(OfficeArtExtensionList),
}
/// Defines the Model3DTransform Class.
/// When the object is serialized out as xml, it's qualified name is am3d:trans.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "am3d:trans")]
pub struct Model3DTransform {
    /// _
    #[xml(child = "am3d:meterPerModelUnit")]
    pub meter_per_model_unit_positive_ratio: Option<MeterPerModelUnitPositiveRatio>,
    /// _
    #[xml(child = "am3d:preTrans")]
    pub pre_trans_vector3_d: Option<PreTransVector3D>,
    /// _
    #[xml(child = "am3d:scale")]
    pub scale3_d: Option<Scale3D>,
    /// _
    #[xml(child = "am3d:rot")]
    pub rotate3_d: Option<Rotate3D>,
    /// _
    #[xml(child = "am3d:postTrans")]
    pub post_trans_vector3_d: Option<PostTransVector3D>,
    /// _
    #[xml(child = "am3d:extLst")]
    pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Optional source attribution URL describes from whence the 3D model came..
/// When the object is serialized out as xml, it's qualified name is am3d:attrSrcUrl.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "am3d:attrSrcUrl")]
pub struct PictureAttributionSourceUrl {
    /// id
    /// Represents the following attribute in the schema: r:id
    #[xml(attr = "r:id")]
    pub r_id: String,
}
/// Defines the Model3DRaster Class.
/// When the object is serialized out as xml, it's qualified name is am3d:raster.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "am3d:raster")]
pub struct Model3DRaster {
    /// rName
    /// Represents the following attribute in the schema: :rName
    #[xml(attr = "rName")]
    pub r_name: String,
    /// rVer
    /// Represents the following attribute in the schema: :rVer
    #[xml(attr = "rVer")]
    pub r_ver: String,
    /// _
    #[xml(child = "am3d:blip")]
    pub blip: Option<Blip>,
}
/// Future Model3D extensions.
/// When the object is serialized out as xml, it's qualified name is am3d:extLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "am3d:extLst")]
pub struct Model3DExtensionList {
    /// _
    #[xml(child = "am3d:ext")]
    pub am3d_ext: Vec<Model3DExtension>,
}
/// Defines the ObjectViewport Class.
/// When the object is serialized out as xml, it's qualified name is am3d:objViewport.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "am3d:objViewport")]
pub struct ObjectViewport {
    /// viewportSz
    /// Represents the following attribute in the schema: :viewportSz
    #[xml(attr = "viewportSz")]
    pub viewport_sz: i32,
    /// _
    #[xml(child = "am3d:extLst")]
    pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the WindowViewport Class.
/// When the object is serialized out as xml, it's qualified name is am3d:winViewport.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "am3d:winViewport")]
pub struct WindowViewport {
    /// _
    #[xml(child = "am3d:extLst")]
    pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Ambient light in a scene.
/// When the object is serialized out as xml, it's qualified name is am3d:ambientLight.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "am3d:ambientLight")]
pub struct AmbientLight {
    /// enabled
    /// Represents the following attribute in the schema: :enabled
    #[xml(attr = "enabled")]
    pub enabled: Option<bool>,
    /// _
    #[xml(child = "am3d:clr")]
    pub color_type: ColorType,
    /// _
    #[xml(child = "am3d:illuminance")]
    pub illuminance_positive_ratio: IlluminancePositiveRatio,
    /// _
    #[xml(child = "am3d:extLst")]
    pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the PointLight Class.
/// When the object is serialized out as xml, it's qualified name is am3d:ptLight.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "am3d:ptLight")]
pub struct PointLight {
    /// enabled
    /// Represents the following attribute in the schema: :enabled
    #[xml(attr = "enabled")]
    pub enabled: Option<bool>,
    /// rad
    /// Represents the following attribute in the schema: :rad
    #[xml(attr = "rad")]
    pub rad: i32,
    /// _
    #[xml(child = "am3d:clr")]
    pub color_type: ColorType,
    /// _
    #[xml(child = "am3d:intensity")]
    pub intensity_positive_ratio: IntensityPositiveRatio,
    /// _
    #[xml(child = "am3d:pos")]
    pub pos_point3_d: PosPoint3D,
    /// _
    #[xml(child = "am3d:extLst")]
    pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the SpotLight Class.
/// When the object is serialized out as xml, it's qualified name is am3d:spotLight.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "am3d:spotLight")]
pub struct SpotLight {
    /// enabled
    /// Represents the following attribute in the schema: :enabled
    #[xml(attr = "enabled")]
    pub enabled: Option<bool>,
    /// rad
    /// Represents the following attribute in the schema: :rad
    #[xml(attr = "rad")]
    pub rad: i32,
    /// spotAng
    /// Represents the following attribute in the schema: :spotAng
    #[xml(attr = "spotAng")]
    pub spot_ang: i32,
    /// _
    #[xml(child = "am3d:clr")]
    pub color_type: ColorType,
    /// _
    #[xml(child = "am3d:intensity")]
    pub intensity_positive_ratio: IntensityPositiveRatio,
    /// _
    #[xml(child = "am3d:pos")]
    pub pos_point3_d: PosPoint3D,
    /// _
    #[xml(child = "am3d:lookAt")]
    pub look_at_point3_d: LookAtPoint3D,
    /// _
    #[xml(child = "am3d:extLst")]
    pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the DirectionalLight Class.
/// When the object is serialized out as xml, it's qualified name is am3d:dirLight.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "am3d:dirLight")]
pub struct DirectionalLight {
    /// enabled
    /// Represents the following attribute in the schema: :enabled
    #[xml(attr = "enabled")]
    pub enabled: Option<bool>,
    /// angularRad
    /// Represents the following attribute in the schema: :angularRad
    #[xml(attr = "angularRad")]
    pub angular_rad: i32,
    /// _
    #[xml(child = "am3d:clr")]
    pub color_type: ColorType,
    /// _
    #[xml(child = "am3d:illuminance")]
    pub illuminance_positive_ratio: IlluminancePositiveRatio,
    /// _
    #[xml(child = "am3d:pos")]
    pub pos_point3_d: PosPoint3D,
    /// _
    #[xml(child = "am3d:lookAt")]
    pub look_at_point3_d: LookAtPoint3D,
    /// _
    #[xml(child = "am3d:extLst")]
    pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the UnknownLight Class.
/// When the object is serialized out as xml, it's qualified name is am3d:unkLight.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "am3d:unkLight")]
pub struct UnknownLight {}
