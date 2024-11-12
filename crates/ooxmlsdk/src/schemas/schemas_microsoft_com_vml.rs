#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum ExtensionHandlingBehaviorValues {
    #[default]
    View,
    Edit,
    BackwardCompatible,
}
crate::__string_enum! {
    ExtensionHandlingBehaviorValues { View = "view", Edit = "edit", BackwardCompatible =
    "backwardCompatible", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum FillTypeValues {
    #[default]
    Solid,
    Gradient,
    GradientRadial,
    Tile,
    Pattern,
    Frame,
}
crate::__string_enum! {
    FillTypeValues { Solid = "solid", Gradient = "gradient", GradientRadial =
    "gradientRadial", Tile = "tile", Pattern = "pattern", Frame = "frame", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum FillMethodValues {
    #[default]
    None,
    Linear,
    Sigma,
    Any,
    Linearsigma,
}
crate::__string_enum! {
    FillMethodValues { None = "none", Linear = "linear", Sigma = "sigma", Any = "any",
    Linearsigma = "linearSigma", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum StrokeLineStyleValues {
    #[default]
    Single,
    ThinThin,
    ThinThick,
    ThickThin,
    ThickBetweenThin,
}
crate::__string_enum! {
    StrokeLineStyleValues { Single = "single", ThinThin = "thinThin", ThinThick =
    "thinThick", ThickThin = "thickThin", ThickBetweenThin = "thickBetweenThin", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum StrokeJoinStyleValues {
    #[default]
    Round,
    Bevel,
    Miter,
}
crate::__string_enum! {
    StrokeJoinStyleValues { Round = "round", Bevel = "bevel", Miter = "miter", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum StrokeEndCapValues {
    #[default]
    Flat,
    Square,
    Round,
}
crate::__string_enum! {
    StrokeEndCapValues { Flat = "flat", Square = "square", Round = "round", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum StrokeArrowLengthValues {
    #[default]
    Short,
    Medium,
    Long,
}
crate::__string_enum! {
    StrokeArrowLengthValues { Short = "short", Medium = "medium", Long = "long", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum StrokeArrowWidthValues {
    #[default]
    Narrow,
    Medium,
    Wide,
}
crate::__string_enum! {
    StrokeArrowWidthValues { Narrow = "narrow", Medium = "medium", Wide = "wide", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum StrokeArrowValues {
    #[default]
    None,
    Block,
    Classic,
    Oval,
    Diamond,
    Open,
}
crate::__string_enum! {
    StrokeArrowValues { None = "none", Block = "block", Classic = "classic", Oval =
    "oval", Diamond = "diamond", Open = "open", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum ImageAspectValues {
    #[default]
    Ignore,
    AtMost,
    AtLeast,
}
crate::__string_enum! {
    ImageAspectValues { Ignore = "ignore", AtMost = "atMost", AtLeast = "atLeast", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum EditAsValues {
    #[default]
    Canvas,
    OrganizationChart,
    Radial,
    Cycle,
    Stacked,
    Venn,
    Bullseye,
}
crate::__string_enum! {
    EditAsValues { Canvas = "canvas", OrganizationChart = "orgchart", Radial = "radial",
    Cycle = "cycle", Stacked = "stacked", Venn = "venn", Bullseye = "bullseye", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum ShadowValues {
    #[default]
    Single,
    Double,
    Emboss,
    Perspective,
    ShapeRelative,
    DrawingRelative,
}
crate::__string_enum! {
    ShadowValues { Single = "single", Double = "double", Emboss = "emboss", Perspective =
    "perspective", ShapeRelative = "shapeRelative", DrawingRelative = "drawingRelative",
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum StrokeFillTypeValues {
    #[default]
    Solid,
    Tile,
    Pattern,
    Frame,
}
crate::__string_enum! {
    StrokeFillTypeValues { Solid = "solid", Tile = "tile", Pattern = "pattern", Frame =
    "frame", }
}
/// Defines the Path Class.
/// When the object is serialized out as xml, it's qualified name is v:path.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "v:path")]
pub struct Path {
    /// Unique Identifier
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: Option<String>,
    /// Path Definition
    /// Represents the following attribute in the schema: :v
    #[xml(attr = "v")]
    pub value: Option<String>,
    /// Limo Stretch Point
    /// Represents the following attribute in the schema: :limo
    #[xml(attr = "limo")]
    pub limo: Option<String>,
    /// Text Box Bounding Box
    /// Represents the following attribute in the schema: :textboxrect
    #[xml(attr = "textboxrect")]
    pub textbox_rectangle: Option<String>,
    /// Shape Fill Toggle
    /// Represents the following attribute in the schema: :fillok
    #[xml(attr = "fillok")]
    pub allow_fill: Option<bool>,
    /// Stroke Toggle
    /// Represents the following attribute in the schema: :strokeok
    #[xml(attr = "strokeok")]
    pub allow_stroke: Option<bool>,
    /// Shadow Toggle
    /// Represents the following attribute in the schema: :shadowok
    #[xml(attr = "shadowok")]
    pub allow_shading: Option<bool>,
    /// Arrowhead Display Toggle
    /// Represents the following attribute in the schema: :arrowok
    #[xml(attr = "arrowok")]
    pub show_arrowhead: Option<bool>,
    /// Gradient Shape Toggle
    /// Represents the following attribute in the schema: :gradientshapeok
    #[xml(attr = "gradientshapeok")]
    pub allow_gradient_shape: Option<bool>,
    /// Text Path Toggle
    /// Represents the following attribute in the schema: :textpathok
    #[xml(attr = "textpathok")]
    pub allow_text_path: Option<bool>,
    /// Inset Stroke From Path Flag
    /// Represents the following attribute in the schema: :insetpenok
    #[xml(attr = "insetpenok")]
    pub allow_inset_pen: Option<bool>,
    /// Connection Point Type
    /// Represents the following attribute in the schema: o:connecttype
    #[xml(attr = "o:connecttype")]
    pub connection_point_type: Option<
        crate::schemas::schemas_microsoft_com_office_office::ConnectValues,
    >,
    /// Connection Points
    /// Represents the following attribute in the schema: o:connectlocs
    #[xml(attr = "o:connectlocs")]
    pub connection_points: Option<String>,
    /// Connection Point Connect Angles
    /// Represents the following attribute in the schema: o:connectangles
    #[xml(attr = "o:connectangles")]
    pub connect_angles: Option<String>,
    /// Extrusion Toggle
    /// Represents the following attribute in the schema: o:extrusionok
    #[xml(attr = "o:extrusionok")]
    pub allow_extrusion: Option<bool>,
}
/// Defines the Formulas Class.
/// When the object is serialized out as xml, it's qualified name is v:formulas.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "v:formulas")]
pub struct Formulas {
    /// _
    #[xml(child = "v:f")]
    pub v_f: Vec<Formula>,
}
/// Defines the ShapeHandles Class.
/// When the object is serialized out as xml, it's qualified name is v:handles.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "v:handles")]
pub struct ShapeHandles {
    /// _
    #[xml(child = "v:h")]
    pub v_h: Vec<ShapeHandle>,
}
/// Defines the Fill Class.
/// When the object is serialized out as xml, it's qualified name is v:fill.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "v:fill")]
pub struct Fill {
    /// Unique Identifier
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: Option<String>,
    /// Fill Type
    /// Represents the following attribute in the schema: :type
    #[xml(attr = "type")]
    pub r#type: Option<FillTypeValues>,
    /// Fill Toggle
    /// Represents the following attribute in the schema: :on
    #[xml(attr = "on")]
    pub on: Option<bool>,
    /// Primary Color
    /// Represents the following attribute in the schema: :color
    #[xml(attr = "color")]
    pub color: Option<String>,
    /// Primary Color Opacity
    /// Represents the following attribute in the schema: :opacity
    #[xml(attr = "opacity")]
    pub opacity: Option<String>,
    /// Secondary Color
    /// Represents the following attribute in the schema: :color2
    #[xml(attr = "color2")]
    pub color2: Option<String>,
    /// Fill Image Source
    /// Represents the following attribute in the schema: :src
    #[xml(attr = "src")]
    pub source: Option<String>,
    /// Hyperlink Target
    /// Represents the following attribute in the schema: o:href
    #[xml(attr = "o:href")]
    pub href: Option<String>,
    /// Alternate Image Reference Location
    /// Represents the following attribute in the schema: o:althref
    #[xml(attr = "o:althref")]
    pub alternate_image_reference: Option<String>,
    /// Fill Image Size
    /// Represents the following attribute in the schema: :size
    #[xml(attr = "size")]
    pub size: Option<String>,
    /// Fill Image Origin
    /// Represents the following attribute in the schema: :origin
    #[xml(attr = "origin")]
    pub origin: Option<String>,
    /// Fill Image Position
    /// Represents the following attribute in the schema: :position
    #[xml(attr = "position")]
    pub position: Option<String>,
    /// Image Aspect Ratio
    /// Represents the following attribute in the schema: :aspect
    #[xml(attr = "aspect")]
    pub aspect: Option<ImageAspectValues>,
    /// Intermediate Colors
    /// Represents the following attribute in the schema: :colors
    #[xml(attr = "colors")]
    pub colors: Option<String>,
    /// Gradient Angle
    /// Represents the following attribute in the schema: :angle
    #[xml(attr = "angle")]
    pub angle: Option<String>,
    /// Align Image With Shape
    /// Represents the following attribute in the schema: :alignshape
    #[xml(attr = "alignshape")]
    pub align_shape: Option<bool>,
    /// Gradient Center
    /// Represents the following attribute in the schema: :focus
    #[xml(attr = "focus")]
    pub focus: Option<String>,
    /// Radial Gradient Size
    /// Represents the following attribute in the schema: :focussize
    #[xml(attr = "focussize")]
    pub focus_size: Option<String>,
    /// Radial Gradient Center
    /// Represents the following attribute in the schema: :focusposition
    #[xml(attr = "focusposition")]
    pub focus_position: Option<String>,
    /// Gradient Fill Method
    /// Represents the following attribute in the schema: :method
    #[xml(attr = "method")]
    pub method: Option<FillMethodValues>,
    /// Detect Mouse Click
    /// Represents the following attribute in the schema: o:detectmouseclick
    #[xml(attr = "o:detectmouseclick")]
    pub detect_mouse_click: Option<bool>,
    /// Title
    /// Represents the following attribute in the schema: o:title
    #[xml(attr = "o:title")]
    pub title: Option<String>,
    /// Secondary Color Opacity
    /// Represents the following attribute in the schema: o:opacity2
    #[xml(attr = "o:opacity2")]
    pub opacity2: Option<String>,
    /// Recolor Fill as Picture
    /// Represents the following attribute in the schema: :recolor
    #[xml(attr = "recolor")]
    pub recolor: Option<bool>,
    /// Rotate Fill with Shape
    /// Represents the following attribute in the schema: :rotate
    #[xml(attr = "rotate")]
    pub rotate: Option<bool>,
    /// Relationship to Part
    /// Represents the following attribute in the schema: r:id
    #[xml(attr = "r:id")]
    pub relationship_id: Option<String>,
    /// _
    #[xml(child = "o:fill")]
    pub fill_extended_properties: Option<
        crate::schemas::schemas_microsoft_com_office_office::FillExtendedProperties,
    >,
}
/// Defines the Stroke Class.
/// When the object is serialized out as xml, it's qualified name is v:stroke.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "v:stroke")]
pub struct Stroke {
    /// Unique Identifier
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: Option<String>,
    /// Stroke Toggle
    /// Represents the following attribute in the schema: :on
    #[xml(attr = "on")]
    pub on: Option<bool>,
    /// Stroke Weight
    /// Represents the following attribute in the schema: :weight
    #[xml(attr = "weight")]
    pub weight: Option<String>,
    /// Stroke Color
    /// Represents the following attribute in the schema: :color
    #[xml(attr = "color")]
    pub color: Option<String>,
    /// Stroke Opacity
    /// Represents the following attribute in the schema: :opacity
    #[xml(attr = "opacity")]
    pub opacity: Option<String>,
    /// Stroke Line Style
    /// Represents the following attribute in the schema: :linestyle
    #[xml(attr = "linestyle")]
    pub line_style: Option<StrokeLineStyleValues>,
    /// Miter Joint Limit
    /// Represents the following attribute in the schema: :miterlimit
    #[xml(attr = "miterlimit")]
    pub miterlimit: Option<String>,
    /// Line End Join Style
    /// Represents the following attribute in the schema: :joinstyle
    #[xml(attr = "joinstyle")]
    pub join_style: Option<StrokeJoinStyleValues>,
    /// Line End Cap
    /// Represents the following attribute in the schema: :endcap
    #[xml(attr = "endcap")]
    pub end_cap: Option<StrokeEndCapValues>,
    /// Stroke Dash Pattern
    /// Represents the following attribute in the schema: :dashstyle
    #[xml(attr = "dashstyle")]
    pub dash_style: Option<String>,
    /// Stroke Image Style
    /// Represents the following attribute in the schema: :filltype
    #[xml(attr = "filltype")]
    pub fill_type: Option<StrokeFillTypeValues>,
    /// Stroke Image Location
    /// Represents the following attribute in the schema: :src
    #[xml(attr = "src")]
    pub source: Option<String>,
    /// Stroke Image Aspect Ratio
    /// Represents the following attribute in the schema: :imageaspect
    #[xml(attr = "imageaspect")]
    pub image_aspect: Option<ImageAspectValues>,
    /// Stroke Image Size
    /// Represents the following attribute in the schema: :imagesize
    #[xml(attr = "imagesize")]
    pub image_size: Option<String>,
    /// Stoke Image Alignment
    /// Represents the following attribute in the schema: :imagealignshape
    #[xml(attr = "imagealignshape")]
    pub image_align_shape: Option<bool>,
    /// Stroke Alternate Pattern Color
    /// Represents the following attribute in the schema: :color2
    #[xml(attr = "color2")]
    pub color2: Option<String>,
    /// Line Start Arrowhead
    /// Represents the following attribute in the schema: :startarrow
    #[xml(attr = "startarrow")]
    pub start_arrow: Option<StrokeArrowValues>,
    /// Line Start Arrowhead Width
    /// Represents the following attribute in the schema: :startarrowwidth
    #[xml(attr = "startarrowwidth")]
    pub start_arrow_width: Option<StrokeArrowWidthValues>,
    /// Line Start Arrowhead Length
    /// Represents the following attribute in the schema: :startarrowlength
    #[xml(attr = "startarrowlength")]
    pub start_arrow_length: Option<StrokeArrowLengthValues>,
    /// Line End Arrowhead
    /// Represents the following attribute in the schema: :endarrow
    #[xml(attr = "endarrow")]
    pub end_arrow: Option<StrokeArrowValues>,
    /// Line End Arrowhead Width
    /// Represents the following attribute in the schema: :endarrowwidth
    #[xml(attr = "endarrowwidth")]
    pub end_arrow_width: Option<StrokeArrowWidthValues>,
    /// Line End Arrowhead Length
    /// Represents the following attribute in the schema: :endarrowlength
    #[xml(attr = "endarrowlength")]
    pub end_arrow_length: Option<StrokeArrowLengthValues>,
    /// Original Image Reference
    /// Represents the following attribute in the schema: o:href
    #[xml(attr = "o:href")]
    pub href: Option<String>,
    /// Alternate Image Reference
    /// Represents the following attribute in the schema: o:althref
    #[xml(attr = "o:althref")]
    pub alternate_image_reference: Option<String>,
    /// Stroke Title
    /// Represents the following attribute in the schema: o:title
    #[xml(attr = "o:title")]
    pub title: Option<String>,
    /// Force Dashed Outline
    /// Represents the following attribute in the schema: o:forcedash
    #[xml(attr = "o:forcedash")]
    pub force_dash: Option<bool>,
    /// Relationship
    /// Represents the following attribute in the schema: r:id
    #[xml(attr = "r:id")]
    pub relationship_id: Option<String>,
    /// Inset Border From Path
    /// Represents the following attribute in the schema: :insetpen
    #[xml(attr = "insetpen")]
    pub insetpen: Option<bool>,
    /// _
    #[xml(child = "o:left")]
    pub left_stroke: Option<
        crate::schemas::schemas_microsoft_com_office_office::LeftStroke,
    >,
    /// _
    #[xml(child = "o:top")]
    pub top_stroke: Option<
        crate::schemas::schemas_microsoft_com_office_office::TopStroke,
    >,
    /// _
    #[xml(child = "o:right")]
    pub right_stroke: Option<
        crate::schemas::schemas_microsoft_com_office_office::RightStroke,
    >,
    /// _
    #[xml(child = "o:bottom")]
    pub bottom_stroke: Option<
        crate::schemas::schemas_microsoft_com_office_office::BottomStroke,
    >,
    /// _
    #[xml(child = "o:column")]
    pub column_stroke: Option<
        crate::schemas::schemas_microsoft_com_office_office::ColumnStroke,
    >,
}
/// Defines the Shadow Class.
/// When the object is serialized out as xml, it's qualified name is v:shadow.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "v:shadow")]
pub struct Shadow {
    /// Unique Identifier
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: Option<String>,
    /// Shadow Toggle
    /// Represents the following attribute in the schema: :on
    #[xml(attr = "on")]
    pub on: Option<bool>,
    /// Shadow Type
    /// Represents the following attribute in the schema: :type
    #[xml(attr = "type")]
    pub r#type: Option<ShadowValues>,
    /// Shadow Transparency
    /// Represents the following attribute in the schema: :obscured
    #[xml(attr = "obscured")]
    pub obscured: Option<bool>,
    /// Shadow Primary Color
    /// Represents the following attribute in the schema: :color
    #[xml(attr = "color")]
    pub color: Option<String>,
    /// Shadow Opacity
    /// Represents the following attribute in the schema: :opacity
    #[xml(attr = "opacity")]
    pub opacity: Option<String>,
    /// Shadow Primary Offset
    /// Represents the following attribute in the schema: :offset
    #[xml(attr = "offset")]
    pub offset: Option<String>,
    /// Shadow Secondary Color
    /// Represents the following attribute in the schema: :color2
    #[xml(attr = "color2")]
    pub color2: Option<String>,
    /// Shadow Secondary Offset
    /// Represents the following attribute in the schema: :offset2
    #[xml(attr = "offset2")]
    pub offset2: Option<String>,
    /// Shadow Origin
    /// Represents the following attribute in the schema: :origin
    #[xml(attr = "origin")]
    pub origin: Option<String>,
    /// Shadow Perspective Matrix
    /// Represents the following attribute in the schema: :matrix
    #[xml(attr = "matrix")]
    pub matrix: Option<String>,
}
/// Defines the TextBox Class.
/// When the object is serialized out as xml, it's qualified name is v:textbox.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "v:textbox")]
pub struct TextBox {
    /// Unique Identifier
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: Option<String>,
    /// Shape Styling Properties
    /// Represents the following attribute in the schema: :style
    #[xml(attr = "style")]
    pub style: Option<String>,
    /// Text Box Inset
    /// Represents the following attribute in the schema: :inset
    #[xml(attr = "inset")]
    pub inset: Option<String>,
    /// Text Box Single-Click Selection Toggle
    /// Represents the following attribute in the schema: o:singleclick
    #[xml(attr = "o:singleclick")]
    pub single_click: Option<bool>,
    #[xml(child = "w:txbxContent")]
    pub children: Vec<TextBoxChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum TextBoxChildChoice {
    #[xml(tag = "w:txbxContent")]
    WTxbxContent(
        crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::TextBoxContent,
    ),
}
/// Defines the TextPath Class.
/// When the object is serialized out as xml, it's qualified name is v:textpath.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "v:textpath")]
pub struct TextPath {
    /// Unique Identifier
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: Option<String>,
    /// Shape Styling Properties
    /// Represents the following attribute in the schema: :style
    #[xml(attr = "style")]
    pub style: Option<String>,
    /// Text Path Toggle
    /// Represents the following attribute in the schema: :on
    #[xml(attr = "on")]
    pub on: Option<bool>,
    /// Shape Fit Toggle
    /// Represents the following attribute in the schema: :fitshape
    #[xml(attr = "fitshape")]
    pub fit_shape: Option<bool>,
    /// Path Fit Toggle
    /// Represents the following attribute in the schema: :fitpath
    #[xml(attr = "fitpath")]
    pub fit_path: Option<bool>,
    /// Text Path Trim Toggle
    /// Represents the following attribute in the schema: :trim
    #[xml(attr = "trim")]
    pub trim: Option<bool>,
    /// Text X-Scaling
    /// Represents the following attribute in the schema: :xscale
    #[xml(attr = "xscale")]
    pub x_scale: Option<bool>,
    /// Text Path Text
    /// Represents the following attribute in the schema: :string
    #[xml(attr = "string")]
    pub string: Option<String>,
}
/// Defines the ImageData Class.
/// When the object is serialized out as xml, it's qualified name is v:imagedata.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "v:imagedata")]
pub struct ImageData {
    /// Unique Identifier
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: Option<String>,
    /// Image Transparency Color
    /// Represents the following attribute in the schema: :chromakey
    #[xml(attr = "chromakey")]
    pub chrom_a_key: Option<String>,
    /// Image Left Crop
    /// Represents the following attribute in the schema: :cropleft
    #[xml(attr = "cropleft")]
    pub crop_left: Option<String>,
    /// Image Top Crop
    /// Represents the following attribute in the schema: :croptop
    #[xml(attr = "croptop")]
    pub crop_top: Option<String>,
    /// Image Right Crop
    /// Represents the following attribute in the schema: :cropright
    #[xml(attr = "cropright")]
    pub crop_right: Option<String>,
    /// Image Bottom Crop
    /// Represents the following attribute in the schema: :cropbottom
    #[xml(attr = "cropbottom")]
    pub crop_bottom: Option<String>,
    /// Image Intensity
    /// Represents the following attribute in the schema: :gain
    #[xml(attr = "gain")]
    pub gain: Option<String>,
    /// Image Brightness
    /// Represents the following attribute in the schema: :blacklevel
    #[xml(attr = "blacklevel")]
    pub black_level: Option<String>,
    /// Image Gamma Correction
    /// Represents the following attribute in the schema: :gamma
    #[xml(attr = "gamma")]
    pub gamma: Option<String>,
    /// Image Grayscale Toggle
    /// Represents the following attribute in the schema: :grayscale
    #[xml(attr = "grayscale")]
    pub grayscale: Option<bool>,
    /// Image Bilevel Toggle
    /// Represents the following attribute in the schema: :bilevel
    #[xml(attr = "bilevel")]
    pub bi_level: Option<bool>,
    /// Embossed Color
    /// Represents the following attribute in the schema: :embosscolor
    #[xml(attr = "embosscolor")]
    pub emboss_color: Option<String>,
    /// Black Recoloring Color
    /// Represents the following attribute in the schema: :recolortarget
    #[xml(attr = "recolortarget")]
    pub recolor_target: Option<String>,
    /// Image Data Title
    /// Represents the following attribute in the schema: o:title
    #[xml(attr = "o:title")]
    pub title: Option<String>,
    /// Detect Mouse Click
    /// Represents the following attribute in the schema: o:detectmouseclick
    #[xml(attr = "o:detectmouseclick")]
    pub detect_mouse_click: Option<bool>,
    /// Relationship to Part
    /// Represents the following attribute in the schema: o:relid
    #[xml(attr = "o:relid")]
    pub rel_id: Option<String>,
    /// Explicit Relationship to Image Data
    /// Represents the following attribute in the schema: r:id
    #[xml(attr = "r:id")]
    pub relationship_id: Option<String>,
    /// Explicit Relationship to Alternate Image Data
    /// Represents the following attribute in the schema: r:pict
    #[xml(attr = "r:pict")]
    pub picture: Option<String>,
    /// Explicit Relationship to Hyperlink Target
    /// Represents the following attribute in the schema: r:href
    #[xml(attr = "r:href")]
    pub rel_href: Option<String>,
}
/// Shape Definition.
/// When the object is serialized out as xml, it's qualified name is v:shape.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "v:shape")]
pub struct Shape {
    /// Unique Identifier
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: Option<String>,
    /// Shape Styling Properties
    /// Represents the following attribute in the schema: :style
    #[xml(attr = "style")]
    pub style: Option<String>,
    /// Hyperlink Target
    /// Represents the following attribute in the schema: :href
    #[xml(attr = "href")]
    pub href: Option<String>,
    /// Hyperlink Display Target
    /// Represents the following attribute in the schema: :target
    #[xml(attr = "target")]
    pub target: Option<String>,
    /// CSS Reference
    /// Represents the following attribute in the schema: :class
    #[xml(attr = "class")]
    pub class: Option<String>,
    /// Shape Title
    /// Represents the following attribute in the schema: :title
    #[xml(attr = "title")]
    pub title: Option<String>,
    /// Alternate Text
    /// Represents the following attribute in the schema: :alt
    #[xml(attr = "alt")]
    pub alternate: Option<String>,
    /// Coordinate Space Size
    /// Represents the following attribute in the schema: :coordsize
    #[xml(attr = "coordsize")]
    pub coordinate_size: Option<String>,
    /// Coordinate Space Origin
    /// Represents the following attribute in the schema: :coordorigin
    #[xml(attr = "coordorigin")]
    pub coordinate_origin: Option<String>,
    /// Shape Bounding Polygon
    /// Represents the following attribute in the schema: :wrapcoords
    #[xml(attr = "wrapcoords")]
    pub wrap_coordinates: Option<String>,
    /// Print Toggle
    /// Represents the following attribute in the schema: :print
    #[xml(attr = "print")]
    pub print: Option<bool>,
    /// Optional String
    /// Represents the following attribute in the schema: o:spid
    #[xml(attr = "o:spid")]
    pub optional_string: Option<String>,
    /// Shape Handle Toggle
    /// Represents the following attribute in the schema: o:oned
    #[xml(attr = "o:oned")]
    pub oned: Option<bool>,
    /// Regroup ID
    /// Represents the following attribute in the schema: o:regroupid
    #[xml(attr = "o:regroupid")]
    pub regroup_id: Option<i32>,
    /// Double-click Notification Toggle
    /// Represents the following attribute in the schema: o:doubleclicknotify
    #[xml(attr = "o:doubleclicknotify")]
    pub double_click_notify: Option<bool>,
    /// Button Behavior Toggle
    /// Represents the following attribute in the schema: o:button
    #[xml(attr = "o:button")]
    pub button: Option<bool>,
    /// Hide Script Anchors
    /// Represents the following attribute in the schema: o:userhidden
    #[xml(attr = "o:userhidden")]
    pub user_hidden: Option<bool>,
    /// Graphical Bullet
    /// Represents the following attribute in the schema: o:bullet
    #[xml(attr = "o:bullet")]
    pub bullet: Option<bool>,
    /// Horizontal Rule Toggle
    /// Represents the following attribute in the schema: o:hr
    #[xml(attr = "o:hr")]
    pub horizontal: Option<bool>,
    /// Horizontal Rule Standard Display Toggle
    /// Represents the following attribute in the schema: o:hrstd
    #[xml(attr = "o:hrstd")]
    pub horizontal_standard: Option<bool>,
    /// Horizontal Rule 3D Shading Toggle
    /// Represents the following attribute in the schema: o:hrnoshade
    #[xml(attr = "o:hrnoshade")]
    pub horizontal_no_shade: Option<bool>,
    /// Horizontal Rule Length Percentage
    /// Represents the following attribute in the schema: o:hrpct
    #[xml(attr = "o:hrpct")]
    pub horizontal_percentage: Option<f32>,
    /// Horizontal Rule Alignment
    /// Represents the following attribute in the schema: o:hralign
    #[xml(attr = "o:hralign")]
    pub horizontal_alignment: Option<
        crate::schemas::schemas_microsoft_com_office_office::HorizontalRuleAlignmentValues,
    >,
    /// Allow in Table Cell
    /// Represents the following attribute in the schema: o:allowincell
    #[xml(attr = "o:allowincell")]
    pub allow_in_cell: Option<bool>,
    /// Allow Shape Overlap
    /// Represents the following attribute in the schema: o:allowoverlap
    #[xml(attr = "o:allowoverlap")]
    pub allow_overlap: Option<bool>,
    /// Exists In Master Slide
    /// Represents the following attribute in the schema: o:userdrawn
    #[xml(attr = "o:userdrawn")]
    pub user_drawn: Option<bool>,
    /// Border Top Color
    /// Represents the following attribute in the schema: o:bordertopcolor
    #[xml(attr = "o:bordertopcolor")]
    pub border_top_color: Option<String>,
    /// Border Left Color
    /// Represents the following attribute in the schema: o:borderleftcolor
    #[xml(attr = "o:borderleftcolor")]
    pub border_left_color: Option<String>,
    /// Bottom Border Color
    /// Represents the following attribute in the schema: o:borderbottomcolor
    #[xml(attr = "o:borderbottomcolor")]
    pub border_bottom_color: Option<String>,
    /// Border Right Color
    /// Represents the following attribute in the schema: o:borderrightcolor
    #[xml(attr = "o:borderrightcolor")]
    pub border_right_color: Option<String>,
    /// Diagram Node Layout Identifier
    /// Represents the following attribute in the schema: o:dgmlayout
    #[xml(attr = "o:dgmlayout")]
    pub diagram_layout: Option<i32>,
    /// Diagram Node Identifier
    /// Represents the following attribute in the schema: o:dgmnodekind
    #[xml(attr = "o:dgmnodekind")]
    pub diagram_node_kind: Option<i32>,
    /// Diagram Node Recent Layout Identifier
    /// Represents the following attribute in the schema: o:dgmlayoutmru
    #[xml(attr = "o:dgmlayoutmru")]
    pub diagram_layout_most_recent_used: Option<i32>,
    /// Text Inset Mode
    /// Represents the following attribute in the schema: o:insetmode
    #[xml(attr = "o:insetmode")]
    pub inset_mode: Option<
        crate::schemas::schemas_microsoft_com_office_office::InsetMarginValues,
    >,
    /// Shape Fill Toggle
    /// Represents the following attribute in the schema: :filled
    #[xml(attr = "filled")]
    pub filled: Option<bool>,
    /// Fill Color
    /// Represents the following attribute in the schema: :fillcolor
    #[xml(attr = "fillcolor")]
    pub fill_color: Option<String>,
    /// Shape Stroke Toggle
    /// Represents the following attribute in the schema: :stroked
    #[xml(attr = "stroked")]
    pub stroked: Option<bool>,
    /// Shape Stroke Color
    /// Represents the following attribute in the schema: :strokecolor
    #[xml(attr = "strokecolor")]
    pub stroke_color: Option<String>,
    /// Shape Stroke Weight
    /// Represents the following attribute in the schema: :strokeweight
    #[xml(attr = "strokeweight")]
    pub stroke_weight: Option<String>,
    /// Inset Border From Path
    /// Represents the following attribute in the schema: :insetpen
    #[xml(attr = "insetpen")]
    pub inset_pen: Option<bool>,
    /// Optional Number
    /// Represents the following attribute in the schema: o:spt
    #[xml(attr = "o:spt")]
    pub optional_number: Option<i32>,
    /// Shape Connector Type
    /// Represents the following attribute in the schema: o:connectortype
    #[xml(attr = "o:connectortype")]
    pub connector_type: Option<
        crate::schemas::schemas_microsoft_com_office_office::ConnectorValues,
    >,
    /// Black-and-White Mode
    /// Represents the following attribute in the schema: o:bwmode
    #[xml(attr = "o:bwmode")]
    pub black_white_mode: Option<
        crate::schemas::schemas_microsoft_com_office_office::BlackAndWhiteModeValues,
    >,
    /// Pure Black-and-White Mode
    /// Represents the following attribute in the schema: o:bwpure
    #[xml(attr = "o:bwpure")]
    pub pure_black_white_mode: Option<
        crate::schemas::schemas_microsoft_com_office_office::BlackAndWhiteModeValues,
    >,
    /// Normal Black-and-White Mode
    /// Represents the following attribute in the schema: o:bwnormal
    #[xml(attr = "o:bwnormal")]
    pub normal_black_white_mode: Option<
        crate::schemas::schemas_microsoft_com_office_office::BlackAndWhiteModeValues,
    >,
    /// Force Dashed Outline
    /// Represents the following attribute in the schema: o:forcedash
    #[xml(attr = "o:forcedash")]
    pub force_dash: Option<bool>,
    /// Embedded Object Icon Toggle
    /// Represents the following attribute in the schema: o:oleicon
    #[xml(attr = "o:oleicon")]
    pub ole_icon: Option<bool>,
    /// Embedded Object Toggle
    /// Represents the following attribute in the schema: o:ole
    #[xml(attr = "o:ole")]
    pub ole: Option<bool>,
    /// Relative Resize Toggle
    /// Represents the following attribute in the schema: o:preferrelative
    #[xml(attr = "o:preferrelative")]
    pub prefer_relative: Option<bool>,
    /// Clip to Wrapping Polygon
    /// Represents the following attribute in the schema: o:cliptowrap
    #[xml(attr = "o:cliptowrap")]
    pub clip_to_wrap: Option<bool>,
    /// Clipping Toggle
    /// Represents the following attribute in the schema: o:clip
    #[xml(attr = "o:clip")]
    pub clip: Option<bool>,
    /// Shape Type Reference
    /// Represents the following attribute in the schema: :type
    #[xml(attr = "type")]
    pub r#type: Option<String>,
    /// Adjustment Parameters
    /// Represents the following attribute in the schema: :adj
    #[xml(attr = "adj")]
    pub adjustment: Option<String>,
    /// Edge Path
    /// Represents the following attribute in the schema: :path
    #[xml(attr = "path")]
    pub edge_path: Option<String>,
    /// Encoded Package
    /// Represents the following attribute in the schema: o:gfxdata
    #[xml(attr = "o:gfxdata")]
    pub encoded_package: Option<String>,
    /// Storage for Alternate Math Content
    /// Represents the following attribute in the schema: :equationxml
    #[xml(attr = "equationxml")]
    pub equation_xml: Option<String>,
    #[xml(
        child = "v:path",
        child = "v:formulas",
        child = "v:handles",
        child = "v:fill",
        child = "v:stroke",
        child = "v:shadow",
        child = "v:textbox",
        child = "v:textpath",
        child = "v:imagedata",
        child = "o:skew",
        child = "o:extrusion",
        child = "o:callout",
        child = "o:lock",
        child = "o:clippath",
        child = "o:signatureline",
        child = "w10:wrap",
        child = "w10:anchorlock",
        child = "w10:bordertop",
        child = "w10:borderbottom",
        child = "w10:borderleft",
        child = "w10:borderright",
        child = "xvml:ClientData",
        child = "pvml:textdata",
        child = "o:ink",
        child = "pvml:iscomment",
    )]
    pub children: Vec<ShapeChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum ShapeChildChoice {
    #[xml(tag = "v:path")]
    VPath(Path),
    #[xml(tag = "v:formulas")]
    VFormulas(Formulas),
    #[xml(tag = "v:handles")]
    VHandles(ShapeHandles),
    #[xml(tag = "v:fill")]
    VFill(Fill),
    #[xml(tag = "v:stroke")]
    VStroke(Stroke),
    #[xml(tag = "v:shadow")]
    VShadow(Shadow),
    #[xml(tag = "v:textbox")]
    VTextbox(TextBox),
    #[xml(tag = "v:textpath")]
    VTextpath(TextPath),
    #[xml(tag = "v:imagedata")]
    VImagedata(ImageData),
    #[xml(tag = "o:skew")]
    OSkew(crate::schemas::schemas_microsoft_com_office_office::Skew),
    #[xml(tag = "o:extrusion")]
    OExtrusion(crate::schemas::schemas_microsoft_com_office_office::Extrusion),
    #[xml(tag = "o:callout")]
    OCallout(crate::schemas::schemas_microsoft_com_office_office::Callout),
    #[xml(tag = "o:lock")]
    OLock(crate::schemas::schemas_microsoft_com_office_office::Lock),
    #[xml(tag = "o:clippath")]
    OClippath(crate::schemas::schemas_microsoft_com_office_office::ClipPath),
    #[xml(tag = "o:signatureline")]
    OSignatureline(crate::schemas::schemas_microsoft_com_office_office::SignatureLine),
    #[xml(tag = "w10:wrap")]
    W10Wrap(crate::schemas::schemas_microsoft_com_office_word::TextWrap),
    #[xml(tag = "w10:anchorlock")]
    W10Anchorlock(crate::schemas::schemas_microsoft_com_office_word::AnchorLock),
    #[xml(tag = "w10:bordertop")]
    W10Bordertop(crate::schemas::schemas_microsoft_com_office_word::TopBorder),
    #[xml(tag = "w10:borderbottom")]
    W10Borderbottom(crate::schemas::schemas_microsoft_com_office_word::BottomBorder),
    #[xml(tag = "w10:borderleft")]
    W10Borderleft(crate::schemas::schemas_microsoft_com_office_word::LeftBorder),
    #[xml(tag = "w10:borderright")]
    W10Borderright(crate::schemas::schemas_microsoft_com_office_word::RightBorder),
    #[xml(tag = "xvml:ClientData")]
    XvmlClientData(crate::schemas::schemas_microsoft_com_office_excel::ClientData),
    #[xml(tag = "pvml:textdata")]
    PvmlTextdata(crate::schemas::schemas_microsoft_com_office_powerpoint::TextData),
    #[xml(tag = "o:ink")]
    OInk(crate::schemas::schemas_microsoft_com_office_office::Ink),
    #[xml(tag = "pvml:iscomment")]
    PvmlIscomment(
        crate::schemas::schemas_microsoft_com_office_powerpoint::InkAnnotationFlag,
    ),
}
/// Shape Template.
/// When the object is serialized out as xml, it's qualified name is v:shapetype.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "v:shapetype")]
pub struct Shapetype {
    /// Unique Identifier
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: Option<String>,
    /// Shape Styling Properties
    /// Represents the following attribute in the schema: :style
    #[xml(attr = "style")]
    pub style: Option<String>,
    /// Hyperlink Target
    /// Represents the following attribute in the schema: :href
    #[xml(attr = "href")]
    pub href: Option<String>,
    /// Hyperlink Display Target
    /// Represents the following attribute in the schema: :target
    #[xml(attr = "target")]
    pub target: Option<String>,
    /// CSS Reference
    /// Represents the following attribute in the schema: :class
    #[xml(attr = "class")]
    pub class: Option<String>,
    /// Shape Title
    /// Represents the following attribute in the schema: :title
    #[xml(attr = "title")]
    pub title: Option<String>,
    /// Alternate Text
    /// Represents the following attribute in the schema: :alt
    #[xml(attr = "alt")]
    pub alternate: Option<String>,
    /// Coordinate Space Size
    /// Represents the following attribute in the schema: :coordsize
    #[xml(attr = "coordsize")]
    pub coordinate_size: Option<String>,
    /// Coordinate Space Origin
    /// Represents the following attribute in the schema: :coordorigin
    #[xml(attr = "coordorigin")]
    pub coordinate_origin: Option<String>,
    /// Shape Bounding Polygon
    /// Represents the following attribute in the schema: :wrapcoords
    #[xml(attr = "wrapcoords")]
    pub wrap_coordinates: Option<String>,
    /// Print Toggle
    /// Represents the following attribute in the schema: :print
    #[xml(attr = "print")]
    pub print: Option<bool>,
    /// Optional String
    /// Represents the following attribute in the schema: o:spid
    #[xml(attr = "o:spid")]
    pub optional_string: Option<String>,
    /// Shape Handle Toggle
    /// Represents the following attribute in the schema: o:oned
    #[xml(attr = "o:oned")]
    pub oned: Option<bool>,
    /// Regroup ID
    /// Represents the following attribute in the schema: o:regroupid
    #[xml(attr = "o:regroupid")]
    pub regroup_id: Option<i32>,
    /// Double-click Notification Toggle
    /// Represents the following attribute in the schema: o:doubleclicknotify
    #[xml(attr = "o:doubleclicknotify")]
    pub double_click_notify: Option<bool>,
    /// Button Behavior Toggle
    /// Represents the following attribute in the schema: o:button
    #[xml(attr = "o:button")]
    pub button: Option<bool>,
    /// Hide Script Anchors
    /// Represents the following attribute in the schema: o:userhidden
    #[xml(attr = "o:userhidden")]
    pub user_hidden: Option<bool>,
    /// Graphical Bullet
    /// Represents the following attribute in the schema: o:bullet
    #[xml(attr = "o:bullet")]
    pub bullet: Option<bool>,
    /// Horizontal Rule Toggle
    /// Represents the following attribute in the schema: o:hr
    #[xml(attr = "o:hr")]
    pub horizontal: Option<bool>,
    /// Horizontal Rule Standard Display Toggle
    /// Represents the following attribute in the schema: o:hrstd
    #[xml(attr = "o:hrstd")]
    pub horizontal_standard: Option<bool>,
    /// Horizontal Rule 3D Shading Toggle
    /// Represents the following attribute in the schema: o:hrnoshade
    #[xml(attr = "o:hrnoshade")]
    pub horizontal_no_shade: Option<bool>,
    /// Horizontal Rule Length Percentage
    /// Represents the following attribute in the schema: o:hrpct
    #[xml(attr = "o:hrpct")]
    pub horizontal_percentage: Option<f32>,
    /// Horizontal Rule Alignment
    /// Represents the following attribute in the schema: o:hralign
    #[xml(attr = "o:hralign")]
    pub horizontal_alignment: Option<
        crate::schemas::schemas_microsoft_com_office_office::HorizontalRuleAlignmentValues,
    >,
    /// Allow in Table Cell
    /// Represents the following attribute in the schema: o:allowincell
    #[xml(attr = "o:allowincell")]
    pub allow_in_cell: Option<bool>,
    /// Allow Shape Overlap
    /// Represents the following attribute in the schema: o:allowoverlap
    #[xml(attr = "o:allowoverlap")]
    pub allow_overlap: Option<bool>,
    /// Exists In Master Slide
    /// Represents the following attribute in the schema: o:userdrawn
    #[xml(attr = "o:userdrawn")]
    pub user_drawn: Option<bool>,
    /// Border Top Color
    /// Represents the following attribute in the schema: o:bordertopcolor
    #[xml(attr = "o:bordertopcolor")]
    pub border_top_color: Option<String>,
    /// Border Left Color
    /// Represents the following attribute in the schema: o:borderleftcolor
    #[xml(attr = "o:borderleftcolor")]
    pub border_left_color: Option<String>,
    /// Bottom Border Color
    /// Represents the following attribute in the schema: o:borderbottomcolor
    #[xml(attr = "o:borderbottomcolor")]
    pub border_bottom_color: Option<String>,
    /// Border Right Color
    /// Represents the following attribute in the schema: o:borderrightcolor
    #[xml(attr = "o:borderrightcolor")]
    pub border_right_color: Option<String>,
    /// Diagram Node Layout Identifier
    /// Represents the following attribute in the schema: o:dgmlayout
    #[xml(attr = "o:dgmlayout")]
    pub diagram_layout: Option<i32>,
    /// Diagram Node Identifier
    /// Represents the following attribute in the schema: o:dgmnodekind
    #[xml(attr = "o:dgmnodekind")]
    pub diagram_node_kind: Option<i32>,
    /// Diagram Node Recent Layout Identifier
    /// Represents the following attribute in the schema: o:dgmlayoutmru
    #[xml(attr = "o:dgmlayoutmru")]
    pub diagram_layout_most_recent_used: Option<i32>,
    /// Text Inset Mode
    /// Represents the following attribute in the schema: o:insetmode
    #[xml(attr = "o:insetmode")]
    pub inset_mode: Option<
        crate::schemas::schemas_microsoft_com_office_office::InsetMarginValues,
    >,
    /// Shape Fill Toggle
    /// Represents the following attribute in the schema: :filled
    #[xml(attr = "filled")]
    pub filled: Option<bool>,
    /// Fill Color
    /// Represents the following attribute in the schema: :fillcolor
    #[xml(attr = "fillcolor")]
    pub fill_color: Option<String>,
    /// Shape Stroke Toggle
    /// Represents the following attribute in the schema: :stroked
    #[xml(attr = "stroked")]
    pub stroked: Option<bool>,
    /// Shape Stroke Color
    /// Represents the following attribute in the schema: :strokecolor
    #[xml(attr = "strokecolor")]
    pub stroke_color: Option<String>,
    /// Shape Stroke Weight
    /// Represents the following attribute in the schema: :strokeweight
    #[xml(attr = "strokeweight")]
    pub stroke_weight: Option<String>,
    /// Inset Border From Path
    /// Represents the following attribute in the schema: :insetpen
    #[xml(attr = "insetpen")]
    pub inset_pen: Option<bool>,
    /// Optional Number
    /// Represents the following attribute in the schema: o:spt
    #[xml(attr = "o:spt")]
    pub optional_number: Option<i32>,
    /// Shape Connector Type
    /// Represents the following attribute in the schema: o:connectortype
    #[xml(attr = "o:connectortype")]
    pub connector_type: Option<
        crate::schemas::schemas_microsoft_com_office_office::ConnectorValues,
    >,
    /// Black-and-White Mode
    /// Represents the following attribute in the schema: o:bwmode
    #[xml(attr = "o:bwmode")]
    pub black_white_mode: Option<
        crate::schemas::schemas_microsoft_com_office_office::BlackAndWhiteModeValues,
    >,
    /// Pure Black-and-White Mode
    /// Represents the following attribute in the schema: o:bwpure
    #[xml(attr = "o:bwpure")]
    pub pure_black_white_mode: Option<
        crate::schemas::schemas_microsoft_com_office_office::BlackAndWhiteModeValues,
    >,
    /// Normal Black-and-White Mode
    /// Represents the following attribute in the schema: o:bwnormal
    #[xml(attr = "o:bwnormal")]
    pub normal_black_white_mode: Option<
        crate::schemas::schemas_microsoft_com_office_office::BlackAndWhiteModeValues,
    >,
    /// Force Dashed Outline
    /// Represents the following attribute in the schema: o:forcedash
    #[xml(attr = "o:forcedash")]
    pub force_dash: Option<bool>,
    /// Embedded Object Icon Toggle
    /// Represents the following attribute in the schema: o:oleicon
    #[xml(attr = "o:oleicon")]
    pub ole_icon: Option<bool>,
    /// Embedded Object Toggle
    /// Represents the following attribute in the schema: o:ole
    #[xml(attr = "o:ole")]
    pub ole: Option<bool>,
    /// Relative Resize Toggle
    /// Represents the following attribute in the schema: o:preferrelative
    #[xml(attr = "o:preferrelative")]
    pub prefer_relative: Option<bool>,
    /// Clip to Wrapping Polygon
    /// Represents the following attribute in the schema: o:cliptowrap
    #[xml(attr = "o:cliptowrap")]
    pub clip_to_wrap: Option<bool>,
    /// Clipping Toggle
    /// Represents the following attribute in the schema: o:clip
    #[xml(attr = "o:clip")]
    pub clip: Option<bool>,
    /// Adjustment Parameters
    /// Represents the following attribute in the schema: :adj
    #[xml(attr = "adj")]
    pub adjustment: Option<String>,
    /// Edge Path
    /// Represents the following attribute in the schema: :path
    #[xml(attr = "path")]
    pub edge_path: Option<String>,
    /// Master Element Toggle
    /// Represents the following attribute in the schema: o:master
    #[xml(attr = "o:master")]
    pub master: Option<String>,
    #[xml(
        child = "v:path",
        child = "v:formulas",
        child = "v:handles",
        child = "v:fill",
        child = "v:stroke",
        child = "v:shadow",
        child = "v:textbox",
        child = "v:textpath",
        child = "v:imagedata",
        child = "o:skew",
        child = "o:extrusion",
        child = "o:callout",
        child = "o:lock",
        child = "o:clippath",
        child = "o:signatureline",
        child = "w10:wrap",
        child = "w10:anchorlock",
        child = "w10:bordertop",
        child = "w10:borderbottom",
        child = "w10:borderleft",
        child = "w10:borderright",
        child = "xvml:ClientData",
        child = "pvml:textdata",
        child = "o:complex",
    )]
    pub children: Vec<ShapetypeChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum ShapetypeChildChoice {
    #[xml(tag = "v:path")]
    VPath(Path),
    #[xml(tag = "v:formulas")]
    VFormulas(Formulas),
    #[xml(tag = "v:handles")]
    VHandles(ShapeHandles),
    #[xml(tag = "v:fill")]
    VFill(Fill),
    #[xml(tag = "v:stroke")]
    VStroke(Stroke),
    #[xml(tag = "v:shadow")]
    VShadow(Shadow),
    #[xml(tag = "v:textbox")]
    VTextbox(TextBox),
    #[xml(tag = "v:textpath")]
    VTextpath(TextPath),
    #[xml(tag = "v:imagedata")]
    VImagedata(ImageData),
    #[xml(tag = "o:skew")]
    OSkew(crate::schemas::schemas_microsoft_com_office_office::Skew),
    #[xml(tag = "o:extrusion")]
    OExtrusion(crate::schemas::schemas_microsoft_com_office_office::Extrusion),
    #[xml(tag = "o:callout")]
    OCallout(crate::schemas::schemas_microsoft_com_office_office::Callout),
    #[xml(tag = "o:lock")]
    OLock(crate::schemas::schemas_microsoft_com_office_office::Lock),
    #[xml(tag = "o:clippath")]
    OClippath(crate::schemas::schemas_microsoft_com_office_office::ClipPath),
    #[xml(tag = "o:signatureline")]
    OSignatureline(crate::schemas::schemas_microsoft_com_office_office::SignatureLine),
    #[xml(tag = "w10:wrap")]
    W10Wrap(crate::schemas::schemas_microsoft_com_office_word::TextWrap),
    #[xml(tag = "w10:anchorlock")]
    W10Anchorlock(crate::schemas::schemas_microsoft_com_office_word::AnchorLock),
    #[xml(tag = "w10:bordertop")]
    W10Bordertop(crate::schemas::schemas_microsoft_com_office_word::TopBorder),
    #[xml(tag = "w10:borderbottom")]
    W10Borderbottom(crate::schemas::schemas_microsoft_com_office_word::BottomBorder),
    #[xml(tag = "w10:borderleft")]
    W10Borderleft(crate::schemas::schemas_microsoft_com_office_word::LeftBorder),
    #[xml(tag = "w10:borderright")]
    W10Borderright(crate::schemas::schemas_microsoft_com_office_word::RightBorder),
    #[xml(tag = "xvml:ClientData")]
    XvmlClientData(crate::schemas::schemas_microsoft_com_office_excel::ClientData),
    #[xml(tag = "pvml:textdata")]
    PvmlTextdata(crate::schemas::schemas_microsoft_com_office_powerpoint::TextData),
    #[xml(tag = "o:complex")]
    OComplex(crate::schemas::schemas_microsoft_com_office_office::Complex),
}
/// Shape Group.
/// When the object is serialized out as xml, it's qualified name is v:group.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "v:group")]
pub struct Group {
    /// Unique Identifier
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: Option<String>,
    /// Shape Styling Properties
    /// Represents the following attribute in the schema: :style
    #[xml(attr = "style")]
    pub style: Option<String>,
    /// Hyperlink Target
    /// Represents the following attribute in the schema: :href
    #[xml(attr = "href")]
    pub href: Option<String>,
    /// Hyperlink Display Target
    /// Represents the following attribute in the schema: :target
    #[xml(attr = "target")]
    pub target: Option<String>,
    /// CSS Reference
    /// Represents the following attribute in the schema: :class
    #[xml(attr = "class")]
    pub class: Option<String>,
    /// Shape Title
    /// Represents the following attribute in the schema: :title
    #[xml(attr = "title")]
    pub title: Option<String>,
    /// Alternate Text
    /// Represents the following attribute in the schema: :alt
    #[xml(attr = "alt")]
    pub alternate: Option<String>,
    /// Coordinate Space Size
    /// Represents the following attribute in the schema: :coordsize
    #[xml(attr = "coordsize")]
    pub coordinate_size: Option<String>,
    /// Coordinate Space Origin
    /// Represents the following attribute in the schema: :coordorigin
    #[xml(attr = "coordorigin")]
    pub coordinate_origin: Option<String>,
    /// Shape Bounding Polygon
    /// Represents the following attribute in the schema: :wrapcoords
    #[xml(attr = "wrapcoords")]
    pub wrap_coordinates: Option<String>,
    /// Print Toggle
    /// Represents the following attribute in the schema: :print
    #[xml(attr = "print")]
    pub print: Option<bool>,
    /// spid
    /// Represents the following attribute in the schema: o:spid
    #[xml(attr = "o:spid")]
    pub optional_string: Option<String>,
    /// oned
    /// Represents the following attribute in the schema: o:oned
    #[xml(attr = "o:oned")]
    pub oned: Option<bool>,
    /// regroupid
    /// Represents the following attribute in the schema: o:regroupid
    #[xml(attr = "o:regroupid")]
    pub regroup_id: Option<i32>,
    /// doubleclicknotify
    /// Represents the following attribute in the schema: o:doubleclicknotify
    #[xml(attr = "o:doubleclicknotify")]
    pub double_click_notify: Option<bool>,
    /// button
    /// Represents the following attribute in the schema: o:button
    #[xml(attr = "o:button")]
    pub button: Option<bool>,
    /// userhidden
    /// Represents the following attribute in the schema: o:userhidden
    #[xml(attr = "o:userhidden")]
    pub user_hidden: Option<bool>,
    /// bullet
    /// Represents the following attribute in the schema: o:bullet
    #[xml(attr = "o:bullet")]
    pub bullet: Option<bool>,
    /// hr
    /// Represents the following attribute in the schema: o:hr
    #[xml(attr = "o:hr")]
    pub horizontal: Option<bool>,
    /// hrstd
    /// Represents the following attribute in the schema: o:hrstd
    #[xml(attr = "o:hrstd")]
    pub horizontal_standard: Option<bool>,
    /// hrnoshade
    /// Represents the following attribute in the schema: o:hrnoshade
    #[xml(attr = "o:hrnoshade")]
    pub horizontal_no_shade: Option<bool>,
    /// hrpct
    /// Represents the following attribute in the schema: o:hrpct
    #[xml(attr = "o:hrpct")]
    pub horizontal_percentage: Option<f32>,
    /// hralign
    /// Represents the following attribute in the schema: o:hralign
    #[xml(attr = "o:hralign")]
    pub horizontal_alignment: Option<
        crate::schemas::schemas_microsoft_com_office_office::HorizontalRuleAlignmentValues,
    >,
    /// allowincell
    /// Represents the following attribute in the schema: o:allowincell
    #[xml(attr = "o:allowincell")]
    pub allow_in_cell: Option<bool>,
    /// allowoverlap
    /// Represents the following attribute in the schema: o:allowoverlap
    #[xml(attr = "o:allowoverlap")]
    pub allow_overlap: Option<bool>,
    /// userdrawn
    /// Represents the following attribute in the schema: o:userdrawn
    #[xml(attr = "o:userdrawn")]
    pub user_drawn: Option<bool>,
    /// dgmlayout
    /// Represents the following attribute in the schema: o:dgmlayout
    #[xml(attr = "o:dgmlayout")]
    pub diagram_layout: Option<i32>,
    /// dgmnodekind
    /// Represents the following attribute in the schema: o:dgmnodekind
    #[xml(attr = "o:dgmnodekind")]
    pub diagram_node_kind: Option<i32>,
    /// dgmlayoutmru
    /// Represents the following attribute in the schema: o:dgmlayoutmru
    #[xml(attr = "o:dgmlayoutmru")]
    pub diagram_layout_most_recent_used: Option<i32>,
    /// insetmode
    /// Represents the following attribute in the schema: o:insetmode
    #[xml(attr = "o:insetmode")]
    pub inset_mode: Option<
        crate::schemas::schemas_microsoft_com_office_office::InsetMarginValues,
    >,
    /// Encoded Package
    /// Represents the following attribute in the schema: o:gfxdata
    #[xml(attr = "o:gfxdata")]
    pub o_gfxdata: Option<String>,
    /// Group Diagram Type
    /// Represents the following attribute in the schema: :editas
    #[xml(attr = "editas")]
    pub edit_as: Option<EditAsValues>,
    /// Table Properties
    /// Represents the following attribute in the schema: o:tableproperties
    #[xml(attr = "o:tableproperties")]
    pub table_properties: Option<String>,
    /// Table Row Height Limits
    /// Represents the following attribute in the schema: o:tablelimits
    #[xml(attr = "o:tablelimits")]
    pub table_limits: Option<String>,
    #[xml(
        child = "v:group",
        child = "v:shape",
        child = "v:shapetype",
        child = "v:arc",
        child = "v:curve",
        child = "v:image",
        child = "v:line",
        child = "v:oval",
        child = "v:polyline",
        child = "v:rect",
        child = "v:roundrect",
        child = "o:diagram",
        child = "o:lock",
        child = "o:clippath",
        child = "w10:wrap",
        child = "w10:anchorlock",
        child = "xvml:ClientData",
    )]
    pub children: Vec<GroupChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum GroupChildChoice {
    #[xml(tag = "v:group")]
    VGroup(Group),
    #[xml(tag = "v:shape")]
    VShape(Shape),
    #[xml(tag = "v:shapetype")]
    VShapetype(Shapetype),
    #[xml(tag = "v:arc")]
    VArc(Arc),
    #[xml(tag = "v:curve")]
    VCurve(Curve),
    #[xml(tag = "v:image")]
    VImage(ImageFile),
    #[xml(tag = "v:line")]
    VLine(Line),
    #[xml(tag = "v:oval")]
    VOval(Oval),
    #[xml(tag = "v:polyline")]
    VPolyline(PolyLine),
    #[xml(tag = "v:rect")]
    VRect(Rectangle),
    #[xml(tag = "v:roundrect")]
    VRoundrect(RoundRectangle),
    #[xml(tag = "o:diagram")]
    ODiagram(crate::schemas::schemas_microsoft_com_office_office::Diagram),
    #[xml(tag = "o:lock")]
    OLock(crate::schemas::schemas_microsoft_com_office_office::Lock),
    #[xml(tag = "o:clippath")]
    OClippath(crate::schemas::schemas_microsoft_com_office_office::ClipPath),
    #[xml(tag = "w10:wrap")]
    W10Wrap(crate::schemas::schemas_microsoft_com_office_word::TextWrap),
    #[xml(tag = "w10:anchorlock")]
    W10Anchorlock(crate::schemas::schemas_microsoft_com_office_word::AnchorLock),
    #[xml(tag = "xvml:ClientData")]
    XvmlClientData(crate::schemas::schemas_microsoft_com_office_excel::ClientData),
}
/// Document Background.
/// When the object is serialized out as xml, it's qualified name is v:background.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "v:background")]
pub struct Background {
    /// Unique Identifier
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: Option<String>,
    /// Shape Fill Toggle
    /// Represents the following attribute in the schema: :fill
    #[xml(attr = "fill")]
    pub filled: Option<bool>,
    /// Fill Color
    /// Represents the following attribute in the schema: :fillcolor
    #[xml(attr = "fillcolor")]
    pub fillcolor: Option<String>,
    /// Black-and-White Mode
    /// Represents the following attribute in the schema: o:bwmode
    #[xml(attr = "o:bwmode")]
    pub black_white_mode: Option<
        crate::schemas::schemas_microsoft_com_office_office::BlackAndWhiteModeValues,
    >,
    /// Pure Black-and-White Mode
    /// Represents the following attribute in the schema: o:bwpure
    #[xml(attr = "o:bwpure")]
    pub pure_black_white_mode: Option<
        crate::schemas::schemas_microsoft_com_office_office::BlackAndWhiteModeValues,
    >,
    /// Normal Black-and-White Mode
    /// Represents the following attribute in the schema: o:bwnormal
    #[xml(attr = "o:bwnormal")]
    pub normal_black_white_mode: Option<
        crate::schemas::schemas_microsoft_com_office_office::BlackAndWhiteModeValues,
    >,
    /// Target Screen Size
    /// Represents the following attribute in the schema: o:targetscreensize
    #[xml(attr = "o:targetscreensize")]
    pub target_screen_size: Option<
        crate::schemas::schemas_microsoft_com_office_office::ScreenSizeValues,
    >,
    /// _
    #[xml(child = "v:fill")]
    pub fill: Option<Fill>,
}
/// Arc Segment.
/// When the object is serialized out as xml, it's qualified name is v:arc.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "v:arc")]
pub struct Arc {
    /// Optional String
    /// Represents the following attribute in the schema: o:spid
    #[xml(attr = "o:spid")]
    pub optional_string: Option<String>,
    /// Shape Handle Toggle
    /// Represents the following attribute in the schema: o:oned
    #[xml(attr = "o:oned")]
    pub oned: Option<bool>,
    /// Regroup ID
    /// Represents the following attribute in the schema: o:regroupid
    #[xml(attr = "o:regroupid")]
    pub regroup_id: Option<i32>,
    /// Double-click Notification Toggle
    /// Represents the following attribute in the schema: o:doubleclicknotify
    #[xml(attr = "o:doubleclicknotify")]
    pub double_click_notify: Option<bool>,
    /// Button Behavior Toggle
    /// Represents the following attribute in the schema: o:button
    #[xml(attr = "o:button")]
    pub button: Option<bool>,
    /// Hide Script Anchors
    /// Represents the following attribute in the schema: o:userhidden
    #[xml(attr = "o:userhidden")]
    pub user_hidden: Option<bool>,
    /// Graphical Bullet
    /// Represents the following attribute in the schema: o:bullet
    #[xml(attr = "o:bullet")]
    pub bullet: Option<bool>,
    /// Horizontal Rule Toggle
    /// Represents the following attribute in the schema: o:hr
    #[xml(attr = "o:hr")]
    pub horizontal: Option<bool>,
    /// Horizontal Rule Standard Display Toggle
    /// Represents the following attribute in the schema: o:hrstd
    #[xml(attr = "o:hrstd")]
    pub horizontal_standard: Option<bool>,
    /// Horizontal Rule 3D Shading Toggle
    /// Represents the following attribute in the schema: o:hrnoshade
    #[xml(attr = "o:hrnoshade")]
    pub horizontal_no_shade: Option<bool>,
    /// Horizontal Rule Length Percentage
    /// Represents the following attribute in the schema: o:hrpct
    #[xml(attr = "o:hrpct")]
    pub horizontal_percentage: Option<f32>,
    /// Horizontal Rule Alignment
    /// Represents the following attribute in the schema: o:hralign
    #[xml(attr = "o:hralign")]
    pub horizontal_alignment: Option<
        crate::schemas::schemas_microsoft_com_office_office::HorizontalRuleAlignmentValues,
    >,
    /// Allow in Table Cell
    /// Represents the following attribute in the schema: o:allowincell
    #[xml(attr = "o:allowincell")]
    pub allow_in_cell: Option<bool>,
    /// Allow Shape Overlap
    /// Represents the following attribute in the schema: o:allowoverlap
    #[xml(attr = "o:allowoverlap")]
    pub allow_overlap: Option<bool>,
    /// Exists In Master Slide
    /// Represents the following attribute in the schema: o:userdrawn
    #[xml(attr = "o:userdrawn")]
    pub user_drawn: Option<bool>,
    /// Border Top Color
    /// Represents the following attribute in the schema: o:bordertopcolor
    #[xml(attr = "o:bordertopcolor")]
    pub border_top_color: Option<String>,
    /// Border Left Color
    /// Represents the following attribute in the schema: o:borderleftcolor
    #[xml(attr = "o:borderleftcolor")]
    pub border_left_color: Option<String>,
    /// Bottom Border Color
    /// Represents the following attribute in the schema: o:borderbottomcolor
    #[xml(attr = "o:borderbottomcolor")]
    pub border_bottom_color: Option<String>,
    /// Border Right Color
    /// Represents the following attribute in the schema: o:borderrightcolor
    #[xml(attr = "o:borderrightcolor")]
    pub border_right_color: Option<String>,
    /// Diagram Node Layout Identifier
    /// Represents the following attribute in the schema: o:dgmlayout
    #[xml(attr = "o:dgmlayout")]
    pub diagram_layout: Option<i32>,
    /// Diagram Node Identifier
    /// Represents the following attribute in the schema: o:dgmnodekind
    #[xml(attr = "o:dgmnodekind")]
    pub diagram_node_kind: Option<i32>,
    /// Diagram Node Recent Layout Identifier
    /// Represents the following attribute in the schema: o:dgmlayoutmru
    #[xml(attr = "o:dgmlayoutmru")]
    pub diagram_layout_most_recent_used: Option<i32>,
    /// Text Inset Mode
    /// Represents the following attribute in the schema: o:insetmode
    #[xml(attr = "o:insetmode")]
    pub inset_mode: Option<
        crate::schemas::schemas_microsoft_com_office_office::InsetMarginValues,
    >,
    /// Shape Fill Toggle
    /// Represents the following attribute in the schema: :filled
    #[xml(attr = "filled")]
    pub filled: Option<bool>,
    /// Fill Color
    /// Represents the following attribute in the schema: :fillcolor
    #[xml(attr = "fillcolor")]
    pub fill_color: Option<String>,
    /// Shape Stroke Toggle
    /// Represents the following attribute in the schema: :stroked
    #[xml(attr = "stroked")]
    pub stroked: Option<bool>,
    /// Shape Stroke Color
    /// Represents the following attribute in the schema: :strokecolor
    #[xml(attr = "strokecolor")]
    pub stroke_color: Option<String>,
    /// Shape Stroke Weight
    /// Represents the following attribute in the schema: :strokeweight
    #[xml(attr = "strokeweight")]
    pub stroke_weight: Option<String>,
    /// Inset Border From Path
    /// Represents the following attribute in the schema: :insetpen
    #[xml(attr = "insetpen")]
    pub inset_pen: Option<bool>,
    /// Optional Number
    /// Represents the following attribute in the schema: o:spt
    #[xml(attr = "o:spt")]
    pub optional_number: Option<i32>,
    /// Shape Connector Type
    /// Represents the following attribute in the schema: o:connectortype
    #[xml(attr = "o:connectortype")]
    pub connector_type: Option<
        crate::schemas::schemas_microsoft_com_office_office::ConnectorValues,
    >,
    /// Black-and-White Mode
    /// Represents the following attribute in the schema: o:bwmode
    #[xml(attr = "o:bwmode")]
    pub black_white_mode: Option<
        crate::schemas::schemas_microsoft_com_office_office::BlackAndWhiteModeValues,
    >,
    /// Pure Black-and-White Mode
    /// Represents the following attribute in the schema: o:bwpure
    #[xml(attr = "o:bwpure")]
    pub pure_black_white_mode: Option<
        crate::schemas::schemas_microsoft_com_office_office::BlackAndWhiteModeValues,
    >,
    /// Normal Black-and-White Mode
    /// Represents the following attribute in the schema: o:bwnormal
    #[xml(attr = "o:bwnormal")]
    pub normal_black_white_mode: Option<
        crate::schemas::schemas_microsoft_com_office_office::BlackAndWhiteModeValues,
    >,
    /// Force Dashed Outline
    /// Represents the following attribute in the schema: o:forcedash
    #[xml(attr = "o:forcedash")]
    pub force_dash: Option<bool>,
    /// Embedded Object Icon Toggle
    /// Represents the following attribute in the schema: o:oleicon
    #[xml(attr = "o:oleicon")]
    pub ole_icon: Option<bool>,
    /// Embedded Object Toggle
    /// Represents the following attribute in the schema: o:ole
    #[xml(attr = "o:ole")]
    pub ole: Option<bool>,
    /// Relative Resize Toggle
    /// Represents the following attribute in the schema: o:preferrelative
    #[xml(attr = "o:preferrelative")]
    pub prefer_relative: Option<bool>,
    /// Clip to Wrapping Polygon
    /// Represents the following attribute in the schema: o:cliptowrap
    #[xml(attr = "o:cliptowrap")]
    pub clip_to_wrap: Option<bool>,
    /// Clipping Toggle
    /// Represents the following attribute in the schema: o:clip
    #[xml(attr = "o:clip")]
    pub clip: Option<bool>,
    /// Encoded Package
    /// Represents the following attribute in the schema: o:gfxdata
    #[xml(attr = "o:gfxdata")]
    pub o_gfxdata: Option<String>,
    /// Unique Identifier
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: Option<String>,
    /// Shape Styling Properties
    /// Represents the following attribute in the schema: :style
    #[xml(attr = "style")]
    pub style: Option<String>,
    /// Hyperlink Target
    /// Represents the following attribute in the schema: :href
    #[xml(attr = "href")]
    pub href: Option<String>,
    /// Hyperlink Display Target
    /// Represents the following attribute in the schema: :target
    #[xml(attr = "target")]
    pub target: Option<String>,
    /// Shape Title
    /// Represents the following attribute in the schema: :title
    #[xml(attr = "title")]
    pub title: Option<String>,
    /// Alternate Text
    /// Represents the following attribute in the schema: :alt
    #[xml(attr = "alt")]
    pub alternate: Option<String>,
    /// Coordinate Space Size
    /// Represents the following attribute in the schema: :coordsize
    #[xml(attr = "coordsize")]
    pub coordinate_size: Option<String>,
    /// Coordinate Space Origin
    /// Represents the following attribute in the schema: :coordorigin
    #[xml(attr = "coordorigin")]
    pub coordinate_origin: Option<String>,
    /// Shape Bounding Polygon
    /// Represents the following attribute in the schema: :wrapcoords
    #[xml(attr = "wrapcoords")]
    pub wrapcoords: Option<String>,
    /// Print Toggle
    /// Represents the following attribute in the schema: :print
    #[xml(attr = "print")]
    pub print: Option<bool>,
    /// Starting Angle
    /// Represents the following attribute in the schema: :startangle
    #[xml(attr = "startangle")]
    pub start_angle: Option<String>,
    /// Ending Angle
    /// Represents the following attribute in the schema: :endangle
    #[xml(attr = "endangle")]
    pub end_angle: Option<String>,
    #[xml(
        child = "v:path",
        child = "v:formulas",
        child = "v:handles",
        child = "v:fill",
        child = "v:stroke",
        child = "v:shadow",
        child = "v:textbox",
        child = "v:textpath",
        child = "v:imagedata",
        child = "o:skew",
        child = "o:extrusion",
        child = "o:callout",
        child = "o:lock",
        child = "o:clippath",
        child = "o:signatureline",
        child = "w10:wrap",
        child = "w10:anchorlock",
        child = "w10:bordertop",
        child = "w10:borderbottom",
        child = "w10:borderleft",
        child = "w10:borderright",
        child = "xvml:ClientData",
        child = "pvml:textdata",
    )]
    pub children: Vec<ArcChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum ArcChildChoice {
    #[xml(tag = "v:path")]
    VPath(Path),
    #[xml(tag = "v:formulas")]
    VFormulas(Formulas),
    #[xml(tag = "v:handles")]
    VHandles(ShapeHandles),
    #[xml(tag = "v:fill")]
    VFill(Fill),
    #[xml(tag = "v:stroke")]
    VStroke(Stroke),
    #[xml(tag = "v:shadow")]
    VShadow(Shadow),
    #[xml(tag = "v:textbox")]
    VTextbox(TextBox),
    #[xml(tag = "v:textpath")]
    VTextpath(TextPath),
    #[xml(tag = "v:imagedata")]
    VImagedata(ImageData),
    #[xml(tag = "o:skew")]
    OSkew(crate::schemas::schemas_microsoft_com_office_office::Skew),
    #[xml(tag = "o:extrusion")]
    OExtrusion(crate::schemas::schemas_microsoft_com_office_office::Extrusion),
    #[xml(tag = "o:callout")]
    OCallout(crate::schemas::schemas_microsoft_com_office_office::Callout),
    #[xml(tag = "o:lock")]
    OLock(crate::schemas::schemas_microsoft_com_office_office::Lock),
    #[xml(tag = "o:clippath")]
    OClippath(crate::schemas::schemas_microsoft_com_office_office::ClipPath),
    #[xml(tag = "o:signatureline")]
    OSignatureline(crate::schemas::schemas_microsoft_com_office_office::SignatureLine),
    #[xml(tag = "w10:wrap")]
    W10Wrap(crate::schemas::schemas_microsoft_com_office_word::TextWrap),
    #[xml(tag = "w10:anchorlock")]
    W10Anchorlock(crate::schemas::schemas_microsoft_com_office_word::AnchorLock),
    #[xml(tag = "w10:bordertop")]
    W10Bordertop(crate::schemas::schemas_microsoft_com_office_word::TopBorder),
    #[xml(tag = "w10:borderbottom")]
    W10Borderbottom(crate::schemas::schemas_microsoft_com_office_word::BottomBorder),
    #[xml(tag = "w10:borderleft")]
    W10Borderleft(crate::schemas::schemas_microsoft_com_office_word::LeftBorder),
    #[xml(tag = "w10:borderright")]
    W10Borderright(crate::schemas::schemas_microsoft_com_office_word::RightBorder),
    #[xml(tag = "xvml:ClientData")]
    XvmlClientData(crate::schemas::schemas_microsoft_com_office_excel::ClientData),
    #[xml(tag = "pvml:textdata")]
    PvmlTextdata(crate::schemas::schemas_microsoft_com_office_powerpoint::TextData),
}
/// Bezier Curve.
/// When the object is serialized out as xml, it's qualified name is v:curve.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "v:curve")]
pub struct Curve {
    /// Unique Identifier
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: Option<String>,
    /// Shape Styling Properties
    /// Represents the following attribute in the schema: :style
    #[xml(attr = "style")]
    pub style: Option<String>,
    /// Hyperlink Target
    /// Represents the following attribute in the schema: :href
    #[xml(attr = "href")]
    pub href: Option<String>,
    /// Hyperlink Display Target
    /// Represents the following attribute in the schema: :target
    #[xml(attr = "target")]
    pub target: Option<String>,
    /// CSS Reference
    /// Represents the following attribute in the schema: :class
    #[xml(attr = "class")]
    pub class: Option<String>,
    /// Shape Title
    /// Represents the following attribute in the schema: :title
    #[xml(attr = "title")]
    pub title: Option<String>,
    /// Alternate Text
    /// Represents the following attribute in the schema: :alt
    #[xml(attr = "alt")]
    pub alternate: Option<String>,
    /// Coordinate Space Size
    /// Represents the following attribute in the schema: :coordsize
    #[xml(attr = "coordsize")]
    pub coordinate_size: Option<String>,
    /// Coordinate Space Origin
    /// Represents the following attribute in the schema: :coordorigin
    #[xml(attr = "coordorigin")]
    pub coordinate_origin: Option<String>,
    /// Shape Bounding Polygon
    /// Represents the following attribute in the schema: :wrapcoords
    #[xml(attr = "wrapcoords")]
    pub wrap_coordinates: Option<String>,
    /// Print Toggle
    /// Represents the following attribute in the schema: :print
    #[xml(attr = "print")]
    pub print: Option<bool>,
    /// Optional String
    /// Represents the following attribute in the schema: o:spid
    #[xml(attr = "o:spid")]
    pub optional_string: Option<String>,
    /// Shape Handle Toggle
    /// Represents the following attribute in the schema: o:oned
    #[xml(attr = "o:oned")]
    pub oned: Option<bool>,
    /// Regroup ID
    /// Represents the following attribute in the schema: o:regroupid
    #[xml(attr = "o:regroupid")]
    pub regroup_id: Option<i32>,
    /// Double-click Notification Toggle
    /// Represents the following attribute in the schema: o:doubleclicknotify
    #[xml(attr = "o:doubleclicknotify")]
    pub double_click_notify: Option<bool>,
    /// Button Behavior Toggle
    /// Represents the following attribute in the schema: o:button
    #[xml(attr = "o:button")]
    pub button: Option<bool>,
    /// Hide Script Anchors
    /// Represents the following attribute in the schema: o:userhidden
    #[xml(attr = "o:userhidden")]
    pub user_hidden: Option<bool>,
    /// Graphical Bullet
    /// Represents the following attribute in the schema: o:bullet
    #[xml(attr = "o:bullet")]
    pub bullet: Option<bool>,
    /// Horizontal Rule Toggle
    /// Represents the following attribute in the schema: o:hr
    #[xml(attr = "o:hr")]
    pub horizontal: Option<bool>,
    /// Horizontal Rule Standard Display Toggle
    /// Represents the following attribute in the schema: o:hrstd
    #[xml(attr = "o:hrstd")]
    pub horizontal_standard: Option<bool>,
    /// Horizontal Rule 3D Shading Toggle
    /// Represents the following attribute in the schema: o:hrnoshade
    #[xml(attr = "o:hrnoshade")]
    pub horizontal_no_shade: Option<bool>,
    /// Horizontal Rule Length Percentage
    /// Represents the following attribute in the schema: o:hrpct
    #[xml(attr = "o:hrpct")]
    pub horizontal_percentage: Option<f32>,
    /// Horizontal Rule Alignment
    /// Represents the following attribute in the schema: o:hralign
    #[xml(attr = "o:hralign")]
    pub horizontal_alignment: Option<
        crate::schemas::schemas_microsoft_com_office_office::HorizontalRuleAlignmentValues,
    >,
    /// Allow in Table Cell
    /// Represents the following attribute in the schema: o:allowincell
    #[xml(attr = "o:allowincell")]
    pub allow_in_cell: Option<bool>,
    /// Allow Shape Overlap
    /// Represents the following attribute in the schema: o:allowoverlap
    #[xml(attr = "o:allowoverlap")]
    pub allow_overlap: Option<bool>,
    /// Exists In Master Slide
    /// Represents the following attribute in the schema: o:userdrawn
    #[xml(attr = "o:userdrawn")]
    pub user_drawn: Option<bool>,
    /// Border Top Color
    /// Represents the following attribute in the schema: o:bordertopcolor
    #[xml(attr = "o:bordertopcolor")]
    pub border_top_color: Option<String>,
    /// Border Left Color
    /// Represents the following attribute in the schema: o:borderleftcolor
    #[xml(attr = "o:borderleftcolor")]
    pub border_left_color: Option<String>,
    /// Bottom Border Color
    /// Represents the following attribute in the schema: o:borderbottomcolor
    #[xml(attr = "o:borderbottomcolor")]
    pub border_bottom_color: Option<String>,
    /// Border Right Color
    /// Represents the following attribute in the schema: o:borderrightcolor
    #[xml(attr = "o:borderrightcolor")]
    pub border_right_color: Option<String>,
    /// Diagram Node Layout Identifier
    /// Represents the following attribute in the schema: o:dgmlayout
    #[xml(attr = "o:dgmlayout")]
    pub diagram_layout: Option<i32>,
    /// Diagram Node Identifier
    /// Represents the following attribute in the schema: o:dgmnodekind
    #[xml(attr = "o:dgmnodekind")]
    pub diagram_node_kind: Option<i32>,
    /// Diagram Node Recent Layout Identifier
    /// Represents the following attribute in the schema: o:dgmlayoutmru
    #[xml(attr = "o:dgmlayoutmru")]
    pub diagram_layout_most_recent_used: Option<i32>,
    /// Text Inset Mode
    /// Represents the following attribute in the schema: o:insetmode
    #[xml(attr = "o:insetmode")]
    pub inset_mode: Option<
        crate::schemas::schemas_microsoft_com_office_office::InsetMarginValues,
    >,
    /// Shape Fill Toggle
    /// Represents the following attribute in the schema: :filled
    #[xml(attr = "filled")]
    pub filled: Option<bool>,
    /// Fill Color
    /// Represents the following attribute in the schema: :fillcolor
    #[xml(attr = "fillcolor")]
    pub fill_color: Option<String>,
    /// Shape Stroke Toggle
    /// Represents the following attribute in the schema: :stroked
    #[xml(attr = "stroked")]
    pub stroked: Option<bool>,
    /// Shape Stroke Color
    /// Represents the following attribute in the schema: :strokecolor
    #[xml(attr = "strokecolor")]
    pub stroke_color: Option<String>,
    /// Shape Stroke Weight
    /// Represents the following attribute in the schema: :strokeweight
    #[xml(attr = "strokeweight")]
    pub stroke_weight: Option<String>,
    /// Inset Border From Path
    /// Represents the following attribute in the schema: :insetpen
    #[xml(attr = "insetpen")]
    pub inset_pen: Option<bool>,
    /// Optional Number
    /// Represents the following attribute in the schema: o:spt
    #[xml(attr = "o:spt")]
    pub optional_number: Option<i32>,
    /// Shape Connector Type
    /// Represents the following attribute in the schema: o:connectortype
    #[xml(attr = "o:connectortype")]
    pub connector_type: Option<
        crate::schemas::schemas_microsoft_com_office_office::ConnectorValues,
    >,
    /// Black-and-White Mode
    /// Represents the following attribute in the schema: o:bwmode
    #[xml(attr = "o:bwmode")]
    pub black_white_mode: Option<
        crate::schemas::schemas_microsoft_com_office_office::BlackAndWhiteModeValues,
    >,
    /// Pure Black-and-White Mode
    /// Represents the following attribute in the schema: o:bwpure
    #[xml(attr = "o:bwpure")]
    pub pure_black_white_mode: Option<
        crate::schemas::schemas_microsoft_com_office_office::BlackAndWhiteModeValues,
    >,
    /// Normal Black-and-White Mode
    /// Represents the following attribute in the schema: o:bwnormal
    #[xml(attr = "o:bwnormal")]
    pub normal_black_white_mode: Option<
        crate::schemas::schemas_microsoft_com_office_office::BlackAndWhiteModeValues,
    >,
    /// Force Dashed Outline
    /// Represents the following attribute in the schema: o:forcedash
    #[xml(attr = "o:forcedash")]
    pub force_dash: Option<bool>,
    /// Embedded Object Icon Toggle
    /// Represents the following attribute in the schema: o:oleicon
    #[xml(attr = "o:oleicon")]
    pub ole_icon: Option<bool>,
    /// Embedded Object Toggle
    /// Represents the following attribute in the schema: o:ole
    #[xml(attr = "o:ole")]
    pub ole: Option<bool>,
    /// Relative Resize Toggle
    /// Represents the following attribute in the schema: o:preferrelative
    #[xml(attr = "o:preferrelative")]
    pub prefer_relative: Option<bool>,
    /// Clip to Wrapping Polygon
    /// Represents the following attribute in the schema: o:cliptowrap
    #[xml(attr = "o:cliptowrap")]
    pub clip_to_wrap: Option<bool>,
    /// Clipping Toggle
    /// Represents the following attribute in the schema: o:clip
    #[xml(attr = "o:clip")]
    pub clip: Option<bool>,
    /// Encoded Package
    /// Represents the following attribute in the schema: o:gfxdata
    #[xml(attr = "o:gfxdata")]
    pub o_gfxdata: Option<String>,
    /// Curve Starting Point
    /// Represents the following attribute in the schema: :from
    #[xml(attr = "from")]
    pub from: Option<String>,
    /// First Curve Control Point
    /// Represents the following attribute in the schema: :control1
    #[xml(attr = "control1")]
    pub control1: Option<String>,
    /// Second Curve Control Point
    /// Represents the following attribute in the schema: :control2
    #[xml(attr = "control2")]
    pub control2: Option<String>,
    /// Curve Ending Point
    /// Represents the following attribute in the schema: :to
    #[xml(attr = "to")]
    pub to: Option<String>,
    #[xml(
        child = "v:path",
        child = "v:formulas",
        child = "v:handles",
        child = "v:fill",
        child = "v:stroke",
        child = "v:shadow",
        child = "v:textbox",
        child = "v:textpath",
        child = "v:imagedata",
        child = "o:skew",
        child = "o:extrusion",
        child = "o:callout",
        child = "o:lock",
        child = "o:clippath",
        child = "o:signatureline",
        child = "w10:wrap",
        child = "w10:anchorlock",
        child = "w10:bordertop",
        child = "w10:borderbottom",
        child = "w10:borderleft",
        child = "w10:borderright",
        child = "xvml:ClientData",
        child = "pvml:textdata",
    )]
    pub children: Vec<CurveChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum CurveChildChoice {
    #[xml(tag = "v:path")]
    VPath(Path),
    #[xml(tag = "v:formulas")]
    VFormulas(Formulas),
    #[xml(tag = "v:handles")]
    VHandles(ShapeHandles),
    #[xml(tag = "v:fill")]
    VFill(Fill),
    #[xml(tag = "v:stroke")]
    VStroke(Stroke),
    #[xml(tag = "v:shadow")]
    VShadow(Shadow),
    #[xml(tag = "v:textbox")]
    VTextbox(TextBox),
    #[xml(tag = "v:textpath")]
    VTextpath(TextPath),
    #[xml(tag = "v:imagedata")]
    VImagedata(ImageData),
    #[xml(tag = "o:skew")]
    OSkew(crate::schemas::schemas_microsoft_com_office_office::Skew),
    #[xml(tag = "o:extrusion")]
    OExtrusion(crate::schemas::schemas_microsoft_com_office_office::Extrusion),
    #[xml(tag = "o:callout")]
    OCallout(crate::schemas::schemas_microsoft_com_office_office::Callout),
    #[xml(tag = "o:lock")]
    OLock(crate::schemas::schemas_microsoft_com_office_office::Lock),
    #[xml(tag = "o:clippath")]
    OClippath(crate::schemas::schemas_microsoft_com_office_office::ClipPath),
    #[xml(tag = "o:signatureline")]
    OSignatureline(crate::schemas::schemas_microsoft_com_office_office::SignatureLine),
    #[xml(tag = "w10:wrap")]
    W10Wrap(crate::schemas::schemas_microsoft_com_office_word::TextWrap),
    #[xml(tag = "w10:anchorlock")]
    W10Anchorlock(crate::schemas::schemas_microsoft_com_office_word::AnchorLock),
    #[xml(tag = "w10:bordertop")]
    W10Bordertop(crate::schemas::schemas_microsoft_com_office_word::TopBorder),
    #[xml(tag = "w10:borderbottom")]
    W10Borderbottom(crate::schemas::schemas_microsoft_com_office_word::BottomBorder),
    #[xml(tag = "w10:borderleft")]
    W10Borderleft(crate::schemas::schemas_microsoft_com_office_word::LeftBorder),
    #[xml(tag = "w10:borderright")]
    W10Borderright(crate::schemas::schemas_microsoft_com_office_word::RightBorder),
    #[xml(tag = "xvml:ClientData")]
    XvmlClientData(crate::schemas::schemas_microsoft_com_office_excel::ClientData),
    #[xml(tag = "pvml:textdata")]
    PvmlTextdata(crate::schemas::schemas_microsoft_com_office_powerpoint::TextData),
}
/// Image File.
/// When the object is serialized out as xml, it's qualified name is v:image.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "v:image")]
pub struct ImageFile {
    /// Unique Identifier
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: Option<String>,
    /// style
    /// Represents the following attribute in the schema: :style
    #[xml(attr = "style")]
    pub style: Option<String>,
    /// href
    /// Represents the following attribute in the schema: :href
    #[xml(attr = "href")]
    pub href: Option<String>,
    /// target
    /// Represents the following attribute in the schema: :target
    #[xml(attr = "target")]
    pub target: Option<String>,
    /// class
    /// Represents the following attribute in the schema: :class
    #[xml(attr = "class")]
    pub class: Option<String>,
    /// title
    /// Represents the following attribute in the schema: :title
    #[xml(attr = "title")]
    pub title: Option<String>,
    /// alt
    /// Represents the following attribute in the schema: :alt
    #[xml(attr = "alt")]
    pub alternate: Option<String>,
    /// coordsize
    /// Represents the following attribute in the schema: :coordsize
    #[xml(attr = "coordsize")]
    pub coordinate_size: Option<String>,
    /// wrapcoords
    /// Represents the following attribute in the schema: :wrapcoords
    #[xml(attr = "wrapcoords")]
    pub wrap_coordinates: Option<String>,
    /// print
    /// Represents the following attribute in the schema: :print
    #[xml(attr = "print")]
    pub print: Option<bool>,
    /// Optional String
    /// Represents the following attribute in the schema: o:spid
    #[xml(attr = "o:spid")]
    pub optional_string: Option<String>,
    /// Shape Handle Toggle
    /// Represents the following attribute in the schema: o:oned
    #[xml(attr = "o:oned")]
    pub oned: Option<bool>,
    /// Regroup ID
    /// Represents the following attribute in the schema: o:regroupid
    #[xml(attr = "o:regroupid")]
    pub regroup_id: Option<i32>,
    /// Double-click Notification Toggle
    /// Represents the following attribute in the schema: o:doubleclicknotify
    #[xml(attr = "o:doubleclicknotify")]
    pub double_click_notify: Option<bool>,
    /// Button Behavior Toggle
    /// Represents the following attribute in the schema: o:button
    #[xml(attr = "o:button")]
    pub button: Option<bool>,
    /// Hide Script Anchors
    /// Represents the following attribute in the schema: o:userhidden
    #[xml(attr = "o:userhidden")]
    pub user_hidden: Option<bool>,
    /// Graphical Bullet
    /// Represents the following attribute in the schema: o:bullet
    #[xml(attr = "o:bullet")]
    pub bullet: Option<bool>,
    /// Horizontal Rule Toggle
    /// Represents the following attribute in the schema: o:hr
    #[xml(attr = "o:hr")]
    pub horizontal: Option<bool>,
    /// Horizontal Rule Standard Display Toggle
    /// Represents the following attribute in the schema: o:hrstd
    #[xml(attr = "o:hrstd")]
    pub horizontal_standard: Option<bool>,
    /// Horizontal Rule 3D Shading Toggle
    /// Represents the following attribute in the schema: o:hrnoshade
    #[xml(attr = "o:hrnoshade")]
    pub horizontal_no_shade: Option<bool>,
    /// Horizontal Rule Length Percentage
    /// Represents the following attribute in the schema: o:hrpct
    #[xml(attr = "o:hrpct")]
    pub horizontal_percentage: Option<f32>,
    /// Horizontal Rule Alignment
    /// Represents the following attribute in the schema: o:hralign
    #[xml(attr = "o:hralign")]
    pub horizontal_alignment: Option<
        crate::schemas::schemas_microsoft_com_office_office::HorizontalRuleAlignmentValues,
    >,
    /// Allow in Table Cell
    /// Represents the following attribute in the schema: o:allowincell
    #[xml(attr = "o:allowincell")]
    pub allow_in_cell: Option<bool>,
    /// Allow Shape Overlap
    /// Represents the following attribute in the schema: o:allowoverlap
    #[xml(attr = "o:allowoverlap")]
    pub allow_overlap: Option<bool>,
    /// Exists In Master Slide
    /// Represents the following attribute in the schema: o:userdrawn
    #[xml(attr = "o:userdrawn")]
    pub user_drawn: Option<bool>,
    /// Border Top Color
    /// Represents the following attribute in the schema: o:bordertopcolor
    #[xml(attr = "o:bordertopcolor")]
    pub border_top_color: Option<String>,
    /// Border Left Color
    /// Represents the following attribute in the schema: o:borderleftcolor
    #[xml(attr = "o:borderleftcolor")]
    pub border_left_color: Option<String>,
    /// Bottom Border Color
    /// Represents the following attribute in the schema: o:borderbottomcolor
    #[xml(attr = "o:borderbottomcolor")]
    pub border_bottom_color: Option<String>,
    /// Border Right Color
    /// Represents the following attribute in the schema: o:borderrightcolor
    #[xml(attr = "o:borderrightcolor")]
    pub border_right_color: Option<String>,
    /// Diagram Node Layout Identifier
    /// Represents the following attribute in the schema: o:dgmlayout
    #[xml(attr = "o:dgmlayout")]
    pub diagram_layout: Option<i32>,
    /// Diagram Node Identifier
    /// Represents the following attribute in the schema: o:dgmnodekind
    #[xml(attr = "o:dgmnodekind")]
    pub diagram_node_kind: Option<i32>,
    /// Diagram Node Recent Layout Identifier
    /// Represents the following attribute in the schema: o:dgmlayoutmru
    #[xml(attr = "o:dgmlayoutmru")]
    pub diagram_layout_most_recent_used: Option<i32>,
    /// Text Inset Mode
    /// Represents the following attribute in the schema: o:insetmode
    #[xml(attr = "o:insetmode")]
    pub inset_mode: Option<
        crate::schemas::schemas_microsoft_com_office_office::InsetMarginValues,
    >,
    /// Shape Fill Toggle
    /// Represents the following attribute in the schema: :filled
    #[xml(attr = "filled")]
    pub filled: Option<bool>,
    /// Fill Color
    /// Represents the following attribute in the schema: :fillcolor
    #[xml(attr = "fillcolor")]
    pub fill_color: Option<String>,
    /// Shape Stroke Toggle
    /// Represents the following attribute in the schema: :stroked
    #[xml(attr = "stroked")]
    pub stroked: Option<bool>,
    /// Shape Stroke Color
    /// Represents the following attribute in the schema: :strokecolor
    #[xml(attr = "strokecolor")]
    pub stroke_color: Option<String>,
    /// Shape Stroke Weight
    /// Represents the following attribute in the schema: :strokeweight
    #[xml(attr = "strokeweight")]
    pub stroke_weight: Option<String>,
    /// Inset Border From Path
    /// Represents the following attribute in the schema: :insetpen
    #[xml(attr = "insetpen")]
    pub inset_pen: Option<bool>,
    /// Optional Number
    /// Represents the following attribute in the schema: o:spt
    #[xml(attr = "o:spt")]
    pub optional_number: Option<i32>,
    /// Shape Connector Type
    /// Represents the following attribute in the schema: o:connectortype
    #[xml(attr = "o:connectortype")]
    pub connector_type: Option<
        crate::schemas::schemas_microsoft_com_office_office::ConnectorValues,
    >,
    /// Black-and-White Mode
    /// Represents the following attribute in the schema: o:bwmode
    #[xml(attr = "o:bwmode")]
    pub black_white_mode: Option<
        crate::schemas::schemas_microsoft_com_office_office::BlackAndWhiteModeValues,
    >,
    /// Pure Black-and-White Mode
    /// Represents the following attribute in the schema: o:bwpure
    #[xml(attr = "o:bwpure")]
    pub pure_black_white_mode: Option<
        crate::schemas::schemas_microsoft_com_office_office::BlackAndWhiteModeValues,
    >,
    /// Normal Black-and-White Mode
    /// Represents the following attribute in the schema: o:bwnormal
    #[xml(attr = "o:bwnormal")]
    pub normal_black_white_mode: Option<
        crate::schemas::schemas_microsoft_com_office_office::BlackAndWhiteModeValues,
    >,
    /// Force Dashed Outline
    /// Represents the following attribute in the schema: o:forcedash
    #[xml(attr = "o:forcedash")]
    pub force_dash: Option<bool>,
    /// Embedded Object Icon Toggle
    /// Represents the following attribute in the schema: o:oleicon
    #[xml(attr = "o:oleicon")]
    pub ole_icon: Option<bool>,
    /// Embedded Object Toggle
    /// Represents the following attribute in the schema: o:ole
    #[xml(attr = "o:ole")]
    pub ole: Option<bool>,
    /// Relative Resize Toggle
    /// Represents the following attribute in the schema: o:preferrelative
    #[xml(attr = "o:preferrelative")]
    pub prefer_relative: Option<bool>,
    /// Clip to Wrapping Polygon
    /// Represents the following attribute in the schema: o:cliptowrap
    #[xml(attr = "o:cliptowrap")]
    pub clip_to_wrap: Option<bool>,
    /// Clipping Toggle
    /// Represents the following attribute in the schema: o:clip
    #[xml(attr = "o:clip")]
    pub clip: Option<bool>,
    /// Image Source
    /// Represents the following attribute in the schema: :src
    #[xml(attr = "src")]
    pub source: Option<String>,
    /// Image Left Crop
    /// Represents the following attribute in the schema: :cropleft
    #[xml(attr = "cropleft")]
    pub crop_left: Option<String>,
    /// Image Top Crop
    /// Represents the following attribute in the schema: :croptop
    #[xml(attr = "croptop")]
    pub crop_top: Option<String>,
    /// Image Right Crop
    /// Represents the following attribute in the schema: :cropright
    #[xml(attr = "cropright")]
    pub crop_right: Option<String>,
    /// Image Bottom Crop
    /// Represents the following attribute in the schema: :cropbottom
    #[xml(attr = "cropbottom")]
    pub crop_bottom: Option<String>,
    /// Image Intensity
    /// Represents the following attribute in the schema: :gain
    #[xml(attr = "gain")]
    pub gain: Option<String>,
    /// Image Brightness
    /// Represents the following attribute in the schema: :blacklevel
    #[xml(attr = "blacklevel")]
    pub black_level: Option<String>,
    /// Image Gamma Correction
    /// Represents the following attribute in the schema: :gamma
    #[xml(attr = "gamma")]
    pub gamma: Option<String>,
    /// Image Grayscale Toggle
    /// Represents the following attribute in the schema: :grayscale
    #[xml(attr = "grayscale")]
    pub gray_scale: Option<bool>,
    /// Image Bilevel Toggle
    /// Represents the following attribute in the schema: :bilevel
    #[xml(attr = "bilevel")]
    pub bi_level: Option<bool>,
    /// Encoded Package
    /// Represents the following attribute in the schema: o:gfxdata
    #[xml(attr = "o:gfxdata")]
    pub o_gfxdata: Option<String>,
    #[xml(
        child = "v:path",
        child = "v:formulas",
        child = "v:handles",
        child = "v:fill",
        child = "v:stroke",
        child = "v:shadow",
        child = "v:textbox",
        child = "v:textpath",
        child = "v:imagedata",
        child = "o:skew",
        child = "o:extrusion",
        child = "o:callout",
        child = "o:lock",
        child = "o:clippath",
        child = "o:signatureline",
        child = "w10:wrap",
        child = "w10:anchorlock",
        child = "w10:bordertop",
        child = "w10:borderbottom",
        child = "w10:borderleft",
        child = "w10:borderright",
        child = "xvml:ClientData",
        child = "pvml:textdata",
    )]
    pub children: Vec<ImageFileChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum ImageFileChildChoice {
    #[xml(tag = "v:path")]
    VPath(Path),
    #[xml(tag = "v:formulas")]
    VFormulas(Formulas),
    #[xml(tag = "v:handles")]
    VHandles(ShapeHandles),
    #[xml(tag = "v:fill")]
    VFill(Fill),
    #[xml(tag = "v:stroke")]
    VStroke(Stroke),
    #[xml(tag = "v:shadow")]
    VShadow(Shadow),
    #[xml(tag = "v:textbox")]
    VTextbox(TextBox),
    #[xml(tag = "v:textpath")]
    VTextpath(TextPath),
    #[xml(tag = "v:imagedata")]
    VImagedata(ImageData),
    #[xml(tag = "o:skew")]
    OSkew(crate::schemas::schemas_microsoft_com_office_office::Skew),
    #[xml(tag = "o:extrusion")]
    OExtrusion(crate::schemas::schemas_microsoft_com_office_office::Extrusion),
    #[xml(tag = "o:callout")]
    OCallout(crate::schemas::schemas_microsoft_com_office_office::Callout),
    #[xml(tag = "o:lock")]
    OLock(crate::schemas::schemas_microsoft_com_office_office::Lock),
    #[xml(tag = "o:clippath")]
    OClippath(crate::schemas::schemas_microsoft_com_office_office::ClipPath),
    #[xml(tag = "o:signatureline")]
    OSignatureline(crate::schemas::schemas_microsoft_com_office_office::SignatureLine),
    #[xml(tag = "w10:wrap")]
    W10Wrap(crate::schemas::schemas_microsoft_com_office_word::TextWrap),
    #[xml(tag = "w10:anchorlock")]
    W10Anchorlock(crate::schemas::schemas_microsoft_com_office_word::AnchorLock),
    #[xml(tag = "w10:bordertop")]
    W10Bordertop(crate::schemas::schemas_microsoft_com_office_word::TopBorder),
    #[xml(tag = "w10:borderbottom")]
    W10Borderbottom(crate::schemas::schemas_microsoft_com_office_word::BottomBorder),
    #[xml(tag = "w10:borderleft")]
    W10Borderleft(crate::schemas::schemas_microsoft_com_office_word::LeftBorder),
    #[xml(tag = "w10:borderright")]
    W10Borderright(crate::schemas::schemas_microsoft_com_office_word::RightBorder),
    #[xml(tag = "xvml:ClientData")]
    XvmlClientData(crate::schemas::schemas_microsoft_com_office_excel::ClientData),
    #[xml(tag = "pvml:textdata")]
    PvmlTextdata(crate::schemas::schemas_microsoft_com_office_powerpoint::TextData),
}
/// Line.
/// When the object is serialized out as xml, it's qualified name is v:line.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "v:line")]
pub struct Line {
    /// Unique Identifier
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: Option<String>,
    /// Shape Styling Properties
    /// Represents the following attribute in the schema: :style
    #[xml(attr = "style")]
    pub style: Option<String>,
    /// Hyperlink Target
    /// Represents the following attribute in the schema: :href
    #[xml(attr = "href")]
    pub href: Option<String>,
    /// Hyperlink Display Target
    /// Represents the following attribute in the schema: :target
    #[xml(attr = "target")]
    pub target: Option<String>,
    /// CSS Reference
    /// Represents the following attribute in the schema: :class
    #[xml(attr = "class")]
    pub class: Option<String>,
    /// Shape Title
    /// Represents the following attribute in the schema: :title
    #[xml(attr = "title")]
    pub title: Option<String>,
    /// Alternate Text
    /// Represents the following attribute in the schema: :alt
    #[xml(attr = "alt")]
    pub alternate: Option<String>,
    /// Coordinate Space Size
    /// Represents the following attribute in the schema: :coordsize
    #[xml(attr = "coordsize")]
    pub coordinate_size: Option<String>,
    /// Coordinate Space Origin
    /// Represents the following attribute in the schema: :coordorigin
    #[xml(attr = "coordorigin")]
    pub coordinate_origin: Option<String>,
    /// Shape Bounding Polygon
    /// Represents the following attribute in the schema: :wrapcoords
    #[xml(attr = "wrapcoords")]
    pub wrap_coordinates: Option<String>,
    /// Print Toggle
    /// Represents the following attribute in the schema: :print
    #[xml(attr = "print")]
    pub print: Option<bool>,
    /// Optional String
    /// Represents the following attribute in the schema: o:spid
    #[xml(attr = "o:spid")]
    pub optional_string: Option<String>,
    /// Shape Handle Toggle
    /// Represents the following attribute in the schema: o:oned
    #[xml(attr = "o:oned")]
    pub oned: Option<bool>,
    /// Regroup ID
    /// Represents the following attribute in the schema: o:regroupid
    #[xml(attr = "o:regroupid")]
    pub regroup_id: Option<i32>,
    /// Double-click Notification Toggle
    /// Represents the following attribute in the schema: o:doubleclicknotify
    #[xml(attr = "o:doubleclicknotify")]
    pub double_click_notify: Option<bool>,
    /// Button Behavior Toggle
    /// Represents the following attribute in the schema: o:button
    #[xml(attr = "o:button")]
    pub button: Option<bool>,
    /// Hide Script Anchors
    /// Represents the following attribute in the schema: o:userhidden
    #[xml(attr = "o:userhidden")]
    pub user_hidden: Option<bool>,
    /// Graphical Bullet
    /// Represents the following attribute in the schema: o:bullet
    #[xml(attr = "o:bullet")]
    pub bullet: Option<bool>,
    /// Horizontal Rule Toggle
    /// Represents the following attribute in the schema: o:hr
    #[xml(attr = "o:hr")]
    pub horizontal: Option<bool>,
    /// Horizontal Rule Standard Display Toggle
    /// Represents the following attribute in the schema: o:hrstd
    #[xml(attr = "o:hrstd")]
    pub horizontal_standard: Option<bool>,
    /// Horizontal Rule 3D Shading Toggle
    /// Represents the following attribute in the schema: o:hrnoshade
    #[xml(attr = "o:hrnoshade")]
    pub horizontal_no_shade: Option<bool>,
    /// Horizontal Rule Length Percentage
    /// Represents the following attribute in the schema: o:hrpct
    #[xml(attr = "o:hrpct")]
    pub horizontal_percentage: Option<f32>,
    /// Horizontal Rule Alignment
    /// Represents the following attribute in the schema: o:hralign
    #[xml(attr = "o:hralign")]
    pub horizontal_alignment: Option<
        crate::schemas::schemas_microsoft_com_office_office::HorizontalRuleAlignmentValues,
    >,
    /// Allow in Table Cell
    /// Represents the following attribute in the schema: o:allowincell
    #[xml(attr = "o:allowincell")]
    pub allow_in_cell: Option<bool>,
    /// Allow Shape Overlap
    /// Represents the following attribute in the schema: o:allowoverlap
    #[xml(attr = "o:allowoverlap")]
    pub allow_overlap: Option<bool>,
    /// Exists In Master Slide
    /// Represents the following attribute in the schema: o:userdrawn
    #[xml(attr = "o:userdrawn")]
    pub user_drawn: Option<bool>,
    /// Border Top Color
    /// Represents the following attribute in the schema: o:bordertopcolor
    #[xml(attr = "o:bordertopcolor")]
    pub border_top_color: Option<String>,
    /// Border Left Color
    /// Represents the following attribute in the schema: o:borderleftcolor
    #[xml(attr = "o:borderleftcolor")]
    pub border_left_color: Option<String>,
    /// Bottom Border Color
    /// Represents the following attribute in the schema: o:borderbottomcolor
    #[xml(attr = "o:borderbottomcolor")]
    pub border_bottom_color: Option<String>,
    /// Border Right Color
    /// Represents the following attribute in the schema: o:borderrightcolor
    #[xml(attr = "o:borderrightcolor")]
    pub border_right_color: Option<String>,
    /// Diagram Node Layout Identifier
    /// Represents the following attribute in the schema: o:dgmlayout
    #[xml(attr = "o:dgmlayout")]
    pub diagram_layout: Option<i32>,
    /// Diagram Node Identifier
    /// Represents the following attribute in the schema: o:dgmnodekind
    #[xml(attr = "o:dgmnodekind")]
    pub diagram_node_kind: Option<i32>,
    /// Diagram Node Recent Layout Identifier
    /// Represents the following attribute in the schema: o:dgmlayoutmru
    #[xml(attr = "o:dgmlayoutmru")]
    pub diagram_layout_most_recent_used: Option<i32>,
    /// Text Inset Mode
    /// Represents the following attribute in the schema: o:insetmode
    #[xml(attr = "o:insetmode")]
    pub inset_mode: Option<
        crate::schemas::schemas_microsoft_com_office_office::InsetMarginValues,
    >,
    /// Shape Fill Toggle
    /// Represents the following attribute in the schema: :filled
    #[xml(attr = "filled")]
    pub filled: Option<bool>,
    /// Fill Color
    /// Represents the following attribute in the schema: :fillcolor
    #[xml(attr = "fillcolor")]
    pub fill_color: Option<String>,
    /// Shape Stroke Toggle
    /// Represents the following attribute in the schema: :stroked
    #[xml(attr = "stroked")]
    pub stroked: Option<bool>,
    /// Shape Stroke Color
    /// Represents the following attribute in the schema: :strokecolor
    #[xml(attr = "strokecolor")]
    pub stroke_color: Option<String>,
    /// Shape Stroke Weight
    /// Represents the following attribute in the schema: :strokeweight
    #[xml(attr = "strokeweight")]
    pub stroke_weight: Option<String>,
    /// Inset Border From Path
    /// Represents the following attribute in the schema: :insetpen
    #[xml(attr = "insetpen")]
    pub inset_pen: Option<bool>,
    /// Optional Number
    /// Represents the following attribute in the schema: o:spt
    #[xml(attr = "o:spt")]
    pub optional_number: Option<i32>,
    /// Shape Connector Type
    /// Represents the following attribute in the schema: o:connectortype
    #[xml(attr = "o:connectortype")]
    pub connector_type: Option<
        crate::schemas::schemas_microsoft_com_office_office::ConnectorValues,
    >,
    /// Black-and-White Mode
    /// Represents the following attribute in the schema: o:bwmode
    #[xml(attr = "o:bwmode")]
    pub black_white_mode: Option<
        crate::schemas::schemas_microsoft_com_office_office::BlackAndWhiteModeValues,
    >,
    /// Pure Black-and-White Mode
    /// Represents the following attribute in the schema: o:bwpure
    #[xml(attr = "o:bwpure")]
    pub pure_black_white_mode: Option<
        crate::schemas::schemas_microsoft_com_office_office::BlackAndWhiteModeValues,
    >,
    /// Normal Black-and-White Mode
    /// Represents the following attribute in the schema: o:bwnormal
    #[xml(attr = "o:bwnormal")]
    pub normal_black_white_mode: Option<
        crate::schemas::schemas_microsoft_com_office_office::BlackAndWhiteModeValues,
    >,
    /// Force Dashed Outline
    /// Represents the following attribute in the schema: o:forcedash
    #[xml(attr = "o:forcedash")]
    pub force_dash: Option<bool>,
    /// Embedded Object Icon Toggle
    /// Represents the following attribute in the schema: o:oleicon
    #[xml(attr = "o:oleicon")]
    pub ole_icon: Option<bool>,
    /// Embedded Object Toggle
    /// Represents the following attribute in the schema: o:ole
    #[xml(attr = "o:ole")]
    pub ole: Option<bool>,
    /// Relative Resize Toggle
    /// Represents the following attribute in the schema: o:preferrelative
    #[xml(attr = "o:preferrelative")]
    pub prefer_relative: Option<bool>,
    /// Clip to Wrapping Polygon
    /// Represents the following attribute in the schema: o:cliptowrap
    #[xml(attr = "o:cliptowrap")]
    pub clip_to_wrap: Option<bool>,
    /// Clipping Toggle
    /// Represents the following attribute in the schema: o:clip
    #[xml(attr = "o:clip")]
    pub clip: Option<bool>,
    /// Encoded Package
    /// Represents the following attribute in the schema: o:gfxdata
    #[xml(attr = "o:gfxdata")]
    pub o_gfxdata: Option<String>,
    /// Line Start
    /// Represents the following attribute in the schema: :from
    #[xml(attr = "from")]
    pub from: Option<String>,
    /// Line End Point
    /// Represents the following attribute in the schema: :to
    #[xml(attr = "to")]
    pub to: Option<String>,
    #[xml(
        child = "v:path",
        child = "v:formulas",
        child = "v:handles",
        child = "v:fill",
        child = "v:stroke",
        child = "v:shadow",
        child = "v:textbox",
        child = "v:textpath",
        child = "v:imagedata",
        child = "o:skew",
        child = "o:extrusion",
        child = "o:callout",
        child = "o:lock",
        child = "o:clippath",
        child = "o:signatureline",
        child = "w10:wrap",
        child = "w10:anchorlock",
        child = "w10:bordertop",
        child = "w10:borderbottom",
        child = "w10:borderleft",
        child = "w10:borderright",
        child = "xvml:ClientData",
        child = "pvml:textdata",
    )]
    pub children: Vec<LineChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum LineChildChoice {
    #[xml(tag = "v:path")]
    VPath(Path),
    #[xml(tag = "v:formulas")]
    VFormulas(Formulas),
    #[xml(tag = "v:handles")]
    VHandles(ShapeHandles),
    #[xml(tag = "v:fill")]
    VFill(Fill),
    #[xml(tag = "v:stroke")]
    VStroke(Stroke),
    #[xml(tag = "v:shadow")]
    VShadow(Shadow),
    #[xml(tag = "v:textbox")]
    VTextbox(TextBox),
    #[xml(tag = "v:textpath")]
    VTextpath(TextPath),
    #[xml(tag = "v:imagedata")]
    VImagedata(ImageData),
    #[xml(tag = "o:skew")]
    OSkew(crate::schemas::schemas_microsoft_com_office_office::Skew),
    #[xml(tag = "o:extrusion")]
    OExtrusion(crate::schemas::schemas_microsoft_com_office_office::Extrusion),
    #[xml(tag = "o:callout")]
    OCallout(crate::schemas::schemas_microsoft_com_office_office::Callout),
    #[xml(tag = "o:lock")]
    OLock(crate::schemas::schemas_microsoft_com_office_office::Lock),
    #[xml(tag = "o:clippath")]
    OClippath(crate::schemas::schemas_microsoft_com_office_office::ClipPath),
    #[xml(tag = "o:signatureline")]
    OSignatureline(crate::schemas::schemas_microsoft_com_office_office::SignatureLine),
    #[xml(tag = "w10:wrap")]
    W10Wrap(crate::schemas::schemas_microsoft_com_office_word::TextWrap),
    #[xml(tag = "w10:anchorlock")]
    W10Anchorlock(crate::schemas::schemas_microsoft_com_office_word::AnchorLock),
    #[xml(tag = "w10:bordertop")]
    W10Bordertop(crate::schemas::schemas_microsoft_com_office_word::TopBorder),
    #[xml(tag = "w10:borderbottom")]
    W10Borderbottom(crate::schemas::schemas_microsoft_com_office_word::BottomBorder),
    #[xml(tag = "w10:borderleft")]
    W10Borderleft(crate::schemas::schemas_microsoft_com_office_word::LeftBorder),
    #[xml(tag = "w10:borderright")]
    W10Borderright(crate::schemas::schemas_microsoft_com_office_word::RightBorder),
    #[xml(tag = "xvml:ClientData")]
    XvmlClientData(crate::schemas::schemas_microsoft_com_office_excel::ClientData),
    #[xml(tag = "pvml:textdata")]
    PvmlTextdata(crate::schemas::schemas_microsoft_com_office_powerpoint::TextData),
}
/// Oval.
/// When the object is serialized out as xml, it's qualified name is v:oval.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "v:oval")]
pub struct Oval {
    /// Unique Identifier
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: Option<String>,
    /// Shape Styling Properties
    /// Represents the following attribute in the schema: :style
    #[xml(attr = "style")]
    pub style: Option<String>,
    /// Hyperlink Target
    /// Represents the following attribute in the schema: :href
    #[xml(attr = "href")]
    pub href: Option<String>,
    /// Hyperlink Display Target
    /// Represents the following attribute in the schema: :target
    #[xml(attr = "target")]
    pub target: Option<String>,
    /// CSS Reference
    /// Represents the following attribute in the schema: :class
    #[xml(attr = "class")]
    pub class: Option<String>,
    /// Shape Title
    /// Represents the following attribute in the schema: :title
    #[xml(attr = "title")]
    pub title: Option<String>,
    /// Alternate Text
    /// Represents the following attribute in the schema: :alt
    #[xml(attr = "alt")]
    pub alternate: Option<String>,
    /// Coordinate Space Size
    /// Represents the following attribute in the schema: :coordsize
    #[xml(attr = "coordsize")]
    pub coordinate_size: Option<String>,
    /// Coordinate Space Origin
    /// Represents the following attribute in the schema: :coordorigin
    #[xml(attr = "coordorigin")]
    pub coordinate_origin: Option<String>,
    /// Shape Bounding Polygon
    /// Represents the following attribute in the schema: :wrapcoords
    #[xml(attr = "wrapcoords")]
    pub wrap_coordinates: Option<String>,
    /// Print Toggle
    /// Represents the following attribute in the schema: :print
    #[xml(attr = "print")]
    pub print: Option<bool>,
    /// Optional String
    /// Represents the following attribute in the schema: o:spid
    #[xml(attr = "o:spid")]
    pub optional_string: Option<String>,
    /// Shape Handle Toggle
    /// Represents the following attribute in the schema: o:oned
    #[xml(attr = "o:oned")]
    pub oned: Option<bool>,
    /// Regroup ID
    /// Represents the following attribute in the schema: o:regroupid
    #[xml(attr = "o:regroupid")]
    pub regroup_id: Option<i32>,
    /// Double-click Notification Toggle
    /// Represents the following attribute in the schema: o:doubleclicknotify
    #[xml(attr = "o:doubleclicknotify")]
    pub double_click_notify: Option<bool>,
    /// Button Behavior Toggle
    /// Represents the following attribute in the schema: o:button
    #[xml(attr = "o:button")]
    pub button: Option<bool>,
    /// Hide Script Anchors
    /// Represents the following attribute in the schema: o:userhidden
    #[xml(attr = "o:userhidden")]
    pub user_hidden: Option<bool>,
    /// Graphical Bullet
    /// Represents the following attribute in the schema: o:bullet
    #[xml(attr = "o:bullet")]
    pub bullet: Option<bool>,
    /// Horizontal Rule Toggle
    /// Represents the following attribute in the schema: o:hr
    #[xml(attr = "o:hr")]
    pub horizontal: Option<bool>,
    /// Horizontal Rule Standard Display Toggle
    /// Represents the following attribute in the schema: o:hrstd
    #[xml(attr = "o:hrstd")]
    pub horizontal_standard: Option<bool>,
    /// Horizontal Rule 3D Shading Toggle
    /// Represents the following attribute in the schema: o:hrnoshade
    #[xml(attr = "o:hrnoshade")]
    pub horizontal_no_shade: Option<bool>,
    /// Horizontal Rule Length Percentage
    /// Represents the following attribute in the schema: o:hrpct
    #[xml(attr = "o:hrpct")]
    pub horizontal_percentage: Option<f32>,
    /// Horizontal Rule Alignment
    /// Represents the following attribute in the schema: o:hralign
    #[xml(attr = "o:hralign")]
    pub horizontal_alignment: Option<
        crate::schemas::schemas_microsoft_com_office_office::HorizontalRuleAlignmentValues,
    >,
    /// Allow in Table Cell
    /// Represents the following attribute in the schema: o:allowincell
    #[xml(attr = "o:allowincell")]
    pub allow_in_cell: Option<bool>,
    /// Allow Shape Overlap
    /// Represents the following attribute in the schema: o:allowoverlap
    #[xml(attr = "o:allowoverlap")]
    pub allow_overlap: Option<bool>,
    /// Exists In Master Slide
    /// Represents the following attribute in the schema: o:userdrawn
    #[xml(attr = "o:userdrawn")]
    pub user_drawn: Option<bool>,
    /// Border Top Color
    /// Represents the following attribute in the schema: o:bordertopcolor
    #[xml(attr = "o:bordertopcolor")]
    pub border_top_color: Option<String>,
    /// Border Left Color
    /// Represents the following attribute in the schema: o:borderleftcolor
    #[xml(attr = "o:borderleftcolor")]
    pub border_left_color: Option<String>,
    /// Bottom Border Color
    /// Represents the following attribute in the schema: o:borderbottomcolor
    #[xml(attr = "o:borderbottomcolor")]
    pub border_bottom_color: Option<String>,
    /// Border Right Color
    /// Represents the following attribute in the schema: o:borderrightcolor
    #[xml(attr = "o:borderrightcolor")]
    pub border_right_color: Option<String>,
    /// Diagram Node Layout Identifier
    /// Represents the following attribute in the schema: o:dgmlayout
    #[xml(attr = "o:dgmlayout")]
    pub diagram_layout: Option<i32>,
    /// Diagram Node Identifier
    /// Represents the following attribute in the schema: o:dgmnodekind
    #[xml(attr = "o:dgmnodekind")]
    pub diagram_node_kind: Option<i32>,
    /// Diagram Node Recent Layout Identifier
    /// Represents the following attribute in the schema: o:dgmlayoutmru
    #[xml(attr = "o:dgmlayoutmru")]
    pub diagram_layout_most_recent_used: Option<i32>,
    /// Text Inset Mode
    /// Represents the following attribute in the schema: o:insetmode
    #[xml(attr = "o:insetmode")]
    pub inset_mode: Option<
        crate::schemas::schemas_microsoft_com_office_office::InsetMarginValues,
    >,
    /// Shape Fill Toggle
    /// Represents the following attribute in the schema: :filled
    #[xml(attr = "filled")]
    pub filled: Option<bool>,
    /// Fill Color
    /// Represents the following attribute in the schema: :fillcolor
    #[xml(attr = "fillcolor")]
    pub fill_color: Option<String>,
    /// Shape Stroke Toggle
    /// Represents the following attribute in the schema: :stroked
    #[xml(attr = "stroked")]
    pub stroked: Option<bool>,
    /// Shape Stroke Color
    /// Represents the following attribute in the schema: :strokecolor
    #[xml(attr = "strokecolor")]
    pub stroke_color: Option<String>,
    /// Shape Stroke Weight
    /// Represents the following attribute in the schema: :strokeweight
    #[xml(attr = "strokeweight")]
    pub stroke_weight: Option<String>,
    /// Inset Border From Path
    /// Represents the following attribute in the schema: :insetpen
    #[xml(attr = "insetpen")]
    pub inset_pen: Option<bool>,
    /// Optional Number
    /// Represents the following attribute in the schema: o:spt
    #[xml(attr = "o:spt")]
    pub optional_number: Option<i32>,
    /// Shape Connector Type
    /// Represents the following attribute in the schema: o:connectortype
    #[xml(attr = "o:connectortype")]
    pub connector_type: Option<
        crate::schemas::schemas_microsoft_com_office_office::ConnectorValues,
    >,
    /// Black-and-White Mode
    /// Represents the following attribute in the schema: o:bwmode
    #[xml(attr = "o:bwmode")]
    pub black_white_mode: Option<
        crate::schemas::schemas_microsoft_com_office_office::BlackAndWhiteModeValues,
    >,
    /// Pure Black-and-White Mode
    /// Represents the following attribute in the schema: o:bwpure
    #[xml(attr = "o:bwpure")]
    pub pure_black_white_mode: Option<
        crate::schemas::schemas_microsoft_com_office_office::BlackAndWhiteModeValues,
    >,
    /// Normal Black-and-White Mode
    /// Represents the following attribute in the schema: o:bwnormal
    #[xml(attr = "o:bwnormal")]
    pub normal_black_white_mode: Option<
        crate::schemas::schemas_microsoft_com_office_office::BlackAndWhiteModeValues,
    >,
    /// Force Dashed Outline
    /// Represents the following attribute in the schema: o:forcedash
    #[xml(attr = "o:forcedash")]
    pub force_dash: Option<bool>,
    /// Embedded Object Icon Toggle
    /// Represents the following attribute in the schema: o:oleicon
    #[xml(attr = "o:oleicon")]
    pub ole_icon: Option<bool>,
    /// Embedded Object Toggle
    /// Represents the following attribute in the schema: o:ole
    #[xml(attr = "o:ole")]
    pub ole: Option<bool>,
    /// Relative Resize Toggle
    /// Represents the following attribute in the schema: o:preferrelative
    #[xml(attr = "o:preferrelative")]
    pub prefer_relative: Option<bool>,
    /// Clip to Wrapping Polygon
    /// Represents the following attribute in the schema: o:cliptowrap
    #[xml(attr = "o:cliptowrap")]
    pub clip_to_wrap: Option<bool>,
    /// Clipping Toggle
    /// Represents the following attribute in the schema: o:clip
    #[xml(attr = "o:clip")]
    pub clip: Option<bool>,
    /// Encoded Package
    /// Represents the following attribute in the schema: o:gfxdata
    #[xml(attr = "o:gfxdata")]
    pub o_gfxdata: Option<String>,
    #[xml(
        child = "v:path",
        child = "v:formulas",
        child = "v:handles",
        child = "v:fill",
        child = "v:stroke",
        child = "v:shadow",
        child = "v:textbox",
        child = "v:textpath",
        child = "v:imagedata",
        child = "o:skew",
        child = "o:extrusion",
        child = "o:callout",
        child = "o:lock",
        child = "o:clippath",
        child = "o:signatureline",
        child = "w10:wrap",
        child = "w10:anchorlock",
        child = "w10:bordertop",
        child = "w10:borderbottom",
        child = "w10:borderleft",
        child = "w10:borderright",
        child = "xvml:ClientData",
        child = "pvml:textdata",
    )]
    pub children: Vec<OvalChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum OvalChildChoice {
    #[xml(tag = "v:path")]
    VPath(Path),
    #[xml(tag = "v:formulas")]
    VFormulas(Formulas),
    #[xml(tag = "v:handles")]
    VHandles(ShapeHandles),
    #[xml(tag = "v:fill")]
    VFill(Fill),
    #[xml(tag = "v:stroke")]
    VStroke(Stroke),
    #[xml(tag = "v:shadow")]
    VShadow(Shadow),
    #[xml(tag = "v:textbox")]
    VTextbox(TextBox),
    #[xml(tag = "v:textpath")]
    VTextpath(TextPath),
    #[xml(tag = "v:imagedata")]
    VImagedata(ImageData),
    #[xml(tag = "o:skew")]
    OSkew(crate::schemas::schemas_microsoft_com_office_office::Skew),
    #[xml(tag = "o:extrusion")]
    OExtrusion(crate::schemas::schemas_microsoft_com_office_office::Extrusion),
    #[xml(tag = "o:callout")]
    OCallout(crate::schemas::schemas_microsoft_com_office_office::Callout),
    #[xml(tag = "o:lock")]
    OLock(crate::schemas::schemas_microsoft_com_office_office::Lock),
    #[xml(tag = "o:clippath")]
    OClippath(crate::schemas::schemas_microsoft_com_office_office::ClipPath),
    #[xml(tag = "o:signatureline")]
    OSignatureline(crate::schemas::schemas_microsoft_com_office_office::SignatureLine),
    #[xml(tag = "w10:wrap")]
    W10Wrap(crate::schemas::schemas_microsoft_com_office_word::TextWrap),
    #[xml(tag = "w10:anchorlock")]
    W10Anchorlock(crate::schemas::schemas_microsoft_com_office_word::AnchorLock),
    #[xml(tag = "w10:bordertop")]
    W10Bordertop(crate::schemas::schemas_microsoft_com_office_word::TopBorder),
    #[xml(tag = "w10:borderbottom")]
    W10Borderbottom(crate::schemas::schemas_microsoft_com_office_word::BottomBorder),
    #[xml(tag = "w10:borderleft")]
    W10Borderleft(crate::schemas::schemas_microsoft_com_office_word::LeftBorder),
    #[xml(tag = "w10:borderright")]
    W10Borderright(crate::schemas::schemas_microsoft_com_office_word::RightBorder),
    #[xml(tag = "xvml:ClientData")]
    XvmlClientData(crate::schemas::schemas_microsoft_com_office_excel::ClientData),
    #[xml(tag = "pvml:textdata")]
    PvmlTextdata(crate::schemas::schemas_microsoft_com_office_powerpoint::TextData),
}
/// Multiple Path Line.
/// When the object is serialized out as xml, it's qualified name is v:polyline.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "v:polyline")]
pub struct PolyLine {
    /// Unique Identifier
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: Option<String>,
    /// Shape Styling Properties
    /// Represents the following attribute in the schema: :style
    #[xml(attr = "style")]
    pub style: Option<String>,
    /// Hyperlink Target
    /// Represents the following attribute in the schema: :href
    #[xml(attr = "href")]
    pub href: Option<String>,
    /// Hyperlink Display Target
    /// Represents the following attribute in the schema: :target
    #[xml(attr = "target")]
    pub target: Option<String>,
    /// CSS Reference
    /// Represents the following attribute in the schema: :class
    #[xml(attr = "class")]
    pub class: Option<String>,
    /// Shape Title
    /// Represents the following attribute in the schema: :title
    #[xml(attr = "title")]
    pub title: Option<String>,
    /// Alternate Text
    /// Represents the following attribute in the schema: :alt
    #[xml(attr = "alt")]
    pub alternate: Option<String>,
    /// Coordinate Space Size
    /// Represents the following attribute in the schema: :coordsize
    #[xml(attr = "coordsize")]
    pub coordinate_size: Option<String>,
    /// Coordinate Space Origin
    /// Represents the following attribute in the schema: :coordorigin
    #[xml(attr = "coordorigin")]
    pub coordinate_origin: Option<String>,
    /// Shape Bounding Polygon
    /// Represents the following attribute in the schema: :wrapcoords
    #[xml(attr = "wrapcoords")]
    pub wrap_coordinates: Option<String>,
    /// Print Toggle
    /// Represents the following attribute in the schema: :print
    #[xml(attr = "print")]
    pub print: Option<bool>,
    /// Optional String
    /// Represents the following attribute in the schema: o:spid
    #[xml(attr = "o:spid")]
    pub optional_string: Option<String>,
    /// Shape Handle Toggle
    /// Represents the following attribute in the schema: o:oned
    #[xml(attr = "o:oned")]
    pub oned: Option<bool>,
    /// Regroup ID
    /// Represents the following attribute in the schema: o:regroupid
    #[xml(attr = "o:regroupid")]
    pub regroup_id: Option<i32>,
    /// Double-click Notification Toggle
    /// Represents the following attribute in the schema: o:doubleclicknotify
    #[xml(attr = "o:doubleclicknotify")]
    pub double_click_notify: Option<bool>,
    /// Button Behavior Toggle
    /// Represents the following attribute in the schema: o:button
    #[xml(attr = "o:button")]
    pub button: Option<bool>,
    /// Hide Script Anchors
    /// Represents the following attribute in the schema: o:userhidden
    #[xml(attr = "o:userhidden")]
    pub user_hidden: Option<bool>,
    /// Graphical Bullet
    /// Represents the following attribute in the schema: o:bullet
    #[xml(attr = "o:bullet")]
    pub bullet: Option<bool>,
    /// Horizontal Rule Toggle
    /// Represents the following attribute in the schema: o:hr
    #[xml(attr = "o:hr")]
    pub horizontal: Option<bool>,
    /// Horizontal Rule Standard Display Toggle
    /// Represents the following attribute in the schema: o:hrstd
    #[xml(attr = "o:hrstd")]
    pub horizontal_standard: Option<bool>,
    /// Horizontal Rule 3D Shading Toggle
    /// Represents the following attribute in the schema: o:hrnoshade
    #[xml(attr = "o:hrnoshade")]
    pub horizontal_no_shade: Option<bool>,
    /// Horizontal Rule Length Percentage
    /// Represents the following attribute in the schema: o:hrpct
    #[xml(attr = "o:hrpct")]
    pub horizontal_percentage: Option<f32>,
    /// Horizontal Rule Alignment
    /// Represents the following attribute in the schema: o:hralign
    #[xml(attr = "o:hralign")]
    pub horizontal_alignment: Option<
        crate::schemas::schemas_microsoft_com_office_office::HorizontalRuleAlignmentValues,
    >,
    /// Allow in Table Cell
    /// Represents the following attribute in the schema: o:allowincell
    #[xml(attr = "o:allowincell")]
    pub allow_in_cell: Option<bool>,
    /// Allow Shape Overlap
    /// Represents the following attribute in the schema: o:allowoverlap
    #[xml(attr = "o:allowoverlap")]
    pub allow_overlap: Option<bool>,
    /// Exists In Master Slide
    /// Represents the following attribute in the schema: o:userdrawn
    #[xml(attr = "o:userdrawn")]
    pub user_drawn: Option<bool>,
    /// Border Top Color
    /// Represents the following attribute in the schema: o:bordertopcolor
    #[xml(attr = "o:bordertopcolor")]
    pub border_top_color: Option<String>,
    /// Border Left Color
    /// Represents the following attribute in the schema: o:borderleftcolor
    #[xml(attr = "o:borderleftcolor")]
    pub border_left_color: Option<String>,
    /// Bottom Border Color
    /// Represents the following attribute in the schema: o:borderbottomcolor
    #[xml(attr = "o:borderbottomcolor")]
    pub border_bottom_color: Option<String>,
    /// Border Right Color
    /// Represents the following attribute in the schema: o:borderrightcolor
    #[xml(attr = "o:borderrightcolor")]
    pub border_right_color: Option<String>,
    /// Diagram Node Layout Identifier
    /// Represents the following attribute in the schema: o:dgmlayout
    #[xml(attr = "o:dgmlayout")]
    pub diagram_layout: Option<i32>,
    /// Diagram Node Identifier
    /// Represents the following attribute in the schema: o:dgmnodekind
    #[xml(attr = "o:dgmnodekind")]
    pub diagram_node_kind: Option<i32>,
    /// Diagram Node Recent Layout Identifier
    /// Represents the following attribute in the schema: o:dgmlayoutmru
    #[xml(attr = "o:dgmlayoutmru")]
    pub diagram_layout_most_recent_used: Option<i32>,
    /// Text Inset Mode
    /// Represents the following attribute in the schema: o:insetmode
    #[xml(attr = "o:insetmode")]
    pub inset_mode: Option<
        crate::schemas::schemas_microsoft_com_office_office::InsetMarginValues,
    >,
    /// Shape Fill Toggle
    /// Represents the following attribute in the schema: :filled
    #[xml(attr = "filled")]
    pub filled: Option<bool>,
    /// Fill Color
    /// Represents the following attribute in the schema: :fillcolor
    #[xml(attr = "fillcolor")]
    pub fill_color: Option<String>,
    /// Shape Stroke Toggle
    /// Represents the following attribute in the schema: :stroked
    #[xml(attr = "stroked")]
    pub stroked: Option<bool>,
    /// Shape Stroke Color
    /// Represents the following attribute in the schema: :strokecolor
    #[xml(attr = "strokecolor")]
    pub stroke_color: Option<String>,
    /// Shape Stroke Weight
    /// Represents the following attribute in the schema: :strokeweight
    #[xml(attr = "strokeweight")]
    pub stroke_weight: Option<String>,
    /// Inset Border From Path
    /// Represents the following attribute in the schema: :insetpen
    #[xml(attr = "insetpen")]
    pub inset_pen: Option<bool>,
    /// Optional Number
    /// Represents the following attribute in the schema: o:spt
    #[xml(attr = "o:spt")]
    pub optional_number: Option<i32>,
    /// Shape Connector Type
    /// Represents the following attribute in the schema: o:connectortype
    #[xml(attr = "o:connectortype")]
    pub connector_type: Option<
        crate::schemas::schemas_microsoft_com_office_office::ConnectorValues,
    >,
    /// Black-and-White Mode
    /// Represents the following attribute in the schema: o:bwmode
    #[xml(attr = "o:bwmode")]
    pub black_white_mode: Option<
        crate::schemas::schemas_microsoft_com_office_office::BlackAndWhiteModeValues,
    >,
    /// Pure Black-and-White Mode
    /// Represents the following attribute in the schema: o:bwpure
    #[xml(attr = "o:bwpure")]
    pub pure_black_white_mode: Option<
        crate::schemas::schemas_microsoft_com_office_office::BlackAndWhiteModeValues,
    >,
    /// Normal Black-and-White Mode
    /// Represents the following attribute in the schema: o:bwnormal
    #[xml(attr = "o:bwnormal")]
    pub normal_black_white_mode: Option<
        crate::schemas::schemas_microsoft_com_office_office::BlackAndWhiteModeValues,
    >,
    /// Force Dashed Outline
    /// Represents the following attribute in the schema: o:forcedash
    #[xml(attr = "o:forcedash")]
    pub force_dash: Option<bool>,
    /// Embedded Object Icon Toggle
    /// Represents the following attribute in the schema: o:oleicon
    #[xml(attr = "o:oleicon")]
    pub ole_icon: Option<bool>,
    /// Embedded Object Toggle
    /// Represents the following attribute in the schema: o:ole
    #[xml(attr = "o:ole")]
    pub ole: Option<bool>,
    /// Relative Resize Toggle
    /// Represents the following attribute in the schema: o:preferrelative
    #[xml(attr = "o:preferrelative")]
    pub prefer_relative: Option<bool>,
    /// Clip to Wrapping Polygon
    /// Represents the following attribute in the schema: o:cliptowrap
    #[xml(attr = "o:cliptowrap")]
    pub clip_to_wrap: Option<bool>,
    /// Clipping Toggle
    /// Represents the following attribute in the schema: o:clip
    #[xml(attr = "o:clip")]
    pub clip: Option<bool>,
    /// Encoded Package
    /// Represents the following attribute in the schema: o:gfxdata
    #[xml(attr = "o:gfxdata")]
    pub o_gfxdata: Option<String>,
    /// Points for Compound Line
    /// Represents the following attribute in the schema: :points
    #[xml(attr = "points")]
    pub points: Option<String>,
    #[xml(
        child = "v:path",
        child = "v:formulas",
        child = "v:handles",
        child = "v:fill",
        child = "v:stroke",
        child = "v:shadow",
        child = "v:textbox",
        child = "v:textpath",
        child = "v:imagedata",
        child = "o:skew",
        child = "o:extrusion",
        child = "o:callout",
        child = "o:lock",
        child = "o:clippath",
        child = "o:signatureline",
        child = "w10:wrap",
        child = "w10:anchorlock",
        child = "w10:bordertop",
        child = "w10:borderbottom",
        child = "w10:borderleft",
        child = "w10:borderright",
        child = "xvml:ClientData",
        child = "pvml:textdata",
        child = "o:ink",
    )]
    pub children: Vec<PolyLineChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum PolyLineChildChoice {
    #[xml(tag = "v:path")]
    VPath(Path),
    #[xml(tag = "v:formulas")]
    VFormulas(Formulas),
    #[xml(tag = "v:handles")]
    VHandles(ShapeHandles),
    #[xml(tag = "v:fill")]
    VFill(Fill),
    #[xml(tag = "v:stroke")]
    VStroke(Stroke),
    #[xml(tag = "v:shadow")]
    VShadow(Shadow),
    #[xml(tag = "v:textbox")]
    VTextbox(TextBox),
    #[xml(tag = "v:textpath")]
    VTextpath(TextPath),
    #[xml(tag = "v:imagedata")]
    VImagedata(ImageData),
    #[xml(tag = "o:skew")]
    OSkew(crate::schemas::schemas_microsoft_com_office_office::Skew),
    #[xml(tag = "o:extrusion")]
    OExtrusion(crate::schemas::schemas_microsoft_com_office_office::Extrusion),
    #[xml(tag = "o:callout")]
    OCallout(crate::schemas::schemas_microsoft_com_office_office::Callout),
    #[xml(tag = "o:lock")]
    OLock(crate::schemas::schemas_microsoft_com_office_office::Lock),
    #[xml(tag = "o:clippath")]
    OClippath(crate::schemas::schemas_microsoft_com_office_office::ClipPath),
    #[xml(tag = "o:signatureline")]
    OSignatureline(crate::schemas::schemas_microsoft_com_office_office::SignatureLine),
    #[xml(tag = "w10:wrap")]
    W10Wrap(crate::schemas::schemas_microsoft_com_office_word::TextWrap),
    #[xml(tag = "w10:anchorlock")]
    W10Anchorlock(crate::schemas::schemas_microsoft_com_office_word::AnchorLock),
    #[xml(tag = "w10:bordertop")]
    W10Bordertop(crate::schemas::schemas_microsoft_com_office_word::TopBorder),
    #[xml(tag = "w10:borderbottom")]
    W10Borderbottom(crate::schemas::schemas_microsoft_com_office_word::BottomBorder),
    #[xml(tag = "w10:borderleft")]
    W10Borderleft(crate::schemas::schemas_microsoft_com_office_word::LeftBorder),
    #[xml(tag = "w10:borderright")]
    W10Borderright(crate::schemas::schemas_microsoft_com_office_word::RightBorder),
    #[xml(tag = "xvml:ClientData")]
    XvmlClientData(crate::schemas::schemas_microsoft_com_office_excel::ClientData),
    #[xml(tag = "pvml:textdata")]
    PvmlTextdata(crate::schemas::schemas_microsoft_com_office_powerpoint::TextData),
    #[xml(tag = "o:ink")]
    OInk(crate::schemas::schemas_microsoft_com_office_office::Ink),
}
/// Rectangle.
/// When the object is serialized out as xml, it's qualified name is v:rect.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "v:rect")]
pub struct Rectangle {
    /// Unique Identifier
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: Option<String>,
    /// Shape Styling Properties
    /// Represents the following attribute in the schema: :style
    #[xml(attr = "style")]
    pub style: Option<String>,
    /// Hyperlink Target
    /// Represents the following attribute in the schema: :href
    #[xml(attr = "href")]
    pub href: Option<String>,
    /// Hyperlink Display Target
    /// Represents the following attribute in the schema: :target
    #[xml(attr = "target")]
    pub target: Option<String>,
    /// CSS Reference
    /// Represents the following attribute in the schema: :class
    #[xml(attr = "class")]
    pub class: Option<String>,
    /// Shape Title
    /// Represents the following attribute in the schema: :title
    #[xml(attr = "title")]
    pub title: Option<String>,
    /// Alternate Text
    /// Represents the following attribute in the schema: :alt
    #[xml(attr = "alt")]
    pub alternate: Option<String>,
    /// Coordinate Space Size
    /// Represents the following attribute in the schema: :coordsize
    #[xml(attr = "coordsize")]
    pub coordinate_size: Option<String>,
    /// Coordinate Space Origin
    /// Represents the following attribute in the schema: :coordorigin
    #[xml(attr = "coordorigin")]
    pub coordinate_origin: Option<String>,
    /// Shape Bounding Polygon
    /// Represents the following attribute in the schema: :wrapcoords
    #[xml(attr = "wrapcoords")]
    pub wrap_coordinates: Option<String>,
    /// Print Toggle
    /// Represents the following attribute in the schema: :print
    #[xml(attr = "print")]
    pub print: Option<bool>,
    /// Optional String
    /// Represents the following attribute in the schema: o:spid
    #[xml(attr = "o:spid")]
    pub optional_string: Option<String>,
    /// Shape Handle Toggle
    /// Represents the following attribute in the schema: o:oned
    #[xml(attr = "o:oned")]
    pub oned: Option<bool>,
    /// Regroup ID
    /// Represents the following attribute in the schema: o:regroupid
    #[xml(attr = "o:regroupid")]
    pub regroup_id: Option<i32>,
    /// Double-click Notification Toggle
    /// Represents the following attribute in the schema: o:doubleclicknotify
    #[xml(attr = "o:doubleclicknotify")]
    pub double_click_notify: Option<bool>,
    /// Button Behavior Toggle
    /// Represents the following attribute in the schema: o:button
    #[xml(attr = "o:button")]
    pub button: Option<bool>,
    /// Hide Script Anchors
    /// Represents the following attribute in the schema: o:userhidden
    #[xml(attr = "o:userhidden")]
    pub user_hidden: Option<bool>,
    /// Graphical Bullet
    /// Represents the following attribute in the schema: o:bullet
    #[xml(attr = "o:bullet")]
    pub bullet: Option<bool>,
    /// Horizontal Rule Toggle
    /// Represents the following attribute in the schema: o:hr
    #[xml(attr = "o:hr")]
    pub horizontal: Option<bool>,
    /// Horizontal Rule Standard Display Toggle
    /// Represents the following attribute in the schema: o:hrstd
    #[xml(attr = "o:hrstd")]
    pub horizontal_standard: Option<bool>,
    /// Horizontal Rule 3D Shading Toggle
    /// Represents the following attribute in the schema: o:hrnoshade
    #[xml(attr = "o:hrnoshade")]
    pub horizontal_no_shade: Option<bool>,
    /// Horizontal Rule Length Percentage
    /// Represents the following attribute in the schema: o:hrpct
    #[xml(attr = "o:hrpct")]
    pub horizontal_percentage: Option<f32>,
    /// Horizontal Rule Alignment
    /// Represents the following attribute in the schema: o:hralign
    #[xml(attr = "o:hralign")]
    pub horizontal_alignment: Option<
        crate::schemas::schemas_microsoft_com_office_office::HorizontalRuleAlignmentValues,
    >,
    /// Allow in Table Cell
    /// Represents the following attribute in the schema: o:allowincell
    #[xml(attr = "o:allowincell")]
    pub allow_in_cell: Option<bool>,
    /// Allow Shape Overlap
    /// Represents the following attribute in the schema: o:allowoverlap
    #[xml(attr = "o:allowoverlap")]
    pub allow_overlap: Option<bool>,
    /// Exists In Master Slide
    /// Represents the following attribute in the schema: o:userdrawn
    #[xml(attr = "o:userdrawn")]
    pub user_drawn: Option<bool>,
    /// Border Top Color
    /// Represents the following attribute in the schema: o:bordertopcolor
    #[xml(attr = "o:bordertopcolor")]
    pub border_top_color: Option<String>,
    /// Border Left Color
    /// Represents the following attribute in the schema: o:borderleftcolor
    #[xml(attr = "o:borderleftcolor")]
    pub border_left_color: Option<String>,
    /// Bottom Border Color
    /// Represents the following attribute in the schema: o:borderbottomcolor
    #[xml(attr = "o:borderbottomcolor")]
    pub border_bottom_color: Option<String>,
    /// Border Right Color
    /// Represents the following attribute in the schema: o:borderrightcolor
    #[xml(attr = "o:borderrightcolor")]
    pub border_right_color: Option<String>,
    /// Diagram Node Layout Identifier
    /// Represents the following attribute in the schema: o:dgmlayout
    #[xml(attr = "o:dgmlayout")]
    pub diagram_layout: Option<i32>,
    /// Diagram Node Identifier
    /// Represents the following attribute in the schema: o:dgmnodekind
    #[xml(attr = "o:dgmnodekind")]
    pub diagram_node_kind: Option<i32>,
    /// Diagram Node Recent Layout Identifier
    /// Represents the following attribute in the schema: o:dgmlayoutmru
    #[xml(attr = "o:dgmlayoutmru")]
    pub diagram_layout_most_recent_used: Option<i32>,
    /// Text Inset Mode
    /// Represents the following attribute in the schema: o:insetmode
    #[xml(attr = "o:insetmode")]
    pub inset_mode: Option<
        crate::schemas::schemas_microsoft_com_office_office::InsetMarginValues,
    >,
    /// Shape Fill Toggle
    /// Represents the following attribute in the schema: :filled
    #[xml(attr = "filled")]
    pub filled: Option<bool>,
    /// Fill Color
    /// Represents the following attribute in the schema: :fillcolor
    #[xml(attr = "fillcolor")]
    pub fill_color: Option<String>,
    /// Shape Stroke Toggle
    /// Represents the following attribute in the schema: :stroked
    #[xml(attr = "stroked")]
    pub stroked: Option<bool>,
    /// Shape Stroke Color
    /// Represents the following attribute in the schema: :strokecolor
    #[xml(attr = "strokecolor")]
    pub stroke_color: Option<String>,
    /// Shape Stroke Weight
    /// Represents the following attribute in the schema: :strokeweight
    #[xml(attr = "strokeweight")]
    pub stroke_weight: Option<String>,
    /// Inset Border From Path
    /// Represents the following attribute in the schema: :insetpen
    #[xml(attr = "insetpen")]
    pub inset_pen: Option<bool>,
    /// Optional Number
    /// Represents the following attribute in the schema: o:spt
    #[xml(attr = "o:spt")]
    pub optional_number: Option<i32>,
    /// Shape Connector Type
    /// Represents the following attribute in the schema: o:connectortype
    #[xml(attr = "o:connectortype")]
    pub connector_type: Option<
        crate::schemas::schemas_microsoft_com_office_office::ConnectorValues,
    >,
    /// Black-and-White Mode
    /// Represents the following attribute in the schema: o:bwmode
    #[xml(attr = "o:bwmode")]
    pub black_white_mode: Option<
        crate::schemas::schemas_microsoft_com_office_office::BlackAndWhiteModeValues,
    >,
    /// Pure Black-and-White Mode
    /// Represents the following attribute in the schema: o:bwpure
    #[xml(attr = "o:bwpure")]
    pub pure_black_white_mode: Option<
        crate::schemas::schemas_microsoft_com_office_office::BlackAndWhiteModeValues,
    >,
    /// Normal Black-and-White Mode
    /// Represents the following attribute in the schema: o:bwnormal
    #[xml(attr = "o:bwnormal")]
    pub normal_black_white_mode: Option<
        crate::schemas::schemas_microsoft_com_office_office::BlackAndWhiteModeValues,
    >,
    /// Force Dashed Outline
    /// Represents the following attribute in the schema: o:forcedash
    #[xml(attr = "o:forcedash")]
    pub force_dash: Option<bool>,
    /// Embedded Object Icon Toggle
    /// Represents the following attribute in the schema: o:oleicon
    #[xml(attr = "o:oleicon")]
    pub ole_icon: Option<bool>,
    /// Embedded Object Toggle
    /// Represents the following attribute in the schema: o:ole
    #[xml(attr = "o:ole")]
    pub ole: Option<bool>,
    /// Relative Resize Toggle
    /// Represents the following attribute in the schema: o:preferrelative
    #[xml(attr = "o:preferrelative")]
    pub prefer_relative: Option<bool>,
    /// Clip to Wrapping Polygon
    /// Represents the following attribute in the schema: o:cliptowrap
    #[xml(attr = "o:cliptowrap")]
    pub clip_to_wrap: Option<bool>,
    /// Clipping Toggle
    /// Represents the following attribute in the schema: o:clip
    #[xml(attr = "o:clip")]
    pub clip: Option<bool>,
    /// Encoded Package
    /// Represents the following attribute in the schema: o:gfxdata
    #[xml(attr = "o:gfxdata")]
    pub o_gfxdata: Option<String>,
    #[xml(
        child = "v:path",
        child = "v:formulas",
        child = "v:handles",
        child = "v:fill",
        child = "v:stroke",
        child = "v:shadow",
        child = "v:textbox",
        child = "v:textpath",
        child = "v:imagedata",
        child = "o:skew",
        child = "o:extrusion",
        child = "o:callout",
        child = "o:lock",
        child = "o:clippath",
        child = "o:signatureline",
        child = "w10:wrap",
        child = "w10:anchorlock",
        child = "w10:bordertop",
        child = "w10:borderbottom",
        child = "w10:borderleft",
        child = "w10:borderright",
        child = "xvml:ClientData",
        child = "pvml:textdata",
    )]
    pub children: Vec<RectangleChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum RectangleChildChoice {
    #[xml(tag = "v:path")]
    VPath(Path),
    #[xml(tag = "v:formulas")]
    VFormulas(Formulas),
    #[xml(tag = "v:handles")]
    VHandles(ShapeHandles),
    #[xml(tag = "v:fill")]
    VFill(Fill),
    #[xml(tag = "v:stroke")]
    VStroke(Stroke),
    #[xml(tag = "v:shadow")]
    VShadow(Shadow),
    #[xml(tag = "v:textbox")]
    VTextbox(TextBox),
    #[xml(tag = "v:textpath")]
    VTextpath(TextPath),
    #[xml(tag = "v:imagedata")]
    VImagedata(ImageData),
    #[xml(tag = "o:skew")]
    OSkew(crate::schemas::schemas_microsoft_com_office_office::Skew),
    #[xml(tag = "o:extrusion")]
    OExtrusion(crate::schemas::schemas_microsoft_com_office_office::Extrusion),
    #[xml(tag = "o:callout")]
    OCallout(crate::schemas::schemas_microsoft_com_office_office::Callout),
    #[xml(tag = "o:lock")]
    OLock(crate::schemas::schemas_microsoft_com_office_office::Lock),
    #[xml(tag = "o:clippath")]
    OClippath(crate::schemas::schemas_microsoft_com_office_office::ClipPath),
    #[xml(tag = "o:signatureline")]
    OSignatureline(crate::schemas::schemas_microsoft_com_office_office::SignatureLine),
    #[xml(tag = "w10:wrap")]
    W10Wrap(crate::schemas::schemas_microsoft_com_office_word::TextWrap),
    #[xml(tag = "w10:anchorlock")]
    W10Anchorlock(crate::schemas::schemas_microsoft_com_office_word::AnchorLock),
    #[xml(tag = "w10:bordertop")]
    W10Bordertop(crate::schemas::schemas_microsoft_com_office_word::TopBorder),
    #[xml(tag = "w10:borderbottom")]
    W10Borderbottom(crate::schemas::schemas_microsoft_com_office_word::BottomBorder),
    #[xml(tag = "w10:borderleft")]
    W10Borderleft(crate::schemas::schemas_microsoft_com_office_word::LeftBorder),
    #[xml(tag = "w10:borderright")]
    W10Borderright(crate::schemas::schemas_microsoft_com_office_word::RightBorder),
    #[xml(tag = "xvml:ClientData")]
    XvmlClientData(crate::schemas::schemas_microsoft_com_office_excel::ClientData),
    #[xml(tag = "pvml:textdata")]
    PvmlTextdata(crate::schemas::schemas_microsoft_com_office_powerpoint::TextData),
}
/// Rounded Rectangle.
/// When the object is serialized out as xml, it's qualified name is v:roundrect.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "v:roundrect")]
pub struct RoundRectangle {
    /// Unique Identifier
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: Option<String>,
    /// style
    /// Represents the following attribute in the schema: :style
    #[xml(attr = "style")]
    pub style: Option<String>,
    /// href
    /// Represents the following attribute in the schema: :href
    #[xml(attr = "href")]
    pub href: Option<String>,
    /// target
    /// Represents the following attribute in the schema: :target
    #[xml(attr = "target")]
    pub target: Option<String>,
    /// class
    /// Represents the following attribute in the schema: :class
    #[xml(attr = "class")]
    pub class: Option<String>,
    /// title
    /// Represents the following attribute in the schema: :title
    #[xml(attr = "title")]
    pub title: Option<String>,
    /// alt
    /// Represents the following attribute in the schema: :alt
    #[xml(attr = "alt")]
    pub alternate: Option<String>,
    /// coordsize
    /// Represents the following attribute in the schema: :coordsize
    #[xml(attr = "coordsize")]
    pub coordinate_size: Option<String>,
    /// wrapcoords
    /// Represents the following attribute in the schema: :wrapcoords
    #[xml(attr = "wrapcoords")]
    pub wrap_coordinates: Option<String>,
    /// print
    /// Represents the following attribute in the schema: :print
    #[xml(attr = "print")]
    pub print: Option<bool>,
    /// Optional String
    /// Represents the following attribute in the schema: o:spid
    #[xml(attr = "o:spid")]
    pub optional_string: Option<String>,
    /// Shape Handle Toggle
    /// Represents the following attribute in the schema: o:oned
    #[xml(attr = "o:oned")]
    pub oned: Option<bool>,
    /// Regroup ID
    /// Represents the following attribute in the schema: o:regroupid
    #[xml(attr = "o:regroupid")]
    pub regroup_id: Option<i32>,
    /// Double-click Notification Toggle
    /// Represents the following attribute in the schema: o:doubleclicknotify
    #[xml(attr = "o:doubleclicknotify")]
    pub double_click_notify: Option<bool>,
    /// Button Behavior Toggle
    /// Represents the following attribute in the schema: o:button
    #[xml(attr = "o:button")]
    pub button: Option<bool>,
    /// Hide Script Anchors
    /// Represents the following attribute in the schema: o:userhidden
    #[xml(attr = "o:userhidden")]
    pub user_hidden: Option<bool>,
    /// Graphical Bullet
    /// Represents the following attribute in the schema: o:bullet
    #[xml(attr = "o:bullet")]
    pub bullet: Option<bool>,
    /// Horizontal Rule Toggle
    /// Represents the following attribute in the schema: o:hr
    #[xml(attr = "o:hr")]
    pub horizontal: Option<bool>,
    /// Horizontal Rule Standard Display Toggle
    /// Represents the following attribute in the schema: o:hrstd
    #[xml(attr = "o:hrstd")]
    pub horizontal_standard: Option<bool>,
    /// Horizontal Rule 3D Shading Toggle
    /// Represents the following attribute in the schema: o:hrnoshade
    #[xml(attr = "o:hrnoshade")]
    pub horizontal_no_shade: Option<bool>,
    /// Horizontal Rule Length Percentage
    /// Represents the following attribute in the schema: o:hrpct
    #[xml(attr = "o:hrpct")]
    pub horizontal_percentage: Option<f32>,
    /// Horizontal Rule Alignment
    /// Represents the following attribute in the schema: o:hralign
    #[xml(attr = "o:hralign")]
    pub horizontal_alignment: Option<
        crate::schemas::schemas_microsoft_com_office_office::HorizontalRuleAlignmentValues,
    >,
    /// Allow in Table Cell
    /// Represents the following attribute in the schema: o:allowincell
    #[xml(attr = "o:allowincell")]
    pub allow_in_cell: Option<bool>,
    /// Allow Shape Overlap
    /// Represents the following attribute in the schema: o:allowoverlap
    #[xml(attr = "o:allowoverlap")]
    pub allow_overlap: Option<bool>,
    /// Exists In Master Slide
    /// Represents the following attribute in the schema: o:userdrawn
    #[xml(attr = "o:userdrawn")]
    pub user_drawn: Option<bool>,
    /// Border Top Color
    /// Represents the following attribute in the schema: o:bordertopcolor
    #[xml(attr = "o:bordertopcolor")]
    pub border_top_color: Option<String>,
    /// Border Left Color
    /// Represents the following attribute in the schema: o:borderleftcolor
    #[xml(attr = "o:borderleftcolor")]
    pub border_left_color: Option<String>,
    /// Bottom Border Color
    /// Represents the following attribute in the schema: o:borderbottomcolor
    #[xml(attr = "o:borderbottomcolor")]
    pub border_bottom_color: Option<String>,
    /// Border Right Color
    /// Represents the following attribute in the schema: o:borderrightcolor
    #[xml(attr = "o:borderrightcolor")]
    pub border_right_color: Option<String>,
    /// Diagram Node Layout Identifier
    /// Represents the following attribute in the schema: o:dgmlayout
    #[xml(attr = "o:dgmlayout")]
    pub diagram_layout: Option<i32>,
    /// Diagram Node Identifier
    /// Represents the following attribute in the schema: o:dgmnodekind
    #[xml(attr = "o:dgmnodekind")]
    pub diagram_node_kind: Option<i32>,
    /// Diagram Node Recent Layout Identifier
    /// Represents the following attribute in the schema: o:dgmlayoutmru
    #[xml(attr = "o:dgmlayoutmru")]
    pub diagram_layout_most_recent_used: Option<i32>,
    /// Text Inset Mode
    /// Represents the following attribute in the schema: o:insetmode
    #[xml(attr = "o:insetmode")]
    pub inset_mode: Option<
        crate::schemas::schemas_microsoft_com_office_office::InsetMarginValues,
    >,
    /// Shape Fill Toggle
    /// Represents the following attribute in the schema: :filled
    #[xml(attr = "filled")]
    pub filled: Option<bool>,
    /// Fill Color
    /// Represents the following attribute in the schema: :fillcolor
    #[xml(attr = "fillcolor")]
    pub fill_color: Option<String>,
    /// Shape Stroke Toggle
    /// Represents the following attribute in the schema: :stroked
    #[xml(attr = "stroked")]
    pub stroked: Option<bool>,
    /// Shape Stroke Color
    /// Represents the following attribute in the schema: :strokecolor
    #[xml(attr = "strokecolor")]
    pub stroke_color: Option<String>,
    /// Shape Stroke Weight
    /// Represents the following attribute in the schema: :strokeweight
    #[xml(attr = "strokeweight")]
    pub stroke_weight: Option<String>,
    /// Inset Border From Path
    /// Represents the following attribute in the schema: :insetpen
    #[xml(attr = "insetpen")]
    pub inset_pen: Option<bool>,
    /// Optional Number
    /// Represents the following attribute in the schema: o:spt
    #[xml(attr = "o:spt")]
    pub optional_number: Option<i32>,
    /// Shape Connector Type
    /// Represents the following attribute in the schema: o:connectortype
    #[xml(attr = "o:connectortype")]
    pub connector_type: Option<
        crate::schemas::schemas_microsoft_com_office_office::ConnectorValues,
    >,
    /// Black-and-White Mode
    /// Represents the following attribute in the schema: o:bwmode
    #[xml(attr = "o:bwmode")]
    pub black_white_mode: Option<
        crate::schemas::schemas_microsoft_com_office_office::BlackAndWhiteModeValues,
    >,
    /// Pure Black-and-White Mode
    /// Represents the following attribute in the schema: o:bwpure
    #[xml(attr = "o:bwpure")]
    pub pure_black_white_mode: Option<
        crate::schemas::schemas_microsoft_com_office_office::BlackAndWhiteModeValues,
    >,
    /// Normal Black-and-White Mode
    /// Represents the following attribute in the schema: o:bwnormal
    #[xml(attr = "o:bwnormal")]
    pub normal_black_white_mode: Option<
        crate::schemas::schemas_microsoft_com_office_office::BlackAndWhiteModeValues,
    >,
    /// Force Dashed Outline
    /// Represents the following attribute in the schema: o:forcedash
    #[xml(attr = "o:forcedash")]
    pub force_dash: Option<bool>,
    /// Embedded Object Icon Toggle
    /// Represents the following attribute in the schema: o:oleicon
    #[xml(attr = "o:oleicon")]
    pub ole_icon: Option<bool>,
    /// Embedded Object Toggle
    /// Represents the following attribute in the schema: o:ole
    #[xml(attr = "o:ole")]
    pub ole: Option<bool>,
    /// Relative Resize Toggle
    /// Represents the following attribute in the schema: o:preferrelative
    #[xml(attr = "o:preferrelative")]
    pub prefer_relative: Option<bool>,
    /// Clip to Wrapping Polygon
    /// Represents the following attribute in the schema: o:cliptowrap
    #[xml(attr = "o:cliptowrap")]
    pub clip_to_wrap: Option<bool>,
    /// Clipping Toggle
    /// Represents the following attribute in the schema: o:clip
    #[xml(attr = "o:clip")]
    pub clip: Option<bool>,
    /// Encoded Package
    /// Represents the following attribute in the schema: o:gfxdata
    #[xml(attr = "o:gfxdata")]
    pub o_gfxdata: Option<String>,
    /// Rounded Corner Arc Size
    /// Represents the following attribute in the schema: :arcsize
    #[xml(attr = "arcsize")]
    pub arc_size: Option<String>,
    #[xml(
        child = "v:path",
        child = "v:formulas",
        child = "v:handles",
        child = "v:fill",
        child = "v:stroke",
        child = "v:shadow",
        child = "v:textbox",
        child = "v:textpath",
        child = "v:imagedata",
        child = "o:skew",
        child = "o:extrusion",
        child = "o:callout",
        child = "o:lock",
        child = "o:clippath",
        child = "o:signatureline",
        child = "w10:wrap",
        child = "w10:anchorlock",
        child = "w10:bordertop",
        child = "w10:borderbottom",
        child = "w10:borderleft",
        child = "w10:borderright",
        child = "xvml:ClientData",
        child = "pvml:textdata",
    )]
    pub children: Vec<RoundRectangleChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum RoundRectangleChildChoice {
    #[xml(tag = "v:path")]
    VPath(Path),
    #[xml(tag = "v:formulas")]
    VFormulas(Formulas),
    #[xml(tag = "v:handles")]
    VHandles(ShapeHandles),
    #[xml(tag = "v:fill")]
    VFill(Fill),
    #[xml(tag = "v:stroke")]
    VStroke(Stroke),
    #[xml(tag = "v:shadow")]
    VShadow(Shadow),
    #[xml(tag = "v:textbox")]
    VTextbox(TextBox),
    #[xml(tag = "v:textpath")]
    VTextpath(TextPath),
    #[xml(tag = "v:imagedata")]
    VImagedata(ImageData),
    #[xml(tag = "o:skew")]
    OSkew(crate::schemas::schemas_microsoft_com_office_office::Skew),
    #[xml(tag = "o:extrusion")]
    OExtrusion(crate::schemas::schemas_microsoft_com_office_office::Extrusion),
    #[xml(tag = "o:callout")]
    OCallout(crate::schemas::schemas_microsoft_com_office_office::Callout),
    #[xml(tag = "o:lock")]
    OLock(crate::schemas::schemas_microsoft_com_office_office::Lock),
    #[xml(tag = "o:clippath")]
    OClippath(crate::schemas::schemas_microsoft_com_office_office::ClipPath),
    #[xml(tag = "o:signatureline")]
    OSignatureline(crate::schemas::schemas_microsoft_com_office_office::SignatureLine),
    #[xml(tag = "w10:wrap")]
    W10Wrap(crate::schemas::schemas_microsoft_com_office_word::TextWrap),
    #[xml(tag = "w10:anchorlock")]
    W10Anchorlock(crate::schemas::schemas_microsoft_com_office_word::AnchorLock),
    #[xml(tag = "w10:bordertop")]
    W10Bordertop(crate::schemas::schemas_microsoft_com_office_word::TopBorder),
    #[xml(tag = "w10:borderbottom")]
    W10Borderbottom(crate::schemas::schemas_microsoft_com_office_word::BottomBorder),
    #[xml(tag = "w10:borderleft")]
    W10Borderleft(crate::schemas::schemas_microsoft_com_office_word::LeftBorder),
    #[xml(tag = "w10:borderright")]
    W10Borderright(crate::schemas::schemas_microsoft_com_office_word::RightBorder),
    #[xml(tag = "xvml:ClientData")]
    XvmlClientData(crate::schemas::schemas_microsoft_com_office_excel::ClientData),
    #[xml(tag = "pvml:textdata")]
    PvmlTextdata(crate::schemas::schemas_microsoft_com_office_powerpoint::TextData),
}
/// Shape Handle.
/// When the object is serialized out as xml, it's qualified name is v:h.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "v:h")]
pub struct ShapeHandle {
    /// Handle Position
    /// Represents the following attribute in the schema: :position
    #[xml(attr = "position")]
    pub position: Option<String>,
    /// Handle Polar Center
    /// Represents the following attribute in the schema: :polar
    #[xml(attr = "polar")]
    pub polar: Option<String>,
    /// Handle Coordinate Mapping
    /// Represents the following attribute in the schema: :map
    #[xml(attr = "map")]
    pub map: Option<String>,
    /// Invert Handle's X Position
    /// Represents the following attribute in the schema: :invx
    #[xml(attr = "invx")]
    pub invert_x: Option<bool>,
    /// Invert Handle's Y Position
    /// Represents the following attribute in the schema: :invy
    #[xml(attr = "invy")]
    pub invert_y: Option<bool>,
    /// Handle Inversion Toggle
    /// Represents the following attribute in the schema: :switch
    #[xml(attr = "switch")]
    pub switch: Option<bool>,
    /// Handle X Position Range
    /// Represents the following attribute in the schema: :xrange
    #[xml(attr = "xrange")]
    pub x_range: Option<String>,
    /// Handle Y Position Range
    /// Represents the following attribute in the schema: :yrange
    #[xml(attr = "yrange")]
    pub y_range: Option<String>,
    /// Handle Polar Radius Range
    /// Represents the following attribute in the schema: :radiusrange
    #[xml(attr = "radiusrange")]
    pub radius_range: Option<String>,
}
/// Single Formula.
/// When the object is serialized out as xml, it's qualified name is v:f.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "v:f")]
pub struct Formula {
    /// Equation
    /// Represents the following attribute in the schema: :eqn
    #[xml(attr = "eqn")]
    pub equation: Option<String>,
}
