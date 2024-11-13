#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum ColorStyleMethodEnum {
    #[default]
    Cycle,
    WithinLinear,
    AcrossLinear,
    WithinLinearReversed,
    AcrossLinearReversed,
}
crate::__string_enum! {
    ColorStyleMethodEnum { Cycle = "cycle", WithinLinear = "withinLinear", AcrossLinear =
    "acrossLinear", WithinLinearReversed = "withinLinearReversed", AcrossLinearReversed =
    "acrossLinearReversed", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum StyleReferenceModifierEnum {
    #[default]
    IgnoreCsTransforms,
}
crate::__string_enum! {
    StyleReferenceModifierEnum { IgnoreCsTransforms = "ignoreCsTransforms", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum StyleColorEnum {
    #[default]
    Automatic,
}
crate::__string_enum! {
    StyleColorEnum { Automatic = "auto", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum StyleEntryModifierEnum {
    #[default]
    AllowNoFillOverride,
    AllowNoLineOverride,
}
crate::__string_enum! {
    StyleEntryModifierEnum { AllowNoFillOverride = "allowNoFillOverride",
    AllowNoLineOverride = "allowNoLineOverride", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum MarkerStyle {
    #[default]
    Circle,
    Dash,
    Diamond,
    Dot,
    Plus,
    Square,
    Star,
    Triangle,
    X,
}
crate::__string_enum! {
    MarkerStyle { Circle = "circle", Dash = "dash", Diamond = "diamond", Dot = "dot",
    Plus = "plus", Square = "square", Star = "star", Triangle = "triangle", X = "x", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum Boolean {
    #[default]
    False,
    True,
    Ninch,
}
crate::__string_enum! {
    Boolean { False = "false", True = "true", Ninch = "ninch", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum TickMarkNinch {
    #[default]
    Cross,
    Inside,
    None,
    Outside,
    Ninch,
}
crate::__string_enum! {
    TickMarkNinch { Cross = "cross", Inside = "inside", None = "none", Outside =
    "outside", Ninch = "ninch", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum TickLabelPositionNinch {
    #[default]
    High,
    Low,
    NextToAxis,
    None,
    Ninch,
}
crate::__string_enum! {
    TickLabelPositionNinch { High = "high", Low = "low", NextToAxis = "nextToAxis", None
    = "none", Ninch = "ninch", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum DataLabelsPosition {
    #[default]
    Center,
    InsideEnd,
    InsideBase,
    OutsideEnd,
    Ninch,
}
crate::__string_enum! {
    DataLabelsPosition { Center = "center", InsideEnd = "insideEnd", InsideBase =
    "insideBase", OutsideEnd = "outsideEnd", Ninch = "ninch", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum LegendPosition {
    #[default]
    Right,
    Top,
    Left,
    Bottom,
    Ninch,
}
crate::__string_enum! {
    LegendPosition { Right = "right", Top = "top", Left = "left", Bottom = "bottom",
    Ninch = "ninch", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum TitlePosition {
    #[default]
    Above,
    Overlay,
    Off,
    Ninch,
}
crate::__string_enum! {
    TitlePosition { Above = "above", Overlay = "overlay", Off = "off", Ninch = "ninch", }
}
/// Defines the ColorStyle Class.
/// When the object is serialized out as xml, it's qualified name is cs:colorStyle.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cs:colorStyle")]
pub struct ColorStyle {
    #[xml(attr = "xmlns", with = "color_style_xmlns_derive")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// meth
    /// Represents the following attribute in the schema: :meth
    #[xml(attr = "meth")]
    pub method: String,
    /// id
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: Option<i32>,
    #[xml(
        child = "a:scrgbClr",
        child = "a:srgbClr",
        child = "a:hslClr",
        child = "a:sysClr",
        child = "a:schemeClr",
        child = "a:prstClr",
        child = "cs:variation",
        child = "cs:extLst",
    )]
    pub children: Vec<ColorStyleChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum ColorStyleChildChoice {
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
    #[xml(tag = "cs:variation")]
    CsVariation(ColorStyleVariation),
    #[xml(tag = "cs:extLst")]
    CsExtLst(OfficeArtExtensionList),
}
mod color_style_xmlns_derive {
    pub fn from_xml(mode: &str) -> hard_xml::XmlResult<String> {
        Ok(mode.to_string())
    }
    pub fn to_xml(_: &String) -> hard_xml::XmlResult<&'static str> {
        Ok("http://schemas.microsoft.com/office/drawing/2012/chartStyle")
    }
}
/// Defines the ChartStyle Class.
/// When the object is serialized out as xml, it's qualified name is cs:chartStyle.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cs:chartStyle")]
pub struct ChartStyle {
    #[xml(attr = "xmlns", with = "chart_style_xmlns_derive")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// id
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: Option<i32>,
    /// _
    #[xml(child = "cs:axisTitle")]
    pub axis_title: AxisTitle,
    /// _
    #[xml(child = "cs:categoryAxis")]
    pub category_axis: CategoryAxis,
    /// _
    #[xml(child = "cs:chartArea")]
    pub chart_area: ChartArea,
    /// _
    #[xml(child = "cs:dataLabel")]
    pub data_label: DataLabel,
    /// _
    #[xml(child = "cs:dataLabelCallout")]
    pub data_label_callout: Option<DataLabelCallout>,
    /// _
    #[xml(child = "cs:dataPoint")]
    pub data_point: DataPoint,
    /// _
    #[xml(child = "cs:dataPoint3D")]
    pub data_point3_d: DataPoint3D,
    /// _
    #[xml(child = "cs:dataPointLine")]
    pub data_point_line: DataPointLine,
    /// _
    #[xml(child = "cs:dataPointMarker")]
    pub data_point_marker: DataPointMarker,
    /// _
    #[xml(child = "cs:dataPointMarkerLayout")]
    pub marker_layout_properties: Option<MarkerLayoutProperties>,
    /// _
    #[xml(child = "cs:dataPointWireframe")]
    pub data_point_wireframe: DataPointWireframe,
    /// _
    #[xml(child = "cs:dataTable")]
    pub data_table_style: DataTableStyle,
    /// _
    #[xml(child = "cs:downBar")]
    pub down_bar: DownBar,
    /// _
    #[xml(child = "cs:dropLine")]
    pub drop_line: DropLine,
    /// _
    #[xml(child = "cs:errorBar")]
    pub error_bar: ErrorBar,
    /// _
    #[xml(child = "cs:floor")]
    pub floor: Floor,
    /// _
    #[xml(child = "cs:gridlineMajor")]
    pub gridline_major: GridlineMajor,
    /// _
    #[xml(child = "cs:gridlineMinor")]
    pub gridline_minor: GridlineMinor,
    /// _
    #[xml(child = "cs:hiLoLine")]
    pub hi_lo_line: HiLoLine,
    /// _
    #[xml(child = "cs:leaderLine")]
    pub leader_line: LeaderLine,
    /// _
    #[xml(child = "cs:legend")]
    pub legend_style: LegendStyle,
    /// _
    #[xml(child = "cs:plotArea")]
    pub plot_area: PlotArea,
    /// _
    #[xml(child = "cs:plotArea3D")]
    pub plot_area3_d: PlotArea3D,
    /// _
    #[xml(child = "cs:seriesAxis")]
    pub series_axis: SeriesAxis,
    /// _
    #[xml(child = "cs:seriesLine")]
    pub series_line: SeriesLine,
    /// _
    #[xml(child = "cs:title")]
    pub title_style: TitleStyle,
    /// _
    #[xml(child = "cs:trendline")]
    pub trendline_style: TrendlineStyle,
    /// _
    #[xml(child = "cs:trendlineLabel")]
    pub trendline_label: TrendlineLabel,
    /// _
    #[xml(child = "cs:upBar")]
    pub up_bar: UpBar,
    /// _
    #[xml(child = "cs:valueAxis")]
    pub value_axis: ValueAxis,
    /// _
    #[xml(child = "cs:wall")]
    pub wall: Wall,
    /// _
    #[xml(child = "cs:extLst")]
    pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
mod chart_style_xmlns_derive {
    pub fn from_xml(mode: &str) -> hard_xml::XmlResult<String> {
        Ok(mode.to_string())
    }
    pub fn to_xml(_: &String) -> hard_xml::XmlResult<&'static str> {
        Ok("http://schemas.microsoft.com/office/drawing/2012/chartStyle")
    }
}
/// Defines the ColorStyleVariation Class.
/// When the object is serialized out as xml, it's qualified name is cs:variation.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cs:variation")]
pub struct ColorStyleVariation {
    #[xml(
        child = "a:tint",
        child = "a:shade",
        child = "a:comp",
        child = "a:inv",
        child = "a:gray",
        child = "a:alpha",
        child = "a:alphaOff",
        child = "a:alphaMod",
        child = "a:hue",
        child = "a:hueOff",
        child = "a:hueMod",
        child = "a:sat",
        child = "a:satOff",
        child = "a:satMod",
        child = "a:lum",
        child = "a:lumOff",
        child = "a:lumMod",
        child = "a:red",
        child = "a:redOff",
        child = "a:redMod",
        child = "a:green",
        child = "a:greenOff",
        child = "a:greenMod",
        child = "a:blue",
        child = "a:blueOff",
        child = "a:blueMod",
        child = "a:gamma",
        child = "a:invGamma",
    )]
    pub children: Vec<ColorStyleVariationChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum ColorStyleVariationChildChoice {
    #[xml(tag = "a:tint")]
    ATint(crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Tint),
    #[xml(tag = "a:shade")]
    AShade(crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Shade),
    #[xml(tag = "a:comp")]
    AComp(crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Complement),
    #[xml(tag = "a:inv")]
    AInv(crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Inverse),
    #[xml(tag = "a:gray")]
    AGray(crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Gray),
    #[xml(tag = "a:alpha")]
    AAlpha(crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Alpha),
    #[xml(tag = "a:alphaOff")]
    AAlphaOff(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::AlphaOffset,
    ),
    #[xml(tag = "a:alphaMod")]
    AAlphaMod(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::AlphaModulation,
    ),
    #[xml(tag = "a:hue")]
    AHue(crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Hue),
    #[xml(tag = "a:hueOff")]
    AHueOff(crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::HueOffset),
    #[xml(tag = "a:hueMod")]
    AHueMod(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::HueModulation,
    ),
    #[xml(tag = "a:sat")]
    ASat(crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Saturation),
    #[xml(tag = "a:satOff")]
    ASatOff(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::SaturationOffset,
    ),
    #[xml(tag = "a:satMod")]
    ASatMod(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::SaturationModulation,
    ),
    #[xml(tag = "a:lum")]
    ALum(crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Luminance),
    #[xml(tag = "a:lumOff")]
    ALumOff(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::LuminanceOffset,
    ),
    #[xml(tag = "a:lumMod")]
    ALumMod(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::LuminanceModulation,
    ),
    #[xml(tag = "a:red")]
    ARed(crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Red),
    #[xml(tag = "a:redOff")]
    ARedOff(crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::RedOffset),
    #[xml(tag = "a:redMod")]
    ARedMod(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::RedModulation,
    ),
    #[xml(tag = "a:green")]
    AGreen(crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Green),
    #[xml(tag = "a:greenOff")]
    AGreenOff(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::GreenOffset,
    ),
    #[xml(tag = "a:greenMod")]
    AGreenMod(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::GreenModulation,
    ),
    #[xml(tag = "a:blue")]
    ABlue(crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Blue),
    #[xml(tag = "a:blueOff")]
    ABlueOff(crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::BlueOffset),
    #[xml(tag = "a:blueMod")]
    ABlueMod(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::BlueModulation,
    ),
    #[xml(tag = "a:gamma")]
    AGamma(crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Gamma),
    #[xml(tag = "a:invGamma")]
    AInvGamma(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::InverseGamma,
    ),
}
/// Defines the OfficeArtExtensionList Class.
/// When the object is serialized out as xml, it's qualified name is cs:extLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cs:extLst")]
pub struct OfficeArtExtensionList {
    #[xml(child = "a:ext")]
    pub children: Vec<OfficeArtExtensionListChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum OfficeArtExtensionListChildChoice {
    #[xml(tag = "a:ext")]
    AExt(crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Extension),
}
/// Defines the StyleColor Class.
/// When the object is serialized out as xml, it's qualified name is cs:styleClr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cs:styleClr")]
pub struct StyleColor {
    /// val
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: Option<String>,
    #[xml(
        child = "a:tint",
        child = "a:shade",
        child = "a:comp",
        child = "a:inv",
        child = "a:gray",
        child = "a:alpha",
        child = "a:alphaOff",
        child = "a:alphaMod",
        child = "a:hue",
        child = "a:hueOff",
        child = "a:hueMod",
        child = "a:sat",
        child = "a:satOff",
        child = "a:satMod",
        child = "a:lum",
        child = "a:lumOff",
        child = "a:lumMod",
        child = "a:red",
        child = "a:redOff",
        child = "a:redMod",
        child = "a:green",
        child = "a:greenOff",
        child = "a:greenMod",
        child = "a:blue",
        child = "a:blueOff",
        child = "a:blueMod",
        child = "a:gamma",
        child = "a:invGamma",
    )]
    pub children: Vec<StyleColorChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum StyleColorChildChoice {
    #[xml(tag = "a:tint")]
    ATint(crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Tint),
    #[xml(tag = "a:shade")]
    AShade(crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Shade),
    #[xml(tag = "a:comp")]
    AComp(crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Complement),
    #[xml(tag = "a:inv")]
    AInv(crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Inverse),
    #[xml(tag = "a:gray")]
    AGray(crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Gray),
    #[xml(tag = "a:alpha")]
    AAlpha(crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Alpha),
    #[xml(tag = "a:alphaOff")]
    AAlphaOff(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::AlphaOffset,
    ),
    #[xml(tag = "a:alphaMod")]
    AAlphaMod(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::AlphaModulation,
    ),
    #[xml(tag = "a:hue")]
    AHue(crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Hue),
    #[xml(tag = "a:hueOff")]
    AHueOff(crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::HueOffset),
    #[xml(tag = "a:hueMod")]
    AHueMod(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::HueModulation,
    ),
    #[xml(tag = "a:sat")]
    ASat(crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Saturation),
    #[xml(tag = "a:satOff")]
    ASatOff(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::SaturationOffset,
    ),
    #[xml(tag = "a:satMod")]
    ASatMod(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::SaturationModulation,
    ),
    #[xml(tag = "a:lum")]
    ALum(crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Luminance),
    #[xml(tag = "a:lumOff")]
    ALumOff(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::LuminanceOffset,
    ),
    #[xml(tag = "a:lumMod")]
    ALumMod(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::LuminanceModulation,
    ),
    #[xml(tag = "a:red")]
    ARed(crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Red),
    #[xml(tag = "a:redOff")]
    ARedOff(crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::RedOffset),
    #[xml(tag = "a:redMod")]
    ARedMod(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::RedModulation,
    ),
    #[xml(tag = "a:green")]
    AGreen(crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Green),
    #[xml(tag = "a:greenOff")]
    AGreenOff(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::GreenOffset,
    ),
    #[xml(tag = "a:greenMod")]
    AGreenMod(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::GreenModulation,
    ),
    #[xml(tag = "a:blue")]
    ABlue(crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Blue),
    #[xml(tag = "a:blueOff")]
    ABlueOff(crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::BlueOffset),
    #[xml(tag = "a:blueMod")]
    ABlueMod(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::BlueModulation,
    ),
    #[xml(tag = "a:gamma")]
    AGamma(crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Gamma),
    #[xml(tag = "a:invGamma")]
    AInvGamma(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::InverseGamma,
    ),
}
/// Defines the LineReference Class.
/// When the object is serialized out as xml, it's qualified name is cs:lnRef.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cs:lnRef")]
pub struct LineReference {
    /// idx
    /// Represents the following attribute in the schema: :idx
    #[xml(attr = "idx")]
    pub index: i32,
    /// mods
    /// Represents the following attribute in the schema: :mods
    #[xml(attr = "mods")]
    pub modifiers: Option<String>,
    #[xml(
        child = "a:scrgbClr",
        child = "a:srgbClr",
        child = "a:hslClr",
        child = "a:sysClr",
        child = "a:schemeClr",
        child = "a:prstClr",
        child = "cs:styleClr",
    )]
    pub children: Vec<LineReferenceChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum LineReferenceChildChoice {
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
    #[xml(tag = "cs:styleClr")]
    CsStyleClr(StyleColor),
}
/// Defines the FillReference Class.
/// When the object is serialized out as xml, it's qualified name is cs:fillRef.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cs:fillRef")]
pub struct FillReference {
    /// idx
    /// Represents the following attribute in the schema: :idx
    #[xml(attr = "idx")]
    pub index: i32,
    /// mods
    /// Represents the following attribute in the schema: :mods
    #[xml(attr = "mods")]
    pub modifiers: Option<String>,
    #[xml(
        child = "a:scrgbClr",
        child = "a:srgbClr",
        child = "a:hslClr",
        child = "a:sysClr",
        child = "a:schemeClr",
        child = "a:prstClr",
        child = "cs:styleClr",
    )]
    pub children: Vec<FillReferenceChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum FillReferenceChildChoice {
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
    #[xml(tag = "cs:styleClr")]
    CsStyleClr(StyleColor),
}
/// Defines the EffectReference Class.
/// When the object is serialized out as xml, it's qualified name is cs:effectRef.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cs:effectRef")]
pub struct EffectReference {
    /// idx
    /// Represents the following attribute in the schema: :idx
    #[xml(attr = "idx")]
    pub index: i32,
    /// mods
    /// Represents the following attribute in the schema: :mods
    #[xml(attr = "mods")]
    pub modifiers: Option<String>,
    #[xml(
        child = "a:scrgbClr",
        child = "a:srgbClr",
        child = "a:hslClr",
        child = "a:sysClr",
        child = "a:schemeClr",
        child = "a:prstClr",
        child = "cs:styleClr",
    )]
    pub children: Vec<EffectReferenceChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum EffectReferenceChildChoice {
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
    #[xml(tag = "cs:styleClr")]
    CsStyleClr(StyleColor),
}
/// Defines the StyleReference Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct StyleReference {
    /// idx
    /// Represents the following attribute in the schema: :idx
    #[xml(attr = "idx")]
    pub index: i32,
    /// mods
    /// Represents the following attribute in the schema: :mods
    #[xml(attr = "mods")]
    pub modifiers: Option<String>,
    #[xml(
        child = "a:scrgbClr",
        child = "a:srgbClr",
        child = "a:hslClr",
        child = "a:sysClr",
        child = "a:schemeClr",
        child = "a:prstClr",
        child = "cs:styleClr",
    )]
    pub children: Vec<StyleReferenceChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum StyleReferenceChildChoice {
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
    #[xml(tag = "cs:styleClr")]
    CsStyleClr(StyleColor),
}
/// Defines the LineWidthScale Class.
/// When the object is serialized out as xml, it's qualified name is cs:lineWidthScale.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cs:lineWidthScale")]
pub struct LineWidthScale {
    #[xml(text)]
    pub child: f64,
}
/// Defines the FontReference Class.
/// When the object is serialized out as xml, it's qualified name is cs:fontRef.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cs:fontRef")]
pub struct FontReference {
    /// idx
    /// Represents the following attribute in the schema: :idx
    #[xml(attr = "idx")]
    pub index: crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::FontCollectionIndexValues,
    /// mods
    /// Represents the following attribute in the schema: :mods
    #[xml(attr = "mods")]
    pub modifiers: Option<String>,
    #[xml(
        child = "a:scrgbClr",
        child = "a:srgbClr",
        child = "a:hslClr",
        child = "a:sysClr",
        child = "a:schemeClr",
        child = "a:prstClr",
        child = "cs:styleClr",
    )]
    pub children: Vec<FontReferenceChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum FontReferenceChildChoice {
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
    #[xml(tag = "cs:styleClr")]
    CsStyleClr(StyleColor),
}
/// Defines the ShapeProperties Class.
/// When the object is serialized out as xml, it's qualified name is cs:spPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cs:spPr")]
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
/// Defines the TextCharacterPropertiesType Class.
/// When the object is serialized out as xml, it's qualified name is cs:defRPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cs:defRPr")]
pub struct TextCharacterPropertiesType {
    /// kumimoji
    /// Represents the following attribute in the schema: :kumimoji
    #[xml(attr = "kumimoji")]
    pub kumimoji: Option<bool>,
    /// lang
    /// Represents the following attribute in the schema: :lang
    #[xml(attr = "lang")]
    pub language: Option<String>,
    /// altLang
    /// Represents the following attribute in the schema: :altLang
    #[xml(attr = "altLang")]
    pub alternative_language: Option<String>,
    /// sz
    /// Represents the following attribute in the schema: :sz
    #[xml(attr = "sz")]
    pub font_size: Option<i32>,
    /// b
    /// Represents the following attribute in the schema: :b
    #[xml(attr = "b")]
    pub bold: Option<bool>,
    /// i
    /// Represents the following attribute in the schema: :i
    #[xml(attr = "i")]
    pub italic: Option<bool>,
    /// u
    /// Represents the following attribute in the schema: :u
    #[xml(attr = "u")]
    pub underline: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::TextUnderlineValues,
    >,
    /// strike
    /// Represents the following attribute in the schema: :strike
    #[xml(attr = "strike")]
    pub strike: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::TextStrikeValues,
    >,
    /// kern
    /// Represents the following attribute in the schema: :kern
    #[xml(attr = "kern")]
    pub kerning: Option<i32>,
    /// cap
    /// Represents the following attribute in the schema: :cap
    #[xml(attr = "cap")]
    pub capital: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::TextCapsValues,
    >,
    /// spc
    /// Represents the following attribute in the schema: :spc
    #[xml(attr = "spc")]
    pub spacing: Option<i32>,
    /// normalizeH
    /// Represents the following attribute in the schema: :normalizeH
    #[xml(attr = "normalizeH")]
    pub normalize_height: Option<bool>,
    /// baseline
    /// Represents the following attribute in the schema: :baseline
    #[xml(attr = "baseline")]
    pub baseline: Option<i32>,
    /// noProof
    /// Represents the following attribute in the schema: :noProof
    #[xml(attr = "noProof")]
    pub no_proof: Option<bool>,
    /// dirty
    /// Represents the following attribute in the schema: :dirty
    #[xml(attr = "dirty")]
    pub dirty: Option<bool>,
    /// err
    /// Represents the following attribute in the schema: :err
    #[xml(attr = "err")]
    pub spelling_error: Option<bool>,
    /// smtClean
    /// Represents the following attribute in the schema: :smtClean
    #[xml(attr = "smtClean")]
    pub smart_tag_clean: Option<bool>,
    /// smtId
    /// Represents the following attribute in the schema: :smtId
    #[xml(attr = "smtId")]
    pub smart_tag_id: Option<i32>,
    /// bmk
    /// Represents the following attribute in the schema: :bmk
    #[xml(attr = "bmk")]
    pub bookmark: Option<String>,
    #[xml(
        child = "a:ln",
        child = "a:noFill",
        child = "a:solidFill",
        child = "a:gradFill",
        child = "a:blipFill",
        child = "a:pattFill",
        child = "a:grpFill",
        child = "a:effectLst",
        child = "a:effectDag",
        child = "a:highlight",
        child = "a:uLnTx",
        child = "a:uLn",
        child = "a:uFillTx",
        child = "a:uFill",
        child = "a:latin",
        child = "a:ea",
        child = "a:cs",
        child = "a:sym",
        child = "a:hlinkClick",
        child = "a:hlinkMouseOver",
        child = "a:rtl",
        child = "a:extLst",
    )]
    pub children: Vec<TextCharacterPropertiesTypeChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum TextCharacterPropertiesTypeChildChoice {
    #[xml(tag = "a:ln")]
    ALn(crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Outline),
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
    #[xml(tag = "a:highlight")]
    AHighlight(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Highlight,
    ),
    #[xml(tag = "a:uLnTx")]
    AULnTx(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::UnderlineFollowsText,
    ),
    #[xml(tag = "a:uLn")]
    AULn(crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Underline),
    #[xml(tag = "a:uFillTx")]
    AUFillTx(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::UnderlineFillText,
    ),
    #[xml(tag = "a:uFill")]
    AUFill(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::UnderlineFill,
    ),
    #[xml(tag = "a:latin")]
    ALatin(crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::LatinFont),
    #[xml(tag = "a:ea")]
    AEa(crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::EastAsianFont),
    #[xml(tag = "a:cs")]
    ACs(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ComplexScriptFont,
    ),
    #[xml(tag = "a:sym")]
    ASym(crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::SymbolFont),
    #[xml(tag = "a:hlinkClick")]
    AHlinkClick(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::HyperlinkOnClick,
    ),
    #[xml(tag = "a:hlinkMouseOver")]
    AHlinkMouseOver(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::HyperlinkOnMouseOver,
    ),
    #[xml(tag = "a:rtl")]
    ARtl(crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::RightToLeft),
    #[xml(tag = "a:extLst")]
    AExtLst(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ExtensionList,
    ),
}
/// Defines the TextBodyProperties Class.
/// When the object is serialized out as xml, it's qualified name is cs:bodyPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cs:bodyPr")]
pub struct TextBodyProperties {
    /// Rotation
    /// Represents the following attribute in the schema: :rot
    #[xml(attr = "rot")]
    pub rotation: Option<i32>,
    /// Paragraph Spacing
    /// Represents the following attribute in the schema: :spcFirstLastPara
    #[xml(attr = "spcFirstLastPara")]
    pub use_paragraph_spacing: Option<bool>,
    /// Text Vertical Overflow
    /// Represents the following attribute in the schema: :vertOverflow
    #[xml(attr = "vertOverflow")]
    pub vertical_overflow: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::TextVerticalOverflowValues,
    >,
    /// Text Horizontal Overflow
    /// Represents the following attribute in the schema: :horzOverflow
    #[xml(attr = "horzOverflow")]
    pub horizontal_overflow: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::TextHorizontalOverflowValues,
    >,
    /// Vertical Text
    /// Represents the following attribute in the schema: :vert
    #[xml(attr = "vert")]
    pub vertical: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::TextVerticalValues,
    >,
    /// Text Wrapping Type
    /// Represents the following attribute in the schema: :wrap
    #[xml(attr = "wrap")]
    pub wrap: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::TextWrappingValues,
    >,
    /// Left Inset
    /// Represents the following attribute in the schema: :lIns
    #[xml(attr = "lIns")]
    pub left_inset: Option<i32>,
    /// Top Inset
    /// Represents the following attribute in the schema: :tIns
    #[xml(attr = "tIns")]
    pub top_inset: Option<i32>,
    /// Right Inset
    /// Represents the following attribute in the schema: :rIns
    #[xml(attr = "rIns")]
    pub right_inset: Option<i32>,
    /// Bottom Inset
    /// Represents the following attribute in the schema: :bIns
    #[xml(attr = "bIns")]
    pub bottom_inset: Option<i32>,
    /// Number of Columns
    /// Represents the following attribute in the schema: :numCol
    #[xml(attr = "numCol")]
    pub column_count: Option<i32>,
    /// Space Between Columns
    /// Represents the following attribute in the schema: :spcCol
    #[xml(attr = "spcCol")]
    pub column_spacing: Option<i32>,
    /// Columns Right-To-Left
    /// Represents the following attribute in the schema: :rtlCol
    #[xml(attr = "rtlCol")]
    pub right_to_left_columns: Option<bool>,
    /// From WordArt
    /// Represents the following attribute in the schema: :fromWordArt
    #[xml(attr = "fromWordArt")]
    pub from_word_art: Option<bool>,
    /// Anchor
    /// Represents the following attribute in the schema: :anchor
    #[xml(attr = "anchor")]
    pub anchor: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::TextAnchoringTypeValues,
    >,
    /// Anchor Center
    /// Represents the following attribute in the schema: :anchorCtr
    #[xml(attr = "anchorCtr")]
    pub anchor_center: Option<bool>,
    /// Force Anti-Alias
    /// Represents the following attribute in the schema: :forceAA
    #[xml(attr = "forceAA")]
    pub force_anti_alias: Option<bool>,
    /// Text Upright
    /// Represents the following attribute in the schema: :upright
    #[xml(attr = "upright")]
    pub up_right: Option<bool>,
    /// Compatible Line Spacing
    /// Represents the following attribute in the schema: :compatLnSpc
    #[xml(attr = "compatLnSpc")]
    pub compatible_line_spacing: Option<bool>,
    #[xml(
        child = "a:prstTxWarp",
        child = "a:noAutofit",
        child = "a:normAutofit",
        child = "a:spAutoFit",
        child = "a:scene3d",
        child = "a:sp3d",
        child = "a:flatTx",
        child = "a:extLst",
    )]
    pub children: Vec<TextBodyPropertiesChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum TextBodyPropertiesChildChoice {
    #[xml(tag = "a:prstTxWarp")]
    APrstTxWarp(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::PresetTextWarp,
    ),
    #[xml(tag = "a:noAutofit")]
    ANoAutofit(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::NoAutoFit,
    ),
    #[xml(tag = "a:normAutofit")]
    ANormAutofit(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::NormalAutoFit,
    ),
    #[xml(tag = "a:spAutoFit")]
    ASpAutoFit(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ShapeAutoFit,
    ),
    #[xml(tag = "a:scene3d")]
    AScene3d(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Scene3DType,
    ),
    #[xml(tag = "a:sp3d")]
    ASp3d(crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Shape3DType),
    #[xml(tag = "a:flatTx")]
    AFlatTx(crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::FlatText),
    #[xml(tag = "a:extLst")]
    AExtLst(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ExtensionList,
    ),
}
/// Defines the CategoryAxisProperties Class.
/// When the object is serialized out as xml, it's qualified name is cs:categoryAxis.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cs:categoryAxis")]
pub struct CategoryAxisProperties {
    /// visible
    /// Represents the following attribute in the schema: :visible
    #[xml(attr = "visible")]
    pub visible: Option<Boolean>,
    /// majorTick
    /// Represents the following attribute in the schema: :majorTick
    #[xml(attr = "majorTick")]
    pub major_tick: Option<TickMarkNinch>,
    /// minorTick
    /// Represents the following attribute in the schema: :minorTick
    #[xml(attr = "minorTick")]
    pub minor_tick_prop: Option<TickMarkNinch>,
    /// labelPosition
    /// Represents the following attribute in the schema: :labelPosition
    #[xml(attr = "labelPosition")]
    pub label_position: Option<TickLabelPositionNinch>,
    /// majorGridlines
    /// Represents the following attribute in the schema: :majorGridlines
    #[xml(attr = "majorGridlines")]
    pub major_gridlines: Option<Boolean>,
    /// minorGridlines
    /// Represents the following attribute in the schema: :minorGridlines
    #[xml(attr = "minorGridlines")]
    pub minor_gridlines_prop: Option<Boolean>,
    /// title
    /// Represents the following attribute in the schema: :title
    #[xml(attr = "title")]
    pub title_prop: Option<Boolean>,
}
/// Defines the SeriesAxisProperties Class.
/// When the object is serialized out as xml, it's qualified name is cs:seriesAxis.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cs:seriesAxis")]
pub struct SeriesAxisProperties {
    /// visible
    /// Represents the following attribute in the schema: :visible
    #[xml(attr = "visible")]
    pub visible: Option<Boolean>,
    /// majorTick
    /// Represents the following attribute in the schema: :majorTick
    #[xml(attr = "majorTick")]
    pub major_tick: Option<TickMarkNinch>,
    /// minorTick
    /// Represents the following attribute in the schema: :minorTick
    #[xml(attr = "minorTick")]
    pub minor_tick_prop: Option<TickMarkNinch>,
    /// labelPosition
    /// Represents the following attribute in the schema: :labelPosition
    #[xml(attr = "labelPosition")]
    pub label_position: Option<TickLabelPositionNinch>,
    /// majorGridlines
    /// Represents the following attribute in the schema: :majorGridlines
    #[xml(attr = "majorGridlines")]
    pub major_gridlines: Option<Boolean>,
    /// minorGridlines
    /// Represents the following attribute in the schema: :minorGridlines
    #[xml(attr = "minorGridlines")]
    pub minor_gridlines_prop: Option<Boolean>,
    /// title
    /// Represents the following attribute in the schema: :title
    #[xml(attr = "title")]
    pub title_prop: Option<Boolean>,
}
/// Defines the ValueAxisProperties Class.
/// When the object is serialized out as xml, it's qualified name is cs:valueAxis.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cs:valueAxis")]
pub struct ValueAxisProperties {
    /// visible
    /// Represents the following attribute in the schema: :visible
    #[xml(attr = "visible")]
    pub visible: Option<Boolean>,
    /// majorTick
    /// Represents the following attribute in the schema: :majorTick
    #[xml(attr = "majorTick")]
    pub major_tick: Option<TickMarkNinch>,
    /// minorTick
    /// Represents the following attribute in the schema: :minorTick
    #[xml(attr = "minorTick")]
    pub minor_tick_prop: Option<TickMarkNinch>,
    /// labelPosition
    /// Represents the following attribute in the schema: :labelPosition
    #[xml(attr = "labelPosition")]
    pub label_position: Option<TickLabelPositionNinch>,
    /// majorGridlines
    /// Represents the following attribute in the schema: :majorGridlines
    #[xml(attr = "majorGridlines")]
    pub major_gridlines: Option<Boolean>,
    /// minorGridlines
    /// Represents the following attribute in the schema: :minorGridlines
    #[xml(attr = "minorGridlines")]
    pub minor_gridlines_prop: Option<Boolean>,
    /// title
    /// Represents the following attribute in the schema: :title
    #[xml(attr = "title")]
    pub title_prop: Option<Boolean>,
}
/// Defines the AxisProperties Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct AxisProperties {
    /// visible
    /// Represents the following attribute in the schema: :visible
    #[xml(attr = "visible")]
    pub visible: Option<Boolean>,
    /// majorTick
    /// Represents the following attribute in the schema: :majorTick
    #[xml(attr = "majorTick")]
    pub major_tick: Option<TickMarkNinch>,
    /// minorTick
    /// Represents the following attribute in the schema: :minorTick
    #[xml(attr = "minorTick")]
    pub minor_tick_prop: Option<TickMarkNinch>,
    /// labelPosition
    /// Represents the following attribute in the schema: :labelPosition
    #[xml(attr = "labelPosition")]
    pub label_position: Option<TickLabelPositionNinch>,
    /// majorGridlines
    /// Represents the following attribute in the schema: :majorGridlines
    #[xml(attr = "majorGridlines")]
    pub major_gridlines: Option<Boolean>,
    /// minorGridlines
    /// Represents the following attribute in the schema: :minorGridlines
    #[xml(attr = "minorGridlines")]
    pub minor_gridlines_prop: Option<Boolean>,
    /// title
    /// Represents the following attribute in the schema: :title
    #[xml(attr = "title")]
    pub title_prop: Option<Boolean>,
}
/// Defines the DataSeries Class.
/// When the object is serialized out as xml, it's qualified name is cs:dataSeries.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cs:dataSeries")]
pub struct DataSeries {
    /// overlap
    /// Represents the following attribute in the schema: :overlap
    #[xml(attr = "overlap")]
    pub overlap: Option<u8>,
    /// gapWidth
    /// Represents the following attribute in the schema: :gapWidth
    #[xml(attr = "gapWidth")]
    pub gap_width: Option<i16>,
    /// gapDepth
    /// Represents the following attribute in the schema: :gapDepth
    #[xml(attr = "gapDepth")]
    pub gap_depth: Option<i16>,
    /// doughnutHoleSize
    /// Represents the following attribute in the schema: :doughnutHoleSize
    #[xml(attr = "doughnutHoleSize")]
    pub doughnut_hole_size: Option<u8>,
    /// markerVisible
    /// Represents the following attribute in the schema: :markerVisible
    #[xml(attr = "markerVisible")]
    pub marker_visible: Option<Boolean>,
    /// hiloLines
    /// Represents the following attribute in the schema: :hiloLines
    #[xml(attr = "hiloLines")]
    pub hilo_lines: Option<Boolean>,
    /// dropLines
    /// Represents the following attribute in the schema: :dropLines
    #[xml(attr = "dropLines")]
    pub drop_lines: Option<Boolean>,
    /// seriesLines
    /// Represents the following attribute in the schema: :seriesLines
    #[xml(attr = "seriesLines")]
    pub series_lines: Option<Boolean>,
}
/// Defines the DataLabels Class.
/// When the object is serialized out as xml, it's qualified name is cs:dataLabels.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cs:dataLabels")]
pub struct DataLabels {
    /// position
    /// Represents the following attribute in the schema: :position
    #[xml(attr = "position")]
    pub position: Option<DataLabelsPosition>,
    /// value
    /// Represents the following attribute in the schema: :value
    #[xml(attr = "value")]
    pub value: Option<Boolean>,
    /// seriesName
    /// Represents the following attribute in the schema: :seriesName
    #[xml(attr = "seriesName")]
    pub series_name: Option<Boolean>,
    /// categoryName
    /// Represents the following attribute in the schema: :categoryName
    #[xml(attr = "categoryName")]
    pub category_name: Option<Boolean>,
    /// legendKey
    /// Represents the following attribute in the schema: :legendKey
    #[xml(attr = "legendKey")]
    pub legend_key: Option<Boolean>,
    /// percentage
    /// Represents the following attribute in the schema: :percentage
    #[xml(attr = "percentage")]
    pub percentage: Option<Boolean>,
}
/// Defines the DataTable Class.
/// When the object is serialized out as xml, it's qualified name is cs:dataTable.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cs:dataTable")]
pub struct DataTable {
    /// legendKeys
    /// Represents the following attribute in the schema: :legendKeys
    #[xml(attr = "legendKeys")]
    pub legend_keys: Option<Boolean>,
    /// horizontalBorder
    /// Represents the following attribute in the schema: :horizontalBorder
    #[xml(attr = "horizontalBorder")]
    pub horizontal_border: Option<Boolean>,
    /// verticalBorder
    /// Represents the following attribute in the schema: :verticalBorder
    #[xml(attr = "verticalBorder")]
    pub vertical_border: Option<Boolean>,
    /// outlineBorder
    /// Represents the following attribute in the schema: :outlineBorder
    #[xml(attr = "outlineBorder")]
    pub outline_border: Option<Boolean>,
}
/// Defines the Legend Class.
/// When the object is serialized out as xml, it's qualified name is cs:legend.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cs:legend")]
pub struct Legend {
    /// visible
    /// Represents the following attribute in the schema: :visible
    #[xml(attr = "visible")]
    pub visible: Option<Boolean>,
    /// includeInLayout
    /// Represents the following attribute in the schema: :includeInLayout
    #[xml(attr = "includeInLayout")]
    pub include_in_layout: Option<Boolean>,
    /// position
    /// Represents the following attribute in the schema: :position
    #[xml(attr = "position")]
    pub position: Option<LegendPosition>,
}
/// Defines the Title Class.
/// When the object is serialized out as xml, it's qualified name is cs:title.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cs:title")]
pub struct Title {
    /// position
    /// Represents the following attribute in the schema: :position
    #[xml(attr = "position")]
    pub position: Option<TitlePosition>,
}
/// Defines the Trendline Class.
/// When the object is serialized out as xml, it's qualified name is cs:trendline.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cs:trendline")]
pub struct Trendline {
    /// add
    /// Represents the following attribute in the schema: :add
    #[xml(attr = "add")]
    pub add: Option<Boolean>,
    /// equation
    /// Represents the following attribute in the schema: :equation
    #[xml(attr = "equation")]
    pub equation: Option<Boolean>,
    /// rsquared
    /// Represents the following attribute in the schema: :rsquared
    #[xml(attr = "rsquared")]
    pub r_squared: Option<Boolean>,
}
/// Defines the View3DProperties Class.
/// When the object is serialized out as xml, it's qualified name is cs:view3D.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cs:view3D")]
pub struct View3DProperties {
    /// rotX
    /// Represents the following attribute in the schema: :rotX
    #[xml(attr = "rotX")]
    pub rot_x: Option<u8>,
    /// rotY
    /// Represents the following attribute in the schema: :rotY
    #[xml(attr = "rotY")]
    pub rot_y: Option<i16>,
    /// rAngAx
    /// Represents the following attribute in the schema: :rAngAx
    #[xml(attr = "rAngAx")]
    pub right_angle_axes: Option<Boolean>,
    /// perspective
    /// Represents the following attribute in the schema: :perspective
    #[xml(attr = "perspective")]
    pub perspective: Option<u8>,
    /// heightPercent
    /// Represents the following attribute in the schema: :heightPercent
    #[xml(attr = "heightPercent")]
    pub height_percent: Option<i16>,
    /// depthPercent
    /// Represents the following attribute in the schema: :depthPercent
    #[xml(attr = "depthPercent")]
    pub depth_percent: Option<i16>,
}
/// Defines the AxisTitle Class.
/// When the object is serialized out as xml, it's qualified name is cs:axisTitle.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cs:axisTitle")]
pub struct AxisTitle {
    /// mods
    /// Represents the following attribute in the schema: :mods
    #[xml(attr = "mods")]
    pub modifiers: Option<String>,
    /// _
    #[xml(child = "cs:lnRef")]
    pub line_reference: LineReference,
    /// _
    #[xml(child = "cs:lineWidthScale")]
    pub line_width_scale: Option<LineWidthScale>,
    /// _
    #[xml(child = "cs:fillRef")]
    pub fill_reference: FillReference,
    /// _
    #[xml(child = "cs:effectRef")]
    pub effect_reference: EffectReference,
    /// _
    #[xml(child = "cs:fontRef")]
    pub font_reference: FontReference,
    /// _
    #[xml(child = "cs:spPr")]
    pub shape_properties: Option<ShapeProperties>,
    /// _
    #[xml(child = "cs:defRPr")]
    pub text_character_properties_type: Option<TextCharacterPropertiesType>,
    /// _
    #[xml(child = "cs:bodyPr")]
    pub text_body_properties: Option<TextBodyProperties>,
    /// _
    #[xml(child = "cs:extLst")]
    pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the CategoryAxis Class.
/// When the object is serialized out as xml, it's qualified name is cs:categoryAxis.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cs:categoryAxis")]
pub struct CategoryAxis {
    /// mods
    /// Represents the following attribute in the schema: :mods
    #[xml(attr = "mods")]
    pub modifiers: Option<String>,
    /// _
    #[xml(child = "cs:lnRef")]
    pub line_reference: LineReference,
    /// _
    #[xml(child = "cs:lineWidthScale")]
    pub line_width_scale: Option<LineWidthScale>,
    /// _
    #[xml(child = "cs:fillRef")]
    pub fill_reference: FillReference,
    /// _
    #[xml(child = "cs:effectRef")]
    pub effect_reference: EffectReference,
    /// _
    #[xml(child = "cs:fontRef")]
    pub font_reference: FontReference,
    /// _
    #[xml(child = "cs:spPr")]
    pub shape_properties: Option<ShapeProperties>,
    /// _
    #[xml(child = "cs:defRPr")]
    pub text_character_properties_type: Option<TextCharacterPropertiesType>,
    /// _
    #[xml(child = "cs:bodyPr")]
    pub text_body_properties: Option<TextBodyProperties>,
    /// _
    #[xml(child = "cs:extLst")]
    pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the ChartArea Class.
/// When the object is serialized out as xml, it's qualified name is cs:chartArea.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cs:chartArea")]
pub struct ChartArea {
    /// mods
    /// Represents the following attribute in the schema: :mods
    #[xml(attr = "mods")]
    pub modifiers: Option<String>,
    /// _
    #[xml(child = "cs:lnRef")]
    pub line_reference: LineReference,
    /// _
    #[xml(child = "cs:lineWidthScale")]
    pub line_width_scale: Option<LineWidthScale>,
    /// _
    #[xml(child = "cs:fillRef")]
    pub fill_reference: FillReference,
    /// _
    #[xml(child = "cs:effectRef")]
    pub effect_reference: EffectReference,
    /// _
    #[xml(child = "cs:fontRef")]
    pub font_reference: FontReference,
    /// _
    #[xml(child = "cs:spPr")]
    pub shape_properties: Option<ShapeProperties>,
    /// _
    #[xml(child = "cs:defRPr")]
    pub text_character_properties_type: Option<TextCharacterPropertiesType>,
    /// _
    #[xml(child = "cs:bodyPr")]
    pub text_body_properties: Option<TextBodyProperties>,
    /// _
    #[xml(child = "cs:extLst")]
    pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the DataLabel Class.
/// When the object is serialized out as xml, it's qualified name is cs:dataLabel.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cs:dataLabel")]
pub struct DataLabel {
    /// mods
    /// Represents the following attribute in the schema: :mods
    #[xml(attr = "mods")]
    pub modifiers: Option<String>,
    /// _
    #[xml(child = "cs:lnRef")]
    pub line_reference: LineReference,
    /// _
    #[xml(child = "cs:lineWidthScale")]
    pub line_width_scale: Option<LineWidthScale>,
    /// _
    #[xml(child = "cs:fillRef")]
    pub fill_reference: FillReference,
    /// _
    #[xml(child = "cs:effectRef")]
    pub effect_reference: EffectReference,
    /// _
    #[xml(child = "cs:fontRef")]
    pub font_reference: FontReference,
    /// _
    #[xml(child = "cs:spPr")]
    pub shape_properties: Option<ShapeProperties>,
    /// _
    #[xml(child = "cs:defRPr")]
    pub text_character_properties_type: Option<TextCharacterPropertiesType>,
    /// _
    #[xml(child = "cs:bodyPr")]
    pub text_body_properties: Option<TextBodyProperties>,
    /// _
    #[xml(child = "cs:extLst")]
    pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the DataLabelCallout Class.
/// When the object is serialized out as xml, it's qualified name is cs:dataLabelCallout.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cs:dataLabelCallout")]
pub struct DataLabelCallout {
    /// mods
    /// Represents the following attribute in the schema: :mods
    #[xml(attr = "mods")]
    pub modifiers: Option<String>,
    /// _
    #[xml(child = "cs:lnRef")]
    pub line_reference: LineReference,
    /// _
    #[xml(child = "cs:lineWidthScale")]
    pub line_width_scale: Option<LineWidthScale>,
    /// _
    #[xml(child = "cs:fillRef")]
    pub fill_reference: FillReference,
    /// _
    #[xml(child = "cs:effectRef")]
    pub effect_reference: EffectReference,
    /// _
    #[xml(child = "cs:fontRef")]
    pub font_reference: FontReference,
    /// _
    #[xml(child = "cs:spPr")]
    pub shape_properties: Option<ShapeProperties>,
    /// _
    #[xml(child = "cs:defRPr")]
    pub text_character_properties_type: Option<TextCharacterPropertiesType>,
    /// _
    #[xml(child = "cs:bodyPr")]
    pub text_body_properties: Option<TextBodyProperties>,
    /// _
    #[xml(child = "cs:extLst")]
    pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the DataPoint Class.
/// When the object is serialized out as xml, it's qualified name is cs:dataPoint.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cs:dataPoint")]
pub struct DataPoint {
    /// mods
    /// Represents the following attribute in the schema: :mods
    #[xml(attr = "mods")]
    pub modifiers: Option<String>,
    /// _
    #[xml(child = "cs:lnRef")]
    pub line_reference: LineReference,
    /// _
    #[xml(child = "cs:lineWidthScale")]
    pub line_width_scale: Option<LineWidthScale>,
    /// _
    #[xml(child = "cs:fillRef")]
    pub fill_reference: FillReference,
    /// _
    #[xml(child = "cs:effectRef")]
    pub effect_reference: EffectReference,
    /// _
    #[xml(child = "cs:fontRef")]
    pub font_reference: FontReference,
    /// _
    #[xml(child = "cs:spPr")]
    pub shape_properties: Option<ShapeProperties>,
    /// _
    #[xml(child = "cs:defRPr")]
    pub text_character_properties_type: Option<TextCharacterPropertiesType>,
    /// _
    #[xml(child = "cs:bodyPr")]
    pub text_body_properties: Option<TextBodyProperties>,
    /// _
    #[xml(child = "cs:extLst")]
    pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the DataPoint3D Class.
/// When the object is serialized out as xml, it's qualified name is cs:dataPoint3D.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cs:dataPoint3D")]
pub struct DataPoint3D {
    /// mods
    /// Represents the following attribute in the schema: :mods
    #[xml(attr = "mods")]
    pub modifiers: Option<String>,
    /// _
    #[xml(child = "cs:lnRef")]
    pub line_reference: LineReference,
    /// _
    #[xml(child = "cs:lineWidthScale")]
    pub line_width_scale: Option<LineWidthScale>,
    /// _
    #[xml(child = "cs:fillRef")]
    pub fill_reference: FillReference,
    /// _
    #[xml(child = "cs:effectRef")]
    pub effect_reference: EffectReference,
    /// _
    #[xml(child = "cs:fontRef")]
    pub font_reference: FontReference,
    /// _
    #[xml(child = "cs:spPr")]
    pub shape_properties: Option<ShapeProperties>,
    /// _
    #[xml(child = "cs:defRPr")]
    pub text_character_properties_type: Option<TextCharacterPropertiesType>,
    /// _
    #[xml(child = "cs:bodyPr")]
    pub text_body_properties: Option<TextBodyProperties>,
    /// _
    #[xml(child = "cs:extLst")]
    pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the DataPointLine Class.
/// When the object is serialized out as xml, it's qualified name is cs:dataPointLine.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cs:dataPointLine")]
pub struct DataPointLine {
    /// mods
    /// Represents the following attribute in the schema: :mods
    #[xml(attr = "mods")]
    pub modifiers: Option<String>,
    /// _
    #[xml(child = "cs:lnRef")]
    pub line_reference: LineReference,
    /// _
    #[xml(child = "cs:lineWidthScale")]
    pub line_width_scale: Option<LineWidthScale>,
    /// _
    #[xml(child = "cs:fillRef")]
    pub fill_reference: FillReference,
    /// _
    #[xml(child = "cs:effectRef")]
    pub effect_reference: EffectReference,
    /// _
    #[xml(child = "cs:fontRef")]
    pub font_reference: FontReference,
    /// _
    #[xml(child = "cs:spPr")]
    pub shape_properties: Option<ShapeProperties>,
    /// _
    #[xml(child = "cs:defRPr")]
    pub text_character_properties_type: Option<TextCharacterPropertiesType>,
    /// _
    #[xml(child = "cs:bodyPr")]
    pub text_body_properties: Option<TextBodyProperties>,
    /// _
    #[xml(child = "cs:extLst")]
    pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the DataPointMarker Class.
/// When the object is serialized out as xml, it's qualified name is cs:dataPointMarker.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cs:dataPointMarker")]
pub struct DataPointMarker {
    /// mods
    /// Represents the following attribute in the schema: :mods
    #[xml(attr = "mods")]
    pub modifiers: Option<String>,
    /// _
    #[xml(child = "cs:lnRef")]
    pub line_reference: LineReference,
    /// _
    #[xml(child = "cs:lineWidthScale")]
    pub line_width_scale: Option<LineWidthScale>,
    /// _
    #[xml(child = "cs:fillRef")]
    pub fill_reference: FillReference,
    /// _
    #[xml(child = "cs:effectRef")]
    pub effect_reference: EffectReference,
    /// _
    #[xml(child = "cs:fontRef")]
    pub font_reference: FontReference,
    /// _
    #[xml(child = "cs:spPr")]
    pub shape_properties: Option<ShapeProperties>,
    /// _
    #[xml(child = "cs:defRPr")]
    pub text_character_properties_type: Option<TextCharacterPropertiesType>,
    /// _
    #[xml(child = "cs:bodyPr")]
    pub text_body_properties: Option<TextBodyProperties>,
    /// _
    #[xml(child = "cs:extLst")]
    pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the DataPointWireframe Class.
/// When the object is serialized out as xml, it's qualified name is cs:dataPointWireframe.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cs:dataPointWireframe")]
pub struct DataPointWireframe {
    /// mods
    /// Represents the following attribute in the schema: :mods
    #[xml(attr = "mods")]
    pub modifiers: Option<String>,
    /// _
    #[xml(child = "cs:lnRef")]
    pub line_reference: LineReference,
    /// _
    #[xml(child = "cs:lineWidthScale")]
    pub line_width_scale: Option<LineWidthScale>,
    /// _
    #[xml(child = "cs:fillRef")]
    pub fill_reference: FillReference,
    /// _
    #[xml(child = "cs:effectRef")]
    pub effect_reference: EffectReference,
    /// _
    #[xml(child = "cs:fontRef")]
    pub font_reference: FontReference,
    /// _
    #[xml(child = "cs:spPr")]
    pub shape_properties: Option<ShapeProperties>,
    /// _
    #[xml(child = "cs:defRPr")]
    pub text_character_properties_type: Option<TextCharacterPropertiesType>,
    /// _
    #[xml(child = "cs:bodyPr")]
    pub text_body_properties: Option<TextBodyProperties>,
    /// _
    #[xml(child = "cs:extLst")]
    pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the DataTableStyle Class.
/// When the object is serialized out as xml, it's qualified name is cs:dataTable.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cs:dataTable")]
pub struct DataTableStyle {
    /// mods
    /// Represents the following attribute in the schema: :mods
    #[xml(attr = "mods")]
    pub modifiers: Option<String>,
    /// _
    #[xml(child = "cs:lnRef")]
    pub line_reference: LineReference,
    /// _
    #[xml(child = "cs:lineWidthScale")]
    pub line_width_scale: Option<LineWidthScale>,
    /// _
    #[xml(child = "cs:fillRef")]
    pub fill_reference: FillReference,
    /// _
    #[xml(child = "cs:effectRef")]
    pub effect_reference: EffectReference,
    /// _
    #[xml(child = "cs:fontRef")]
    pub font_reference: FontReference,
    /// _
    #[xml(child = "cs:spPr")]
    pub shape_properties: Option<ShapeProperties>,
    /// _
    #[xml(child = "cs:defRPr")]
    pub text_character_properties_type: Option<TextCharacterPropertiesType>,
    /// _
    #[xml(child = "cs:bodyPr")]
    pub text_body_properties: Option<TextBodyProperties>,
    /// _
    #[xml(child = "cs:extLst")]
    pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the DownBar Class.
/// When the object is serialized out as xml, it's qualified name is cs:downBar.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cs:downBar")]
pub struct DownBar {
    /// mods
    /// Represents the following attribute in the schema: :mods
    #[xml(attr = "mods")]
    pub modifiers: Option<String>,
    /// _
    #[xml(child = "cs:lnRef")]
    pub line_reference: LineReference,
    /// _
    #[xml(child = "cs:lineWidthScale")]
    pub line_width_scale: Option<LineWidthScale>,
    /// _
    #[xml(child = "cs:fillRef")]
    pub fill_reference: FillReference,
    /// _
    #[xml(child = "cs:effectRef")]
    pub effect_reference: EffectReference,
    /// _
    #[xml(child = "cs:fontRef")]
    pub font_reference: FontReference,
    /// _
    #[xml(child = "cs:spPr")]
    pub shape_properties: Option<ShapeProperties>,
    /// _
    #[xml(child = "cs:defRPr")]
    pub text_character_properties_type: Option<TextCharacterPropertiesType>,
    /// _
    #[xml(child = "cs:bodyPr")]
    pub text_body_properties: Option<TextBodyProperties>,
    /// _
    #[xml(child = "cs:extLst")]
    pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the DropLine Class.
/// When the object is serialized out as xml, it's qualified name is cs:dropLine.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cs:dropLine")]
pub struct DropLine {
    /// mods
    /// Represents the following attribute in the schema: :mods
    #[xml(attr = "mods")]
    pub modifiers: Option<String>,
    /// _
    #[xml(child = "cs:lnRef")]
    pub line_reference: LineReference,
    /// _
    #[xml(child = "cs:lineWidthScale")]
    pub line_width_scale: Option<LineWidthScale>,
    /// _
    #[xml(child = "cs:fillRef")]
    pub fill_reference: FillReference,
    /// _
    #[xml(child = "cs:effectRef")]
    pub effect_reference: EffectReference,
    /// _
    #[xml(child = "cs:fontRef")]
    pub font_reference: FontReference,
    /// _
    #[xml(child = "cs:spPr")]
    pub shape_properties: Option<ShapeProperties>,
    /// _
    #[xml(child = "cs:defRPr")]
    pub text_character_properties_type: Option<TextCharacterPropertiesType>,
    /// _
    #[xml(child = "cs:bodyPr")]
    pub text_body_properties: Option<TextBodyProperties>,
    /// _
    #[xml(child = "cs:extLst")]
    pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the ErrorBar Class.
/// When the object is serialized out as xml, it's qualified name is cs:errorBar.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cs:errorBar")]
pub struct ErrorBar {
    /// mods
    /// Represents the following attribute in the schema: :mods
    #[xml(attr = "mods")]
    pub modifiers: Option<String>,
    /// _
    #[xml(child = "cs:lnRef")]
    pub line_reference: LineReference,
    /// _
    #[xml(child = "cs:lineWidthScale")]
    pub line_width_scale: Option<LineWidthScale>,
    /// _
    #[xml(child = "cs:fillRef")]
    pub fill_reference: FillReference,
    /// _
    #[xml(child = "cs:effectRef")]
    pub effect_reference: EffectReference,
    /// _
    #[xml(child = "cs:fontRef")]
    pub font_reference: FontReference,
    /// _
    #[xml(child = "cs:spPr")]
    pub shape_properties: Option<ShapeProperties>,
    /// _
    #[xml(child = "cs:defRPr")]
    pub text_character_properties_type: Option<TextCharacterPropertiesType>,
    /// _
    #[xml(child = "cs:bodyPr")]
    pub text_body_properties: Option<TextBodyProperties>,
    /// _
    #[xml(child = "cs:extLst")]
    pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the Floor Class.
/// When the object is serialized out as xml, it's qualified name is cs:floor.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cs:floor")]
pub struct Floor {
    /// mods
    /// Represents the following attribute in the schema: :mods
    #[xml(attr = "mods")]
    pub modifiers: Option<String>,
    /// _
    #[xml(child = "cs:lnRef")]
    pub line_reference: LineReference,
    /// _
    #[xml(child = "cs:lineWidthScale")]
    pub line_width_scale: Option<LineWidthScale>,
    /// _
    #[xml(child = "cs:fillRef")]
    pub fill_reference: FillReference,
    /// _
    #[xml(child = "cs:effectRef")]
    pub effect_reference: EffectReference,
    /// _
    #[xml(child = "cs:fontRef")]
    pub font_reference: FontReference,
    /// _
    #[xml(child = "cs:spPr")]
    pub shape_properties: Option<ShapeProperties>,
    /// _
    #[xml(child = "cs:defRPr")]
    pub text_character_properties_type: Option<TextCharacterPropertiesType>,
    /// _
    #[xml(child = "cs:bodyPr")]
    pub text_body_properties: Option<TextBodyProperties>,
    /// _
    #[xml(child = "cs:extLst")]
    pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the GridlineMajor Class.
/// When the object is serialized out as xml, it's qualified name is cs:gridlineMajor.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cs:gridlineMajor")]
pub struct GridlineMajor {
    /// mods
    /// Represents the following attribute in the schema: :mods
    #[xml(attr = "mods")]
    pub modifiers: Option<String>,
    /// _
    #[xml(child = "cs:lnRef")]
    pub line_reference: LineReference,
    /// _
    #[xml(child = "cs:lineWidthScale")]
    pub line_width_scale: Option<LineWidthScale>,
    /// _
    #[xml(child = "cs:fillRef")]
    pub fill_reference: FillReference,
    /// _
    #[xml(child = "cs:effectRef")]
    pub effect_reference: EffectReference,
    /// _
    #[xml(child = "cs:fontRef")]
    pub font_reference: FontReference,
    /// _
    #[xml(child = "cs:spPr")]
    pub shape_properties: Option<ShapeProperties>,
    /// _
    #[xml(child = "cs:defRPr")]
    pub text_character_properties_type: Option<TextCharacterPropertiesType>,
    /// _
    #[xml(child = "cs:bodyPr")]
    pub text_body_properties: Option<TextBodyProperties>,
    /// _
    #[xml(child = "cs:extLst")]
    pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the GridlineMinor Class.
/// When the object is serialized out as xml, it's qualified name is cs:gridlineMinor.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cs:gridlineMinor")]
pub struct GridlineMinor {
    /// mods
    /// Represents the following attribute in the schema: :mods
    #[xml(attr = "mods")]
    pub modifiers: Option<String>,
    /// _
    #[xml(child = "cs:lnRef")]
    pub line_reference: LineReference,
    /// _
    #[xml(child = "cs:lineWidthScale")]
    pub line_width_scale: Option<LineWidthScale>,
    /// _
    #[xml(child = "cs:fillRef")]
    pub fill_reference: FillReference,
    /// _
    #[xml(child = "cs:effectRef")]
    pub effect_reference: EffectReference,
    /// _
    #[xml(child = "cs:fontRef")]
    pub font_reference: FontReference,
    /// _
    #[xml(child = "cs:spPr")]
    pub shape_properties: Option<ShapeProperties>,
    /// _
    #[xml(child = "cs:defRPr")]
    pub text_character_properties_type: Option<TextCharacterPropertiesType>,
    /// _
    #[xml(child = "cs:bodyPr")]
    pub text_body_properties: Option<TextBodyProperties>,
    /// _
    #[xml(child = "cs:extLst")]
    pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the HiLoLine Class.
/// When the object is serialized out as xml, it's qualified name is cs:hiLoLine.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cs:hiLoLine")]
pub struct HiLoLine {
    /// mods
    /// Represents the following attribute in the schema: :mods
    #[xml(attr = "mods")]
    pub modifiers: Option<String>,
    /// _
    #[xml(child = "cs:lnRef")]
    pub line_reference: LineReference,
    /// _
    #[xml(child = "cs:lineWidthScale")]
    pub line_width_scale: Option<LineWidthScale>,
    /// _
    #[xml(child = "cs:fillRef")]
    pub fill_reference: FillReference,
    /// _
    #[xml(child = "cs:effectRef")]
    pub effect_reference: EffectReference,
    /// _
    #[xml(child = "cs:fontRef")]
    pub font_reference: FontReference,
    /// _
    #[xml(child = "cs:spPr")]
    pub shape_properties: Option<ShapeProperties>,
    /// _
    #[xml(child = "cs:defRPr")]
    pub text_character_properties_type: Option<TextCharacterPropertiesType>,
    /// _
    #[xml(child = "cs:bodyPr")]
    pub text_body_properties: Option<TextBodyProperties>,
    /// _
    #[xml(child = "cs:extLst")]
    pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the LeaderLine Class.
/// When the object is serialized out as xml, it's qualified name is cs:leaderLine.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cs:leaderLine")]
pub struct LeaderLine {
    /// mods
    /// Represents the following attribute in the schema: :mods
    #[xml(attr = "mods")]
    pub modifiers: Option<String>,
    /// _
    #[xml(child = "cs:lnRef")]
    pub line_reference: LineReference,
    /// _
    #[xml(child = "cs:lineWidthScale")]
    pub line_width_scale: Option<LineWidthScale>,
    /// _
    #[xml(child = "cs:fillRef")]
    pub fill_reference: FillReference,
    /// _
    #[xml(child = "cs:effectRef")]
    pub effect_reference: EffectReference,
    /// _
    #[xml(child = "cs:fontRef")]
    pub font_reference: FontReference,
    /// _
    #[xml(child = "cs:spPr")]
    pub shape_properties: Option<ShapeProperties>,
    /// _
    #[xml(child = "cs:defRPr")]
    pub text_character_properties_type: Option<TextCharacterPropertiesType>,
    /// _
    #[xml(child = "cs:bodyPr")]
    pub text_body_properties: Option<TextBodyProperties>,
    /// _
    #[xml(child = "cs:extLst")]
    pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the LegendStyle Class.
/// When the object is serialized out as xml, it's qualified name is cs:legend.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cs:legend")]
pub struct LegendStyle {
    /// mods
    /// Represents the following attribute in the schema: :mods
    #[xml(attr = "mods")]
    pub modifiers: Option<String>,
    /// _
    #[xml(child = "cs:lnRef")]
    pub line_reference: LineReference,
    /// _
    #[xml(child = "cs:lineWidthScale")]
    pub line_width_scale: Option<LineWidthScale>,
    /// _
    #[xml(child = "cs:fillRef")]
    pub fill_reference: FillReference,
    /// _
    #[xml(child = "cs:effectRef")]
    pub effect_reference: EffectReference,
    /// _
    #[xml(child = "cs:fontRef")]
    pub font_reference: FontReference,
    /// _
    #[xml(child = "cs:spPr")]
    pub shape_properties: Option<ShapeProperties>,
    /// _
    #[xml(child = "cs:defRPr")]
    pub text_character_properties_type: Option<TextCharacterPropertiesType>,
    /// _
    #[xml(child = "cs:bodyPr")]
    pub text_body_properties: Option<TextBodyProperties>,
    /// _
    #[xml(child = "cs:extLst")]
    pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the PlotArea Class.
/// When the object is serialized out as xml, it's qualified name is cs:plotArea.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cs:plotArea")]
pub struct PlotArea {
    /// mods
    /// Represents the following attribute in the schema: :mods
    #[xml(attr = "mods")]
    pub modifiers: Option<String>,
    /// _
    #[xml(child = "cs:lnRef")]
    pub line_reference: LineReference,
    /// _
    #[xml(child = "cs:lineWidthScale")]
    pub line_width_scale: Option<LineWidthScale>,
    /// _
    #[xml(child = "cs:fillRef")]
    pub fill_reference: FillReference,
    /// _
    #[xml(child = "cs:effectRef")]
    pub effect_reference: EffectReference,
    /// _
    #[xml(child = "cs:fontRef")]
    pub font_reference: FontReference,
    /// _
    #[xml(child = "cs:spPr")]
    pub shape_properties: Option<ShapeProperties>,
    /// _
    #[xml(child = "cs:defRPr")]
    pub text_character_properties_type: Option<TextCharacterPropertiesType>,
    /// _
    #[xml(child = "cs:bodyPr")]
    pub text_body_properties: Option<TextBodyProperties>,
    /// _
    #[xml(child = "cs:extLst")]
    pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the PlotArea3D Class.
/// When the object is serialized out as xml, it's qualified name is cs:plotArea3D.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cs:plotArea3D")]
pub struct PlotArea3D {
    /// mods
    /// Represents the following attribute in the schema: :mods
    #[xml(attr = "mods")]
    pub modifiers: Option<String>,
    /// _
    #[xml(child = "cs:lnRef")]
    pub line_reference: LineReference,
    /// _
    #[xml(child = "cs:lineWidthScale")]
    pub line_width_scale: Option<LineWidthScale>,
    /// _
    #[xml(child = "cs:fillRef")]
    pub fill_reference: FillReference,
    /// _
    #[xml(child = "cs:effectRef")]
    pub effect_reference: EffectReference,
    /// _
    #[xml(child = "cs:fontRef")]
    pub font_reference: FontReference,
    /// _
    #[xml(child = "cs:spPr")]
    pub shape_properties: Option<ShapeProperties>,
    /// _
    #[xml(child = "cs:defRPr")]
    pub text_character_properties_type: Option<TextCharacterPropertiesType>,
    /// _
    #[xml(child = "cs:bodyPr")]
    pub text_body_properties: Option<TextBodyProperties>,
    /// _
    #[xml(child = "cs:extLst")]
    pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the SeriesAxis Class.
/// When the object is serialized out as xml, it's qualified name is cs:seriesAxis.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cs:seriesAxis")]
pub struct SeriesAxis {
    /// mods
    /// Represents the following attribute in the schema: :mods
    #[xml(attr = "mods")]
    pub modifiers: Option<String>,
    /// _
    #[xml(child = "cs:lnRef")]
    pub line_reference: LineReference,
    /// _
    #[xml(child = "cs:lineWidthScale")]
    pub line_width_scale: Option<LineWidthScale>,
    /// _
    #[xml(child = "cs:fillRef")]
    pub fill_reference: FillReference,
    /// _
    #[xml(child = "cs:effectRef")]
    pub effect_reference: EffectReference,
    /// _
    #[xml(child = "cs:fontRef")]
    pub font_reference: FontReference,
    /// _
    #[xml(child = "cs:spPr")]
    pub shape_properties: Option<ShapeProperties>,
    /// _
    #[xml(child = "cs:defRPr")]
    pub text_character_properties_type: Option<TextCharacterPropertiesType>,
    /// _
    #[xml(child = "cs:bodyPr")]
    pub text_body_properties: Option<TextBodyProperties>,
    /// _
    #[xml(child = "cs:extLst")]
    pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the SeriesLine Class.
/// When the object is serialized out as xml, it's qualified name is cs:seriesLine.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cs:seriesLine")]
pub struct SeriesLine {
    /// mods
    /// Represents the following attribute in the schema: :mods
    #[xml(attr = "mods")]
    pub modifiers: Option<String>,
    /// _
    #[xml(child = "cs:lnRef")]
    pub line_reference: LineReference,
    /// _
    #[xml(child = "cs:lineWidthScale")]
    pub line_width_scale: Option<LineWidthScale>,
    /// _
    #[xml(child = "cs:fillRef")]
    pub fill_reference: FillReference,
    /// _
    #[xml(child = "cs:effectRef")]
    pub effect_reference: EffectReference,
    /// _
    #[xml(child = "cs:fontRef")]
    pub font_reference: FontReference,
    /// _
    #[xml(child = "cs:spPr")]
    pub shape_properties: Option<ShapeProperties>,
    /// _
    #[xml(child = "cs:defRPr")]
    pub text_character_properties_type: Option<TextCharacterPropertiesType>,
    /// _
    #[xml(child = "cs:bodyPr")]
    pub text_body_properties: Option<TextBodyProperties>,
    /// _
    #[xml(child = "cs:extLst")]
    pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the TitleStyle Class.
/// When the object is serialized out as xml, it's qualified name is cs:title.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cs:title")]
pub struct TitleStyle {
    /// mods
    /// Represents the following attribute in the schema: :mods
    #[xml(attr = "mods")]
    pub modifiers: Option<String>,
    /// _
    #[xml(child = "cs:lnRef")]
    pub line_reference: LineReference,
    /// _
    #[xml(child = "cs:lineWidthScale")]
    pub line_width_scale: Option<LineWidthScale>,
    /// _
    #[xml(child = "cs:fillRef")]
    pub fill_reference: FillReference,
    /// _
    #[xml(child = "cs:effectRef")]
    pub effect_reference: EffectReference,
    /// _
    #[xml(child = "cs:fontRef")]
    pub font_reference: FontReference,
    /// _
    #[xml(child = "cs:spPr")]
    pub shape_properties: Option<ShapeProperties>,
    /// _
    #[xml(child = "cs:defRPr")]
    pub text_character_properties_type: Option<TextCharacterPropertiesType>,
    /// _
    #[xml(child = "cs:bodyPr")]
    pub text_body_properties: Option<TextBodyProperties>,
    /// _
    #[xml(child = "cs:extLst")]
    pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the TrendlineStyle Class.
/// When the object is serialized out as xml, it's qualified name is cs:trendline.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cs:trendline")]
pub struct TrendlineStyle {
    /// mods
    /// Represents the following attribute in the schema: :mods
    #[xml(attr = "mods")]
    pub modifiers: Option<String>,
    /// _
    #[xml(child = "cs:lnRef")]
    pub line_reference: LineReference,
    /// _
    #[xml(child = "cs:lineWidthScale")]
    pub line_width_scale: Option<LineWidthScale>,
    /// _
    #[xml(child = "cs:fillRef")]
    pub fill_reference: FillReference,
    /// _
    #[xml(child = "cs:effectRef")]
    pub effect_reference: EffectReference,
    /// _
    #[xml(child = "cs:fontRef")]
    pub font_reference: FontReference,
    /// _
    #[xml(child = "cs:spPr")]
    pub shape_properties: Option<ShapeProperties>,
    /// _
    #[xml(child = "cs:defRPr")]
    pub text_character_properties_type: Option<TextCharacterPropertiesType>,
    /// _
    #[xml(child = "cs:bodyPr")]
    pub text_body_properties: Option<TextBodyProperties>,
    /// _
    #[xml(child = "cs:extLst")]
    pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the TrendlineLabel Class.
/// When the object is serialized out as xml, it's qualified name is cs:trendlineLabel.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cs:trendlineLabel")]
pub struct TrendlineLabel {
    /// mods
    /// Represents the following attribute in the schema: :mods
    #[xml(attr = "mods")]
    pub modifiers: Option<String>,
    /// _
    #[xml(child = "cs:lnRef")]
    pub line_reference: LineReference,
    /// _
    #[xml(child = "cs:lineWidthScale")]
    pub line_width_scale: Option<LineWidthScale>,
    /// _
    #[xml(child = "cs:fillRef")]
    pub fill_reference: FillReference,
    /// _
    #[xml(child = "cs:effectRef")]
    pub effect_reference: EffectReference,
    /// _
    #[xml(child = "cs:fontRef")]
    pub font_reference: FontReference,
    /// _
    #[xml(child = "cs:spPr")]
    pub shape_properties: Option<ShapeProperties>,
    /// _
    #[xml(child = "cs:defRPr")]
    pub text_character_properties_type: Option<TextCharacterPropertiesType>,
    /// _
    #[xml(child = "cs:bodyPr")]
    pub text_body_properties: Option<TextBodyProperties>,
    /// _
    #[xml(child = "cs:extLst")]
    pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the UpBar Class.
/// When the object is serialized out as xml, it's qualified name is cs:upBar.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cs:upBar")]
pub struct UpBar {
    /// mods
    /// Represents the following attribute in the schema: :mods
    #[xml(attr = "mods")]
    pub modifiers: Option<String>,
    /// _
    #[xml(child = "cs:lnRef")]
    pub line_reference: LineReference,
    /// _
    #[xml(child = "cs:lineWidthScale")]
    pub line_width_scale: Option<LineWidthScale>,
    /// _
    #[xml(child = "cs:fillRef")]
    pub fill_reference: FillReference,
    /// _
    #[xml(child = "cs:effectRef")]
    pub effect_reference: EffectReference,
    /// _
    #[xml(child = "cs:fontRef")]
    pub font_reference: FontReference,
    /// _
    #[xml(child = "cs:spPr")]
    pub shape_properties: Option<ShapeProperties>,
    /// _
    #[xml(child = "cs:defRPr")]
    pub text_character_properties_type: Option<TextCharacterPropertiesType>,
    /// _
    #[xml(child = "cs:bodyPr")]
    pub text_body_properties: Option<TextBodyProperties>,
    /// _
    #[xml(child = "cs:extLst")]
    pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the ValueAxis Class.
/// When the object is serialized out as xml, it's qualified name is cs:valueAxis.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cs:valueAxis")]
pub struct ValueAxis {
    /// mods
    /// Represents the following attribute in the schema: :mods
    #[xml(attr = "mods")]
    pub modifiers: Option<String>,
    /// _
    #[xml(child = "cs:lnRef")]
    pub line_reference: LineReference,
    /// _
    #[xml(child = "cs:lineWidthScale")]
    pub line_width_scale: Option<LineWidthScale>,
    /// _
    #[xml(child = "cs:fillRef")]
    pub fill_reference: FillReference,
    /// _
    #[xml(child = "cs:effectRef")]
    pub effect_reference: EffectReference,
    /// _
    #[xml(child = "cs:fontRef")]
    pub font_reference: FontReference,
    /// _
    #[xml(child = "cs:spPr")]
    pub shape_properties: Option<ShapeProperties>,
    /// _
    #[xml(child = "cs:defRPr")]
    pub text_character_properties_type: Option<TextCharacterPropertiesType>,
    /// _
    #[xml(child = "cs:bodyPr")]
    pub text_body_properties: Option<TextBodyProperties>,
    /// _
    #[xml(child = "cs:extLst")]
    pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the Wall Class.
/// When the object is serialized out as xml, it's qualified name is cs:wall.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cs:wall")]
pub struct Wall {
    /// mods
    /// Represents the following attribute in the schema: :mods
    #[xml(attr = "mods")]
    pub modifiers: Option<String>,
    /// _
    #[xml(child = "cs:lnRef")]
    pub line_reference: LineReference,
    /// _
    #[xml(child = "cs:lineWidthScale")]
    pub line_width_scale: Option<LineWidthScale>,
    /// _
    #[xml(child = "cs:fillRef")]
    pub fill_reference: FillReference,
    /// _
    #[xml(child = "cs:effectRef")]
    pub effect_reference: EffectReference,
    /// _
    #[xml(child = "cs:fontRef")]
    pub font_reference: FontReference,
    /// _
    #[xml(child = "cs:spPr")]
    pub shape_properties: Option<ShapeProperties>,
    /// _
    #[xml(child = "cs:defRPr")]
    pub text_character_properties_type: Option<TextCharacterPropertiesType>,
    /// _
    #[xml(child = "cs:bodyPr")]
    pub text_body_properties: Option<TextBodyProperties>,
    /// _
    #[xml(child = "cs:extLst")]
    pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the StyleEntry Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct StyleEntry {
    /// mods
    /// Represents the following attribute in the schema: :mods
    #[xml(attr = "mods")]
    pub modifiers: Option<String>,
}
/// Defines the MarkerLayoutProperties Class.
/// When the object is serialized out as xml, it's qualified name is cs:dataPointMarkerLayout.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "cs:dataPointMarkerLayout")]
pub struct MarkerLayoutProperties {
    /// symbol
    /// Represents the following attribute in the schema: :symbol
    #[xml(attr = "symbol")]
    pub symbol: Option<MarkerStyle>,
    /// size
    /// Represents the following attribute in the schema: :size
    #[xml(attr = "size")]
    pub size: Option<u8>,
}
