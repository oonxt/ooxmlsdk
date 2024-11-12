#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum AlignmentValues {
    #[default]
    Top,
    Middle,
    Bottom,
    Left,
    Center,
    Right,
}
crate::__string_enum! {
    AlignmentValues { Top = "top", Middle = "middle", Bottom = "bottom", Left = "left",
    Center = "center", Right = "right", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum ScreenSizeValues {
    #[default]
    Sz544x376,
    Sz640x480,
    Sz720x512,
    Sz800x600,
    Sz1024x768,
    Sz1152x862,
}
crate::__string_enum! {
    ScreenSizeValues { Sz544x376 = "544376", Sz640x480 = "640480", Sz720x512 = "720512",
    Sz800x600 = "800600", Sz1024x768 = "1024768", Sz1152x862 = "1152862", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum InsetMarginValues {
    #[default]
    Auto,
    Custom,
}
crate::__string_enum! {
    InsetMarginValues { Auto = "auto", Custom = "custom", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum ColorModeValues {
    #[default]
    Auto,
    Custom,
}
crate::__string_enum! {
    ColorModeValues { Auto = "auto", Custom = "custom", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum ExtrusionValues {
    #[default]
    Perspective,
    Parallel,
}
crate::__string_enum! {
    ExtrusionValues { Perspective = "perspective", Parallel = "parallel", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum ExtrusionRenderValues {
    #[default]
    Solid,
    WireFrame,
    BoundingCube,
}
crate::__string_enum! {
    ExtrusionRenderValues { Solid = "solid", WireFrame = "wireFrame", BoundingCube =
    "boundingCube", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum ExtrusionPlaneValues {
    #[default]
    Xy,
    Zx,
    Yz,
}
crate::__string_enum! {
    ExtrusionPlaneValues { Xy = "xy", Zx = "zx", Yz = "yz", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum AngleValues {
    #[default]
    Any,
    Degree30,
    Degree45,
    Degree60,
    Degree90,
    Auto,
}
crate::__string_enum! {
    AngleValues { Any = "any", Degree30 = "30", Degree45 = "45", Degree60 = "60",
    Degree90 = "90", Auto = "auto", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum CalloutPlacementValues {
    #[default]
    Top,
    Center,
    Bottom,
    User,
}
crate::__string_enum! {
    CalloutPlacementValues { Top = "top", Center = "center", Bottom = "bottom", User =
    "user", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum ConnectorValues {
    #[default]
    None,
    Straight,
    Elbow,
    Curved,
}
crate::__string_enum! {
    ConnectorValues { None = "none", Straight = "straight", Elbow = "elbow", Curved =
    "curved", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum HorizontalRuleAlignmentValues {
    #[default]
    Left,
    Right,
    Center,
}
crate::__string_enum! {
    HorizontalRuleAlignmentValues { Left = "left", Right = "right", Center = "center", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum ConnectValues {
    #[default]
    None,
    Rectangle,
    Segments,
    Custom,
}
crate::__string_enum! {
    ConnectValues { None = "none", Rectangle = "rect", Segments = "segments", Custom =
    "custom", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum OleLinkValues {
    #[default]
    Picture,
    Bitmap,
    EnhancedMetaFile,
}
crate::__string_enum! {
    OleLinkValues { Picture = "picture", Bitmap = "bitmap", EnhancedMetaFile =
    "enhancedMetaFile", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum OleValues {
    #[default]
    Embed,
    Link,
}
crate::__string_enum! {
    OleValues { Embed = "embed", Link = "link", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum OleDrawAspectValues {
    #[default]
    Content,
    Icon,
}
crate::__string_enum! {
    OleDrawAspectValues { Content = "content", Icon = "icon", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum OleUpdateModeValues {
    #[default]
    Always,
    OnCall,
}
crate::__string_enum! {
    OleUpdateModeValues { Always = "always", OnCall = "onCall", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum FillValues {
    #[default]
    GradientCenter,
    Solid,
    Pattern,
    Tile,
    Frame,
    GradientUnscaled,
    GradientRadial,
    Gradient,
    Background,
}
crate::__string_enum! {
    FillValues { GradientCenter = "gradientCenter", Solid = "solid", Pattern = "pattern",
    Tile = "tile", Frame = "frame", GradientUnscaled = "gradientUnscaled", GradientRadial
    = "gradientRadial", Gradient = "gradient", Background = "background", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum RuleValues {
    #[default]
    Arc,
    Callout,
    Connector,
}
crate::__string_enum! {
    RuleValues { Arc = "arc", Callout = "callout", Connector = "connector", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum BlackAndWhiteModeValues {
    #[default]
    Color,
    Auto,
    GrayScale,
    LightGrayScale,
    InverseGray,
    GrayOutline,
    HighContrast,
    Black,
    White,
    Undrawn,
    BlackTextAndLines,
}
crate::__string_enum! {
    BlackAndWhiteModeValues { Color = "color", Auto = "auto", GrayScale = "grayScale",
    LightGrayScale = "lightGrayScale", InverseGray = "inverseGray", GrayOutline =
    "grayOutline", HighContrast = "highContrast", Black = "black", White = "white",
    Undrawn = "undrawn", BlackTextAndLines = "blackTextAndLines", }
}
/// New Shape Defaults.
/// When the object is serialized out as xml, it's qualified name is o:shapedefaults.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "o:shapedefaults")]
pub struct ShapeDefaults {
    /// VML Extension Handling Behavior
    /// Represents the following attribute in the schema: v:ext
    #[xml(attr = "v:ext")]
    pub extension: Option<
        crate::schemas::schemas_microsoft_com_vml::ExtensionHandlingBehaviorValues,
    >,
    /// Shape ID Optional Storage
    /// Represents the following attribute in the schema: :spidmax
    #[xml(attr = "spidmax")]
    pub max_shape_id: Option<i32>,
    /// style
    /// Represents the following attribute in the schema: :style
    #[xml(attr = "style")]
    pub style: Option<String>,
    /// Shape Fill Toggle
    /// Represents the following attribute in the schema: :fill
    #[xml(attr = "fill")]
    pub be_filled: Option<bool>,
    /// Default Fill Color
    /// Represents the following attribute in the schema: :fillcolor
    #[xml(attr = "fillcolor")]
    pub fill_color: Option<String>,
    /// Shape Stroke Toggle
    /// Represents the following attribute in the schema: :stroke
    #[xml(attr = "stroke")]
    pub is_stroke: Option<bool>,
    /// Shape Stroke Color
    /// Represents the following attribute in the schema: :strokecolor
    #[xml(attr = "strokecolor")]
    pub stroke_color: Option<String>,
    /// Allow in Table Cell
    /// Represents the following attribute in the schema: o:allowincell
    #[xml(attr = "o:allowincell")]
    pub allow_in_cell: Option<bool>,
    /// allowoverlap
    /// Represents the following attribute in the schema: o:allowoverlap
    #[xml(attr = "o:allowoverlap")]
    pub allow_overlap: Option<bool>,
    /// insetmode
    /// Represents the following attribute in the schema: o:insetmode
    #[xml(attr = "o:insetmode")]
    pub inset_mode: Option<InsetMarginValues>,
    #[xml(
        child = "v:fill",
        child = "v:imagedata",
        child = "v:stroke",
        child = "v:textbox",
        child = "v:shadow",
        child = "o:skew",
        child = "o:extrusion",
        child = "o:callout",
        child = "o:lock",
        child = "o:colormru",
        child = "o:colormenu",
    )]
    pub children: Vec<ShapeDefaultsChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum ShapeDefaultsChildChoice {
    #[xml(tag = "v:fill")]
    VFill(crate::schemas::schemas_microsoft_com_vml::Fill),
    #[xml(tag = "v:imagedata")]
    VImagedata(crate::schemas::schemas_microsoft_com_vml::ImageData),
    #[xml(tag = "v:stroke")]
    VStroke(crate::schemas::schemas_microsoft_com_vml::Stroke),
    #[xml(tag = "v:textbox")]
    VTextbox(crate::schemas::schemas_microsoft_com_vml::TextBox),
    #[xml(tag = "v:shadow")]
    VShadow(crate::schemas::schemas_microsoft_com_vml::Shadow),
    #[xml(tag = "o:skew")]
    OSkew(Skew),
    #[xml(tag = "o:extrusion")]
    OExtrusion(Extrusion),
    #[xml(tag = "o:callout")]
    OCallout(Callout),
    #[xml(tag = "o:lock")]
    OLock(Lock),
    #[xml(tag = "o:colormru")]
    OColormru(ColorMostRecentlyUsed),
    #[xml(tag = "o:colormenu")]
    OColormenu(ColorMenu),
}
/// Shape Layout Properties.
/// When the object is serialized out as xml, it's qualified name is o:shapelayout.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "o:shapelayout")]
pub struct ShapeLayout {
    /// VML Extension Handling Behavior
    /// Represents the following attribute in the schema: v:ext
    #[xml(attr = "v:ext")]
    pub extension: Option<
        crate::schemas::schemas_microsoft_com_vml::ExtensionHandlingBehaviorValues,
    >,
    #[xml(child = "o:idmap", child = "o:regrouptable", child = "o:rules")]
    pub children: Vec<ShapeLayoutChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum ShapeLayoutChildChoice {
    #[xml(tag = "o:idmap")]
    OIdmap(ShapeIdMap),
    #[xml(tag = "o:regrouptable")]
    ORegrouptable(RegroupTable),
    #[xml(tag = "o:rules")]
    ORules(Rules),
}
/// Digital Signature Line.
/// When the object is serialized out as xml, it's qualified name is o:signatureline.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "o:signatureline")]
pub struct SignatureLine {
    /// VML Extension Handling Behavior
    /// Represents the following attribute in the schema: v:ext
    #[xml(attr = "v:ext")]
    pub extension: Option<
        crate::schemas::schemas_microsoft_com_vml::ExtensionHandlingBehaviorValues,
    >,
    /// Signature Line Flag
    /// Represents the following attribute in the schema: :issignatureline
    #[xml(attr = "issignatureline")]
    pub is_signature_line: Option<bool>,
    /// Unique ID
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: Option<String>,
    /// Signature Provider ID
    /// Represents the following attribute in the schema: :provid
    #[xml(attr = "provid")]
    pub provider_id: Option<String>,
    /// Use Signing Instructions Flag
    /// Represents the following attribute in the schema: :signinginstructionsset
    #[xml(attr = "signinginstructionsset")]
    pub signing_instructions_set: Option<bool>,
    /// User-specified Comments Flag
    /// Represents the following attribute in the schema: :allowcomments
    #[xml(attr = "allowcomments")]
    pub allow_comments: Option<bool>,
    /// Show Signed Date Flag
    /// Represents the following attribute in the schema: :showsigndate
    #[xml(attr = "showsigndate")]
    pub show_sign_date: Option<bool>,
    /// Suggested Signer Line 1
    /// Represents the following attribute in the schema: o:suggestedsigner
    #[xml(attr = "o:suggestedsigner")]
    pub suggested_signer: Option<String>,
    /// Suggested Signer Line 2
    /// Represents the following attribute in the schema: o:suggestedsigner2
    #[xml(attr = "o:suggestedsigner2")]
    pub suggested_signer2: Option<String>,
    /// Suggested Signer E-mail Address
    /// Represents the following attribute in the schema: o:suggestedsigneremail
    #[xml(attr = "o:suggestedsigneremail")]
    pub suggested_signer_email: Option<String>,
    /// Instructions for Signing
    /// Represents the following attribute in the schema: :signinginstructions
    #[xml(attr = "signinginstructions")]
    pub signing_instructions: Option<String>,
    /// Additional Signature Information
    /// Represents the following attribute in the schema: :addlxml
    #[xml(attr = "addlxml")]
    pub additional_xml: Option<String>,
    /// Signature Provider Download URL
    /// Represents the following attribute in the schema: :sigprovurl
    #[xml(attr = "sigprovurl")]
    pub signature_provider_url: Option<String>,
}
/// Ink.
/// When the object is serialized out as xml, it's qualified name is o:ink.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "o:ink")]
pub struct Ink {
    /// Ink Data
    /// Represents the following attribute in the schema: :i
    #[xml(attr = "i")]
    pub ink_data: Option<String>,
    /// Annotation Flag
    /// Represents the following attribute in the schema: :annotation
    #[xml(attr = "annotation")]
    pub annotation_flag: Option<bool>,
}
/// VML Diagram.
/// When the object is serialized out as xml, it's qualified name is o:diagram.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "o:diagram")]
pub struct Diagram {
    /// VML Extension Handling Behavior
    /// Represents the following attribute in the schema: v:ext
    #[xml(attr = "v:ext")]
    pub extension: Option<
        crate::schemas::schemas_microsoft_com_vml::ExtensionHandlingBehaviorValues,
    >,
    /// Diagram Style Options
    /// Represents the following attribute in the schema: :dgmstyle
    #[xml(attr = "dgmstyle")]
    pub style: Option<i32>,
    /// Diagram Automatic Format
    /// Represents the following attribute in the schema: :autoformat
    #[xml(attr = "autoformat")]
    pub auto_format: Option<bool>,
    /// Diagram Reverse Direction
    /// Represents the following attribute in the schema: :reverse
    #[xml(attr = "reverse")]
    pub reverse: Option<bool>,
    /// Diagram Automatic Layout
    /// Represents the following attribute in the schema: :autolayout
    #[xml(attr = "autolayout")]
    pub auto_layout: Option<bool>,
    /// Diagram Layout X Scale
    /// Represents the following attribute in the schema: :dgmscalex
    #[xml(attr = "dgmscalex")]
    pub scale_x: Option<i32>,
    /// Diagram Layout Y Scale
    /// Represents the following attribute in the schema: :dgmscaley
    #[xml(attr = "dgmscaley")]
    pub scale_y: Option<i32>,
    /// Diagram Font Size
    /// Represents the following attribute in the schema: :dgmfontsize
    #[xml(attr = "dgmfontsize")]
    pub font_size: Option<i32>,
    /// Diagram Layout Extents
    /// Represents the following attribute in the schema: :constrainbounds
    #[xml(attr = "constrainbounds")]
    pub constrain_bounds: Option<String>,
    /// Diagram Base Font Size
    /// Represents the following attribute in the schema: :dgmbasetextscale
    #[xml(attr = "dgmbasetextscale")]
    pub base_text_scale: Option<i32>,
    ///Diagram Relationship Table
    #[xml(child = "o:relationtable")]
    pub relation_table: Option<RelationTable>,
}
/// Skew Transform.
/// When the object is serialized out as xml, it's qualified name is o:skew.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "o:skew")]
pub struct Skew {
    /// VML Extension Handling Behavior
    /// Represents the following attribute in the schema: v:ext
    #[xml(attr = "v:ext")]
    pub extension: Option<
        crate::schemas::schemas_microsoft_com_vml::ExtensionHandlingBehaviorValues,
    >,
    /// Skew ID
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: Option<String>,
    /// Skew Toggle
    /// Represents the following attribute in the schema: :on
    #[xml(attr = "on")]
    pub on: Option<bool>,
    /// Skew Offset
    /// Represents the following attribute in the schema: :offset
    #[xml(attr = "offset")]
    pub offset: Option<String>,
    /// Skew Origin
    /// Represents the following attribute in the schema: :origin
    #[xml(attr = "origin")]
    pub origin: Option<String>,
    /// Skew Perspective Matrix
    /// Represents the following attribute in the schema: :matrix
    #[xml(attr = "matrix")]
    pub matrix: Option<String>,
}
/// 3D Extrusion.
/// When the object is serialized out as xml, it's qualified name is o:extrusion.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "o:extrusion")]
pub struct Extrusion {
    /// VML Extension Handling Behavior
    /// Represents the following attribute in the schema: v:ext
    #[xml(attr = "v:ext")]
    pub extension: Option<
        crate::schemas::schemas_microsoft_com_vml::ExtensionHandlingBehaviorValues,
    >,
    /// Extrusion Toggle
    /// Represents the following attribute in the schema: :on
    #[xml(attr = "on")]
    pub on: Option<bool>,
    /// Extrusion Type
    /// Represents the following attribute in the schema: :type
    #[xml(attr = "type")]
    pub r#type: Option<ExtrusionValues>,
    /// Extrusion Render Mode
    /// Represents the following attribute in the schema: :render
    #[xml(attr = "render")]
    pub render: Option<ExtrusionRenderValues>,
    /// Extrusion Viewpoint Origin
    /// Represents the following attribute in the schema: :viewpointorigin
    #[xml(attr = "viewpointorigin")]
    pub viewpoint_origin: Option<String>,
    /// Extrusion Viewpoint
    /// Represents the following attribute in the schema: :viewpoint
    #[xml(attr = "viewpoint")]
    pub viewpoint: Option<String>,
    /// Extrusion Skew Angle
    /// Represents the following attribute in the schema: :skewangle
    #[xml(attr = "skewangle")]
    pub skew_angle: Option<f32>,
    /// Extrusion Skew
    /// Represents the following attribute in the schema: :skewamt
    #[xml(attr = "skewamt")]
    pub skew_amount: Option<String>,
    /// Forward Extrusion
    /// Represents the following attribute in the schema: :foredepth
    #[xml(attr = "foredepth")]
    pub force_depth: Option<String>,
    /// Backward Extrusion Depth
    /// Represents the following attribute in the schema: :backdepth
    #[xml(attr = "backdepth")]
    pub back_depth: Option<String>,
    /// Rotation Axis
    /// Represents the following attribute in the schema: :orientation
    #[xml(attr = "orientation")]
    pub orientation: Option<String>,
    /// Rotation Around Axis
    /// Represents the following attribute in the schema: :orientationangle
    #[xml(attr = "orientationangle")]
    pub orientation_angle: Option<f32>,
    /// Rotation Toggle
    /// Represents the following attribute in the schema: :lockrotationcenter
    #[xml(attr = "lockrotationcenter")]
    pub lock_rotation_center: Option<bool>,
    /// Center of Rotation Toggle
    /// Represents the following attribute in the schema: :autorotationcenter
    #[xml(attr = "autorotationcenter")]
    pub auto_rotation_center: Option<bool>,
    /// Rotation Center
    /// Represents the following attribute in the schema: :rotationcenter
    #[xml(attr = "rotationcenter")]
    pub rotation_center: Option<String>,
    /// X-Y Rotation Angle
    /// Represents the following attribute in the schema: :rotationangle
    #[xml(attr = "rotationangle")]
    pub rotation_angle: Option<String>,
    /// Extrusion Color
    /// Represents the following attribute in the schema: :color
    #[xml(attr = "color")]
    pub color: Option<String>,
    /// Shininess
    /// Represents the following attribute in the schema: :shininess
    #[xml(attr = "shininess")]
    pub shininess: Option<f32>,
    /// Specularity
    /// Represents the following attribute in the schema: :specularity
    #[xml(attr = "specularity")]
    pub specularity: Option<String>,
    /// Diffuse Reflection
    /// Represents the following attribute in the schema: :diffusity
    #[xml(attr = "diffusity")]
    pub diffusity: Option<String>,
    /// Metallic Surface Toggle
    /// Represents the following attribute in the schema: :metal
    #[xml(attr = "metal")]
    pub metal: Option<bool>,
    /// Simulated Bevel
    /// Represents the following attribute in the schema: :edge
    #[xml(attr = "edge")]
    pub edge: Option<String>,
    /// Faceting Quality
    /// Represents the following attribute in the schema: :facet
    #[xml(attr = "facet")]
    pub facet: Option<String>,
    /// Shape Face Lighting Toggle
    /// Represents the following attribute in the schema: :lightface
    #[xml(attr = "lightface")]
    pub light_face: Option<bool>,
    /// Brightness
    /// Represents the following attribute in the schema: :brightness
    #[xml(attr = "brightness")]
    pub brightness: Option<String>,
    /// Primary Light Position
    /// Represents the following attribute in the schema: :lightposition
    #[xml(attr = "lightposition")]
    pub light_position: Option<String>,
    /// Primary Light Intensity
    /// Represents the following attribute in the schema: :lightlevel
    #[xml(attr = "lightlevel")]
    pub light_level: Option<String>,
    /// Primary Light Harshness Toggle
    /// Represents the following attribute in the schema: :lightharsh
    #[xml(attr = "lightharsh")]
    pub light_harsh: Option<bool>,
    /// Secondary Light Position
    /// Represents the following attribute in the schema: :lightposition2
    #[xml(attr = "lightposition2")]
    pub light_position2: Option<String>,
    /// Secondary Light Intensity
    /// Represents the following attribute in the schema: :lightlevel2
    #[xml(attr = "lightlevel2")]
    pub light_level2: Option<String>,
    /// Secondary Light Harshness Toggle
    /// Represents the following attribute in the schema: :lightharsh2
    #[xml(attr = "lightharsh2")]
    pub light_harsh2: Option<bool>,
}
/// Defines the Callout Class.
/// When the object is serialized out as xml, it's qualified name is o:callout.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "o:callout")]
pub struct Callout {
    /// VML Extension Handling Behavior
    /// Represents the following attribute in the schema: v:ext
    #[xml(attr = "v:ext")]
    pub extension: Option<
        crate::schemas::schemas_microsoft_com_vml::ExtensionHandlingBehaviorValues,
    >,
    /// Callout toggle
    /// Represents the following attribute in the schema: :on
    #[xml(attr = "on")]
    pub on: Option<bool>,
    /// Callout type
    /// Represents the following attribute in the schema: :type
    #[xml(attr = "type")]
    pub r#type: Option<String>,
    /// Callout gap
    /// Represents the following attribute in the schema: :gap
    #[xml(attr = "gap")]
    pub gap: Option<String>,
    /// Callout angle
    /// Represents the following attribute in the schema: :angle
    #[xml(attr = "angle")]
    pub angle: Option<AngleValues>,
    /// Callout automatic drop toggle
    /// Represents the following attribute in the schema: :dropauto
    #[xml(attr = "dropauto")]
    pub drop_auto: Option<bool>,
    /// Callout drop position
    /// Represents the following attribute in the schema: :drop
    #[xml(attr = "drop")]
    pub drop: Option<String>,
    /// Callout drop distance
    /// Represents the following attribute in the schema: :distance
    #[xml(attr = "distance")]
    pub distance: Option<String>,
    /// Callout length toggle
    /// Represents the following attribute in the schema: :lengthspecified
    #[xml(attr = "lengthspecified")]
    pub length_specified: Option<bool>,
    /// Callout length
    /// Represents the following attribute in the schema: :length
    #[xml(attr = "length")]
    pub length: Option<String>,
    /// Callout accent bar toggle
    /// Represents the following attribute in the schema: :accentbar
    #[xml(attr = "accentbar")]
    pub accent_bar: Option<bool>,
    /// Callout text border toggle
    /// Represents the following attribute in the schema: :textborder
    #[xml(attr = "textborder")]
    pub text_border: Option<bool>,
    /// Callout flip x
    /// Represents the following attribute in the schema: :minusx
    #[xml(attr = "minusx")]
    pub minus_x: Option<bool>,
    /// Callout flip y
    /// Represents the following attribute in the schema: :minusy
    #[xml(attr = "minusy")]
    pub minus_y: Option<bool>,
}
/// Defines the Lock Class.
/// When the object is serialized out as xml, it's qualified name is o:lock.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "o:lock")]
pub struct Lock {
    /// VML Extension Handling Behavior
    /// Represents the following attribute in the schema: v:ext
    #[xml(attr = "v:ext")]
    pub extension: Option<
        crate::schemas::schemas_microsoft_com_vml::ExtensionHandlingBehaviorValues,
    >,
    /// Position Lock
    /// Represents the following attribute in the schema: :position
    #[xml(attr = "position")]
    pub position: Option<bool>,
    /// Selection Lock
    /// Represents the following attribute in the schema: :selection
    #[xml(attr = "selection")]
    pub selection: Option<bool>,
    /// Grouping Lock
    /// Represents the following attribute in the schema: :grouping
    #[xml(attr = "grouping")]
    pub grouping: Option<bool>,
    /// Ungrouping Lock
    /// Represents the following attribute in the schema: :ungrouping
    #[xml(attr = "ungrouping")]
    pub ungrouping: Option<bool>,
    /// Rotation Lock
    /// Represents the following attribute in the schema: :rotation
    #[xml(attr = "rotation")]
    pub rotation: Option<bool>,
    /// Cropping Lock
    /// Represents the following attribute in the schema: :cropping
    #[xml(attr = "cropping")]
    pub cropping: Option<bool>,
    /// Vertices Lock
    /// Represents the following attribute in the schema: :verticies
    #[xml(attr = "verticies")]
    pub verticies: Option<bool>,
    /// Handles Lock
    /// Represents the following attribute in the schema: :adjusthandles
    #[xml(attr = "adjusthandles")]
    pub adjust_handles: Option<bool>,
    /// Text Lock
    /// Represents the following attribute in the schema: :text
    #[xml(attr = "text")]
    pub text_lock: Option<bool>,
    /// Aspect Ratio Lock
    /// Represents the following attribute in the schema: :aspectratio
    #[xml(attr = "aspectratio")]
    pub aspect_ratio: Option<bool>,
    /// AutoShape Type Lock
    /// Represents the following attribute in the schema: :shapetype
    #[xml(attr = "shapetype")]
    pub shape_type: Option<bool>,
}
/// Embedded OLE Object.
/// When the object is serialized out as xml, it's qualified name is o:OLEObject.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "o:OLEObject")]
pub struct OleObject {
    /// OLE Object Type
    /// Represents the following attribute in the schema: :Type
    #[xml(attr = "Type")]
    pub r#type: Option<OleValues>,
    /// OLE Object Application
    /// Represents the following attribute in the schema: :ProgID
    #[xml(attr = "ProgID")]
    pub prog_id: Option<String>,
    /// OLE Object Shape
    /// Represents the following attribute in the schema: :ShapeID
    #[xml(attr = "ShapeID")]
    pub shape_id: Option<String>,
    /// OLE Object Representation
    /// Represents the following attribute in the schema: :DrawAspect
    #[xml(attr = "DrawAspect")]
    pub draw_aspect: Option<OleDrawAspectValues>,
    /// OLE Object Unique ID
    /// Represents the following attribute in the schema: :ObjectID
    #[xml(attr = "ObjectID")]
    pub object_id: Option<String>,
    /// Relationship
    /// Represents the following attribute in the schema: r:id
    #[xml(attr = "r:id")]
    pub id: Option<String>,
    /// OLE Update Mode
    /// Represents the following attribute in the schema: :UpdateMode
    #[xml(attr = "UpdateMode")]
    pub update_mode: Option<OleUpdateModeValues>,
    ///Embedded Object Alternate Image Request
    #[xml(child = "o:LinkType")]
    pub link_type: Option<LinkType>,
    ///Embedded Object Cannot Be Refreshed
    #[xml(child = "o:LockedField")]
    pub locked_field: Option<LockedField>,
    ///WordprocessingML Field Switches
    #[xml(child = "o:FieldCodes")]
    pub field_codes: Option<FieldCodes>,
}
/// Complex.
/// When the object is serialized out as xml, it's qualified name is o:complex.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "o:complex")]
pub struct Complex {
    /// VML Extension Handling Behavior
    /// Represents the following attribute in the schema: v:ext
    #[xml(attr = "v:ext")]
    pub extension: Option<
        crate::schemas::schemas_microsoft_com_vml::ExtensionHandlingBehaviorValues,
    >,
}
/// Text Box Left Stroke.
/// When the object is serialized out as xml, it's qualified name is o:left.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "o:left")]
pub struct LeftStroke {
    /// VML Extension Handling Behavior
    /// Represents the following attribute in the schema: v:ext
    #[xml(attr = "v:ext")]
    pub extension: Option<
        crate::schemas::schemas_microsoft_com_vml::ExtensionHandlingBehaviorValues,
    >,
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
    /// Stroke Alternate Pattern Color
    /// Represents the following attribute in the schema: :color2
    #[xml(attr = "color2")]
    pub color2: Option<String>,
    /// Stroke Opacity
    /// Represents the following attribute in the schema: :opacity
    #[xml(attr = "opacity")]
    pub opacity: Option<String>,
    /// Stroke Line Style
    /// Represents the following attribute in the schema: :linestyle
    #[xml(attr = "linestyle")]
    pub line_style: Option<
        crate::schemas::schemas_microsoft_com_vml::StrokeLineStyleValues,
    >,
    /// Miter Joint Limit
    /// Represents the following attribute in the schema: :miterlimit
    #[xml(attr = "miterlimit")]
    pub miter_limit: Option<String>,
    /// Line End Join Style)
    /// Represents the following attribute in the schema: :joinstyle
    #[xml(attr = "joinstyle")]
    pub join_style: Option<
        crate::schemas::schemas_microsoft_com_vml::StrokeJoinStyleValues,
    >,
    /// Line End Cap
    /// Represents the following attribute in the schema: :endcap
    #[xml(attr = "endcap")]
    pub end_cap: Option<crate::schemas::schemas_microsoft_com_vml::StrokeEndCapValues>,
    /// Stroke Dash Pattern
    /// Represents the following attribute in the schema: :dashstyle
    #[xml(attr = "dashstyle")]
    pub dash_style: Option<String>,
    /// Inset Border From Path
    /// Represents the following attribute in the schema: :insetpen
    #[xml(attr = "insetpen")]
    pub inset_pen: Option<bool>,
    /// Stroke Image Style
    /// Represents the following attribute in the schema: :filltype
    #[xml(attr = "filltype")]
    pub fill_type: Option<crate::schemas::schemas_microsoft_com_vml::FillTypeValues>,
    /// Stroke Image Location
    /// Represents the following attribute in the schema: :src
    #[xml(attr = "src")]
    pub source: Option<String>,
    /// Stroke Image Aspect Ratio
    /// Represents the following attribute in the schema: :imageaspect
    #[xml(attr = "imageaspect")]
    pub image_aspect: Option<
        crate::schemas::schemas_microsoft_com_vml::ImageAspectValues,
    >,
    /// Stroke Image Size
    /// Represents the following attribute in the schema: :imagesize
    #[xml(attr = "imagesize")]
    pub image_size: Option<String>,
    /// Stoke Image Alignment
    /// Represents the following attribute in the schema: :imagealignshape
    #[xml(attr = "imagealignshape")]
    pub image_align_shape: Option<bool>,
    /// Line Start Arrowhead
    /// Represents the following attribute in the schema: :startarrow
    #[xml(attr = "startarrow")]
    pub start_arrow: Option<
        crate::schemas::schemas_microsoft_com_vml::StrokeArrowValues,
    >,
    /// Line Start Arrowhead Width
    /// Represents the following attribute in the schema: :startarrowwidth
    #[xml(attr = "startarrowwidth")]
    pub start_arrow_width: Option<
        crate::schemas::schemas_microsoft_com_vml::StrokeArrowWidthValues,
    >,
    /// Line Start Arrowhead Length
    /// Represents the following attribute in the schema: :startarrowlength
    #[xml(attr = "startarrowlength")]
    pub start_arrow_length: Option<
        crate::schemas::schemas_microsoft_com_vml::StrokeArrowLengthValues,
    >,
    /// Line End Arrowhead
    /// Represents the following attribute in the schema: :endarrow
    #[xml(attr = "endarrow")]
    pub end_arrow: Option<crate::schemas::schemas_microsoft_com_vml::StrokeArrowValues>,
    /// Line End Arrowhead Width
    /// Represents the following attribute in the schema: :endarrowwidth
    #[xml(attr = "endarrowwidth")]
    pub end_arrow_width: Option<
        crate::schemas::schemas_microsoft_com_vml::StrokeArrowWidthValues,
    >,
    /// Line End Arrowhead Length
    /// Represents the following attribute in the schema: :endarrowlength
    #[xml(attr = "endarrowlength")]
    pub end_arrow_length: Option<
        crate::schemas::schemas_microsoft_com_vml::StrokeArrowLengthValues,
    >,
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
}
/// Text Box Top Stroke.
/// When the object is serialized out as xml, it's qualified name is o:top.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "o:top")]
pub struct TopStroke {
    /// VML Extension Handling Behavior
    /// Represents the following attribute in the schema: v:ext
    #[xml(attr = "v:ext")]
    pub extension: Option<
        crate::schemas::schemas_microsoft_com_vml::ExtensionHandlingBehaviorValues,
    >,
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
    /// Stroke Alternate Pattern Color
    /// Represents the following attribute in the schema: :color2
    #[xml(attr = "color2")]
    pub color2: Option<String>,
    /// Stroke Opacity
    /// Represents the following attribute in the schema: :opacity
    #[xml(attr = "opacity")]
    pub opacity: Option<String>,
    /// Stroke Line Style
    /// Represents the following attribute in the schema: :linestyle
    #[xml(attr = "linestyle")]
    pub line_style: Option<
        crate::schemas::schemas_microsoft_com_vml::StrokeLineStyleValues,
    >,
    /// Miter Joint Limit
    /// Represents the following attribute in the schema: :miterlimit
    #[xml(attr = "miterlimit")]
    pub miter_limit: Option<String>,
    /// Line End Join Style)
    /// Represents the following attribute in the schema: :joinstyle
    #[xml(attr = "joinstyle")]
    pub join_style: Option<
        crate::schemas::schemas_microsoft_com_vml::StrokeJoinStyleValues,
    >,
    /// Line End Cap
    /// Represents the following attribute in the schema: :endcap
    #[xml(attr = "endcap")]
    pub end_cap: Option<crate::schemas::schemas_microsoft_com_vml::StrokeEndCapValues>,
    /// Stroke Dash Pattern
    /// Represents the following attribute in the schema: :dashstyle
    #[xml(attr = "dashstyle")]
    pub dash_style: Option<String>,
    /// Inset Border From Path
    /// Represents the following attribute in the schema: :insetpen
    #[xml(attr = "insetpen")]
    pub inset_pen: Option<bool>,
    /// Stroke Image Style
    /// Represents the following attribute in the schema: :filltype
    #[xml(attr = "filltype")]
    pub fill_type: Option<crate::schemas::schemas_microsoft_com_vml::FillTypeValues>,
    /// Stroke Image Location
    /// Represents the following attribute in the schema: :src
    #[xml(attr = "src")]
    pub source: Option<String>,
    /// Stroke Image Aspect Ratio
    /// Represents the following attribute in the schema: :imageaspect
    #[xml(attr = "imageaspect")]
    pub image_aspect: Option<
        crate::schemas::schemas_microsoft_com_vml::ImageAspectValues,
    >,
    /// Stroke Image Size
    /// Represents the following attribute in the schema: :imagesize
    #[xml(attr = "imagesize")]
    pub image_size: Option<String>,
    /// Stoke Image Alignment
    /// Represents the following attribute in the schema: :imagealignshape
    #[xml(attr = "imagealignshape")]
    pub image_align_shape: Option<bool>,
    /// Line Start Arrowhead
    /// Represents the following attribute in the schema: :startarrow
    #[xml(attr = "startarrow")]
    pub start_arrow: Option<
        crate::schemas::schemas_microsoft_com_vml::StrokeArrowValues,
    >,
    /// Line Start Arrowhead Width
    /// Represents the following attribute in the schema: :startarrowwidth
    #[xml(attr = "startarrowwidth")]
    pub start_arrow_width: Option<
        crate::schemas::schemas_microsoft_com_vml::StrokeArrowWidthValues,
    >,
    /// Line Start Arrowhead Length
    /// Represents the following attribute in the schema: :startarrowlength
    #[xml(attr = "startarrowlength")]
    pub start_arrow_length: Option<
        crate::schemas::schemas_microsoft_com_vml::StrokeArrowLengthValues,
    >,
    /// Line End Arrowhead
    /// Represents the following attribute in the schema: :endarrow
    #[xml(attr = "endarrow")]
    pub end_arrow: Option<crate::schemas::schemas_microsoft_com_vml::StrokeArrowValues>,
    /// Line End Arrowhead Width
    /// Represents the following attribute in the schema: :endarrowwidth
    #[xml(attr = "endarrowwidth")]
    pub end_arrow_width: Option<
        crate::schemas::schemas_microsoft_com_vml::StrokeArrowWidthValues,
    >,
    /// Line End Arrowhead Length
    /// Represents the following attribute in the schema: :endarrowlength
    #[xml(attr = "endarrowlength")]
    pub end_arrow_length: Option<
        crate::schemas::schemas_microsoft_com_vml::StrokeArrowLengthValues,
    >,
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
}
/// Text Box Right Stroke.
/// When the object is serialized out as xml, it's qualified name is o:right.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "o:right")]
pub struct RightStroke {
    /// VML Extension Handling Behavior
    /// Represents the following attribute in the schema: v:ext
    #[xml(attr = "v:ext")]
    pub extension: Option<
        crate::schemas::schemas_microsoft_com_vml::ExtensionHandlingBehaviorValues,
    >,
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
    /// Stroke Alternate Pattern Color
    /// Represents the following attribute in the schema: :color2
    #[xml(attr = "color2")]
    pub color2: Option<String>,
    /// Stroke Opacity
    /// Represents the following attribute in the schema: :opacity
    #[xml(attr = "opacity")]
    pub opacity: Option<String>,
    /// Stroke Line Style
    /// Represents the following attribute in the schema: :linestyle
    #[xml(attr = "linestyle")]
    pub line_style: Option<
        crate::schemas::schemas_microsoft_com_vml::StrokeLineStyleValues,
    >,
    /// Miter Joint Limit
    /// Represents the following attribute in the schema: :miterlimit
    #[xml(attr = "miterlimit")]
    pub miter_limit: Option<String>,
    /// Line End Join Style)
    /// Represents the following attribute in the schema: :joinstyle
    #[xml(attr = "joinstyle")]
    pub join_style: Option<
        crate::schemas::schemas_microsoft_com_vml::StrokeJoinStyleValues,
    >,
    /// Line End Cap
    /// Represents the following attribute in the schema: :endcap
    #[xml(attr = "endcap")]
    pub end_cap: Option<crate::schemas::schemas_microsoft_com_vml::StrokeEndCapValues>,
    /// Stroke Dash Pattern
    /// Represents the following attribute in the schema: :dashstyle
    #[xml(attr = "dashstyle")]
    pub dash_style: Option<String>,
    /// Inset Border From Path
    /// Represents the following attribute in the schema: :insetpen
    #[xml(attr = "insetpen")]
    pub inset_pen: Option<bool>,
    /// Stroke Image Style
    /// Represents the following attribute in the schema: :filltype
    #[xml(attr = "filltype")]
    pub fill_type: Option<crate::schemas::schemas_microsoft_com_vml::FillTypeValues>,
    /// Stroke Image Location
    /// Represents the following attribute in the schema: :src
    #[xml(attr = "src")]
    pub source: Option<String>,
    /// Stroke Image Aspect Ratio
    /// Represents the following attribute in the schema: :imageaspect
    #[xml(attr = "imageaspect")]
    pub image_aspect: Option<
        crate::schemas::schemas_microsoft_com_vml::ImageAspectValues,
    >,
    /// Stroke Image Size
    /// Represents the following attribute in the schema: :imagesize
    #[xml(attr = "imagesize")]
    pub image_size: Option<String>,
    /// Stoke Image Alignment
    /// Represents the following attribute in the schema: :imagealignshape
    #[xml(attr = "imagealignshape")]
    pub image_align_shape: Option<bool>,
    /// Line Start Arrowhead
    /// Represents the following attribute in the schema: :startarrow
    #[xml(attr = "startarrow")]
    pub start_arrow: Option<
        crate::schemas::schemas_microsoft_com_vml::StrokeArrowValues,
    >,
    /// Line Start Arrowhead Width
    /// Represents the following attribute in the schema: :startarrowwidth
    #[xml(attr = "startarrowwidth")]
    pub start_arrow_width: Option<
        crate::schemas::schemas_microsoft_com_vml::StrokeArrowWidthValues,
    >,
    /// Line Start Arrowhead Length
    /// Represents the following attribute in the schema: :startarrowlength
    #[xml(attr = "startarrowlength")]
    pub start_arrow_length: Option<
        crate::schemas::schemas_microsoft_com_vml::StrokeArrowLengthValues,
    >,
    /// Line End Arrowhead
    /// Represents the following attribute in the schema: :endarrow
    #[xml(attr = "endarrow")]
    pub end_arrow: Option<crate::schemas::schemas_microsoft_com_vml::StrokeArrowValues>,
    /// Line End Arrowhead Width
    /// Represents the following attribute in the schema: :endarrowwidth
    #[xml(attr = "endarrowwidth")]
    pub end_arrow_width: Option<
        crate::schemas::schemas_microsoft_com_vml::StrokeArrowWidthValues,
    >,
    /// Line End Arrowhead Length
    /// Represents the following attribute in the schema: :endarrowlength
    #[xml(attr = "endarrowlength")]
    pub end_arrow_length: Option<
        crate::schemas::schemas_microsoft_com_vml::StrokeArrowLengthValues,
    >,
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
}
/// Text Box Bottom Stroke.
/// When the object is serialized out as xml, it's qualified name is o:bottom.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "o:bottom")]
pub struct BottomStroke {
    /// VML Extension Handling Behavior
    /// Represents the following attribute in the schema: v:ext
    #[xml(attr = "v:ext")]
    pub extension: Option<
        crate::schemas::schemas_microsoft_com_vml::ExtensionHandlingBehaviorValues,
    >,
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
    /// Stroke Alternate Pattern Color
    /// Represents the following attribute in the schema: :color2
    #[xml(attr = "color2")]
    pub color2: Option<String>,
    /// Stroke Opacity
    /// Represents the following attribute in the schema: :opacity
    #[xml(attr = "opacity")]
    pub opacity: Option<String>,
    /// Stroke Line Style
    /// Represents the following attribute in the schema: :linestyle
    #[xml(attr = "linestyle")]
    pub line_style: Option<
        crate::schemas::schemas_microsoft_com_vml::StrokeLineStyleValues,
    >,
    /// Miter Joint Limit
    /// Represents the following attribute in the schema: :miterlimit
    #[xml(attr = "miterlimit")]
    pub miter_limit: Option<String>,
    /// Line End Join Style)
    /// Represents the following attribute in the schema: :joinstyle
    #[xml(attr = "joinstyle")]
    pub join_style: Option<
        crate::schemas::schemas_microsoft_com_vml::StrokeJoinStyleValues,
    >,
    /// Line End Cap
    /// Represents the following attribute in the schema: :endcap
    #[xml(attr = "endcap")]
    pub end_cap: Option<crate::schemas::schemas_microsoft_com_vml::StrokeEndCapValues>,
    /// Stroke Dash Pattern
    /// Represents the following attribute in the schema: :dashstyle
    #[xml(attr = "dashstyle")]
    pub dash_style: Option<String>,
    /// Inset Border From Path
    /// Represents the following attribute in the schema: :insetpen
    #[xml(attr = "insetpen")]
    pub inset_pen: Option<bool>,
    /// Stroke Image Style
    /// Represents the following attribute in the schema: :filltype
    #[xml(attr = "filltype")]
    pub fill_type: Option<crate::schemas::schemas_microsoft_com_vml::FillTypeValues>,
    /// Stroke Image Location
    /// Represents the following attribute in the schema: :src
    #[xml(attr = "src")]
    pub source: Option<String>,
    /// Stroke Image Aspect Ratio
    /// Represents the following attribute in the schema: :imageaspect
    #[xml(attr = "imageaspect")]
    pub image_aspect: Option<
        crate::schemas::schemas_microsoft_com_vml::ImageAspectValues,
    >,
    /// Stroke Image Size
    /// Represents the following attribute in the schema: :imagesize
    #[xml(attr = "imagesize")]
    pub image_size: Option<String>,
    /// Stoke Image Alignment
    /// Represents the following attribute in the schema: :imagealignshape
    #[xml(attr = "imagealignshape")]
    pub image_align_shape: Option<bool>,
    /// Line Start Arrowhead
    /// Represents the following attribute in the schema: :startarrow
    #[xml(attr = "startarrow")]
    pub start_arrow: Option<
        crate::schemas::schemas_microsoft_com_vml::StrokeArrowValues,
    >,
    /// Line Start Arrowhead Width
    /// Represents the following attribute in the schema: :startarrowwidth
    #[xml(attr = "startarrowwidth")]
    pub start_arrow_width: Option<
        crate::schemas::schemas_microsoft_com_vml::StrokeArrowWidthValues,
    >,
    /// Line Start Arrowhead Length
    /// Represents the following attribute in the schema: :startarrowlength
    #[xml(attr = "startarrowlength")]
    pub start_arrow_length: Option<
        crate::schemas::schemas_microsoft_com_vml::StrokeArrowLengthValues,
    >,
    /// Line End Arrowhead
    /// Represents the following attribute in the schema: :endarrow
    #[xml(attr = "endarrow")]
    pub end_arrow: Option<crate::schemas::schemas_microsoft_com_vml::StrokeArrowValues>,
    /// Line End Arrowhead Width
    /// Represents the following attribute in the schema: :endarrowwidth
    #[xml(attr = "endarrowwidth")]
    pub end_arrow_width: Option<
        crate::schemas::schemas_microsoft_com_vml::StrokeArrowWidthValues,
    >,
    /// Line End Arrowhead Length
    /// Represents the following attribute in the schema: :endarrowlength
    #[xml(attr = "endarrowlength")]
    pub end_arrow_length: Option<
        crate::schemas::schemas_microsoft_com_vml::StrokeArrowLengthValues,
    >,
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
}
/// Text Box Interior Stroke.
/// When the object is serialized out as xml, it's qualified name is o:column.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "o:column")]
pub struct ColumnStroke {
    /// VML Extension Handling Behavior
    /// Represents the following attribute in the schema: v:ext
    #[xml(attr = "v:ext")]
    pub extension: Option<
        crate::schemas::schemas_microsoft_com_vml::ExtensionHandlingBehaviorValues,
    >,
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
    /// Stroke Alternate Pattern Color
    /// Represents the following attribute in the schema: :color2
    #[xml(attr = "color2")]
    pub color2: Option<String>,
    /// Stroke Opacity
    /// Represents the following attribute in the schema: :opacity
    #[xml(attr = "opacity")]
    pub opacity: Option<String>,
    /// Stroke Line Style
    /// Represents the following attribute in the schema: :linestyle
    #[xml(attr = "linestyle")]
    pub line_style: Option<
        crate::schemas::schemas_microsoft_com_vml::StrokeLineStyleValues,
    >,
    /// Miter Joint Limit
    /// Represents the following attribute in the schema: :miterlimit
    #[xml(attr = "miterlimit")]
    pub miter_limit: Option<String>,
    /// Line End Join Style)
    /// Represents the following attribute in the schema: :joinstyle
    #[xml(attr = "joinstyle")]
    pub join_style: Option<
        crate::schemas::schemas_microsoft_com_vml::StrokeJoinStyleValues,
    >,
    /// Line End Cap
    /// Represents the following attribute in the schema: :endcap
    #[xml(attr = "endcap")]
    pub end_cap: Option<crate::schemas::schemas_microsoft_com_vml::StrokeEndCapValues>,
    /// Stroke Dash Pattern
    /// Represents the following attribute in the schema: :dashstyle
    #[xml(attr = "dashstyle")]
    pub dash_style: Option<String>,
    /// Inset Border From Path
    /// Represents the following attribute in the schema: :insetpen
    #[xml(attr = "insetpen")]
    pub inset_pen: Option<bool>,
    /// Stroke Image Style
    /// Represents the following attribute in the schema: :filltype
    #[xml(attr = "filltype")]
    pub fill_type: Option<crate::schemas::schemas_microsoft_com_vml::FillTypeValues>,
    /// Stroke Image Location
    /// Represents the following attribute in the schema: :src
    #[xml(attr = "src")]
    pub source: Option<String>,
    /// Stroke Image Aspect Ratio
    /// Represents the following attribute in the schema: :imageaspect
    #[xml(attr = "imageaspect")]
    pub image_aspect: Option<
        crate::schemas::schemas_microsoft_com_vml::ImageAspectValues,
    >,
    /// Stroke Image Size
    /// Represents the following attribute in the schema: :imagesize
    #[xml(attr = "imagesize")]
    pub image_size: Option<String>,
    /// Stoke Image Alignment
    /// Represents the following attribute in the schema: :imagealignshape
    #[xml(attr = "imagealignshape")]
    pub image_align_shape: Option<bool>,
    /// Line Start Arrowhead
    /// Represents the following attribute in the schema: :startarrow
    #[xml(attr = "startarrow")]
    pub start_arrow: Option<
        crate::schemas::schemas_microsoft_com_vml::StrokeArrowValues,
    >,
    /// Line Start Arrowhead Width
    /// Represents the following attribute in the schema: :startarrowwidth
    #[xml(attr = "startarrowwidth")]
    pub start_arrow_width: Option<
        crate::schemas::schemas_microsoft_com_vml::StrokeArrowWidthValues,
    >,
    /// Line Start Arrowhead Length
    /// Represents the following attribute in the schema: :startarrowlength
    #[xml(attr = "startarrowlength")]
    pub start_arrow_length: Option<
        crate::schemas::schemas_microsoft_com_vml::StrokeArrowLengthValues,
    >,
    /// Line End Arrowhead
    /// Represents the following attribute in the schema: :endarrow
    #[xml(attr = "endarrow")]
    pub end_arrow: Option<crate::schemas::schemas_microsoft_com_vml::StrokeArrowValues>,
    /// Line End Arrowhead Width
    /// Represents the following attribute in the schema: :endarrowwidth
    #[xml(attr = "endarrowwidth")]
    pub end_arrow_width: Option<
        crate::schemas::schemas_microsoft_com_vml::StrokeArrowWidthValues,
    >,
    /// Line End Arrowhead Length
    /// Represents the following attribute in the schema: :endarrowlength
    #[xml(attr = "endarrowlength")]
    pub end_arrow_length: Option<
        crate::schemas::schemas_microsoft_com_vml::StrokeArrowLengthValues,
    >,
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
}
/// Defines the StrokeChildType Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct StrokeChildType {
    /// VML Extension Handling Behavior
    /// Represents the following attribute in the schema: v:ext
    #[xml(attr = "v:ext")]
    pub extension: Option<
        crate::schemas::schemas_microsoft_com_vml::ExtensionHandlingBehaviorValues,
    >,
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
    /// Stroke Alternate Pattern Color
    /// Represents the following attribute in the schema: :color2
    #[xml(attr = "color2")]
    pub color2: Option<String>,
    /// Stroke Opacity
    /// Represents the following attribute in the schema: :opacity
    #[xml(attr = "opacity")]
    pub opacity: Option<String>,
    /// Stroke Line Style
    /// Represents the following attribute in the schema: :linestyle
    #[xml(attr = "linestyle")]
    pub line_style: Option<
        crate::schemas::schemas_microsoft_com_vml::StrokeLineStyleValues,
    >,
    /// Miter Joint Limit
    /// Represents the following attribute in the schema: :miterlimit
    #[xml(attr = "miterlimit")]
    pub miter_limit: Option<String>,
    /// Line End Join Style)
    /// Represents the following attribute in the schema: :joinstyle
    #[xml(attr = "joinstyle")]
    pub join_style: Option<
        crate::schemas::schemas_microsoft_com_vml::StrokeJoinStyleValues,
    >,
    /// Line End Cap
    /// Represents the following attribute in the schema: :endcap
    #[xml(attr = "endcap")]
    pub end_cap: Option<crate::schemas::schemas_microsoft_com_vml::StrokeEndCapValues>,
    /// Stroke Dash Pattern
    /// Represents the following attribute in the schema: :dashstyle
    #[xml(attr = "dashstyle")]
    pub dash_style: Option<String>,
    /// Inset Border From Path
    /// Represents the following attribute in the schema: :insetpen
    #[xml(attr = "insetpen")]
    pub inset_pen: Option<bool>,
    /// Stroke Image Style
    /// Represents the following attribute in the schema: :filltype
    #[xml(attr = "filltype")]
    pub fill_type: Option<crate::schemas::schemas_microsoft_com_vml::FillTypeValues>,
    /// Stroke Image Location
    /// Represents the following attribute in the schema: :src
    #[xml(attr = "src")]
    pub source: Option<String>,
    /// Stroke Image Aspect Ratio
    /// Represents the following attribute in the schema: :imageaspect
    #[xml(attr = "imageaspect")]
    pub image_aspect: Option<
        crate::schemas::schemas_microsoft_com_vml::ImageAspectValues,
    >,
    /// Stroke Image Size
    /// Represents the following attribute in the schema: :imagesize
    #[xml(attr = "imagesize")]
    pub image_size: Option<String>,
    /// Stoke Image Alignment
    /// Represents the following attribute in the schema: :imagealignshape
    #[xml(attr = "imagealignshape")]
    pub image_align_shape: Option<bool>,
    /// Line Start Arrowhead
    /// Represents the following attribute in the schema: :startarrow
    #[xml(attr = "startarrow")]
    pub start_arrow: Option<
        crate::schemas::schemas_microsoft_com_vml::StrokeArrowValues,
    >,
    /// Line Start Arrowhead Width
    /// Represents the following attribute in the schema: :startarrowwidth
    #[xml(attr = "startarrowwidth")]
    pub start_arrow_width: Option<
        crate::schemas::schemas_microsoft_com_vml::StrokeArrowWidthValues,
    >,
    /// Line Start Arrowhead Length
    /// Represents the following attribute in the schema: :startarrowlength
    #[xml(attr = "startarrowlength")]
    pub start_arrow_length: Option<
        crate::schemas::schemas_microsoft_com_vml::StrokeArrowLengthValues,
    >,
    /// Line End Arrowhead
    /// Represents the following attribute in the schema: :endarrow
    #[xml(attr = "endarrow")]
    pub end_arrow: Option<crate::schemas::schemas_microsoft_com_vml::StrokeArrowValues>,
    /// Line End Arrowhead Width
    /// Represents the following attribute in the schema: :endarrowwidth
    #[xml(attr = "endarrowwidth")]
    pub end_arrow_width: Option<
        crate::schemas::schemas_microsoft_com_vml::StrokeArrowWidthValues,
    >,
    /// Line End Arrowhead Length
    /// Represents the following attribute in the schema: :endarrowlength
    #[xml(attr = "endarrowlength")]
    pub end_arrow_length: Option<
        crate::schemas::schemas_microsoft_com_vml::StrokeArrowLengthValues,
    >,
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
}
/// Shape Clipping Path.
/// When the object is serialized out as xml, it's qualified name is o:clippath.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "o:clippath")]
pub struct ClipPath {
    /// Path Definition
    /// Represents the following attribute in the schema: o:v
    #[xml(attr = "o:v")]
    pub value: String,
}
/// Shape Fill Extended Properties.
/// When the object is serialized out as xml, it's qualified name is o:fill.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "o:fill")]
pub struct FillExtendedProperties {
    /// VML Extension Handling Behavior
    /// Represents the following attribute in the schema: v:ext
    #[xml(attr = "v:ext")]
    pub extension: Option<
        crate::schemas::schemas_microsoft_com_vml::ExtensionHandlingBehaviorValues,
    >,
    /// Fill Type
    /// Represents the following attribute in the schema: :type
    #[xml(attr = "type")]
    pub r#type: Option<FillValues>,
}
/// Shape ID Map.
/// When the object is serialized out as xml, it's qualified name is o:idmap.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "o:idmap")]
pub struct ShapeIdMap {
    /// VML Extension Handling Behavior
    /// Represents the following attribute in the schema: v:ext
    #[xml(attr = "v:ext")]
    pub extension: Option<
        crate::schemas::schemas_microsoft_com_vml::ExtensionHandlingBehaviorValues,
    >,
    /// Shape IDs
    /// Represents the following attribute in the schema: :data
    #[xml(attr = "data")]
    pub data: Option<String>,
}
/// Shape Grouping History.
/// When the object is serialized out as xml, it's qualified name is o:regrouptable.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "o:regrouptable")]
pub struct RegroupTable {
    /// VML Extension Handling Behavior
    /// Represents the following attribute in the schema: v:ext
    #[xml(attr = "v:ext")]
    pub extension: Option<
        crate::schemas::schemas_microsoft_com_vml::ExtensionHandlingBehaviorValues,
    >,
    /// _
    #[xml(child = "o:entry")]
    pub o_entry: Vec<Entry>,
}
/// Rule Set.
/// When the object is serialized out as xml, it's qualified name is o:rules.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "o:rules")]
pub struct Rules {
    /// VML Extension Handling Behavior
    /// Represents the following attribute in the schema: v:ext
    #[xml(attr = "v:ext")]
    pub extension: Option<
        crate::schemas::schemas_microsoft_com_vml::ExtensionHandlingBehaviorValues,
    >,
    /// _
    #[xml(child = "o:r")]
    pub o_r: Vec<Rule>,
}
/// Regroup Entry.
/// When the object is serialized out as xml, it's qualified name is o:entry.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "o:entry")]
pub struct Entry {
    /// New Group ID
    /// Represents the following attribute in the schema: :new
    #[xml(attr = "new")]
    pub new: Option<i32>,
    /// Old Group ID
    /// Represents the following attribute in the schema: :old
    #[xml(attr = "old")]
    pub old: Option<i32>,
}
/// Rule.
/// When the object is serialized out as xml, it's qualified name is o:r.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "o:r")]
pub struct Rule {
    /// Rule ID
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: String,
    /// Rule Type
    /// Represents the following attribute in the schema: :type
    #[xml(attr = "type")]
    pub r#type: Option<RuleValues>,
    /// Alignment Rule Type
    /// Represents the following attribute in the schema: :how
    #[xml(attr = "how")]
    pub how: Option<AlignmentValues>,
    /// Rule Shape Reference
    /// Represents the following attribute in the schema: :idref
    #[xml(attr = "idref")]
    pub shape_reference: Option<String>,
    /// _
    #[xml(child = "o:proxy")]
    pub o_proxy: Vec<Proxy>,
}
/// Diagram Relationship Table.
/// When the object is serialized out as xml, it's qualified name is o:relationtable.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "o:relationtable")]
pub struct RelationTable {
    /// VML Extension Handling Behavior
    /// Represents the following attribute in the schema: v:ext
    #[xml(attr = "v:ext")]
    pub extension: Option<
        crate::schemas::schemas_microsoft_com_vml::ExtensionHandlingBehaviorValues,
    >,
    /// _
    #[xml(child = "o:rel")]
    pub o_rel: Vec<Relation>,
}
/// Diagram Relationship.
/// When the object is serialized out as xml, it's qualified name is o:rel.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "o:rel")]
pub struct Relation {
    /// VML Extension Handling Behavior
    /// Represents the following attribute in the schema: v:ext
    #[xml(attr = "v:ext")]
    pub extension: Option<
        crate::schemas::schemas_microsoft_com_vml::ExtensionHandlingBehaviorValues,
    >,
    /// Diagram Relationship Source Shape
    /// Represents the following attribute in the schema: :idsrc
    #[xml(attr = "idsrc")]
    pub source_id: Option<String>,
    /// Diagram Relationship Destination Shape
    /// Represents the following attribute in the schema: :iddest
    #[xml(attr = "iddest")]
    pub destination_id: Option<String>,
    /// Diagram Relationship Center Shape
    /// Represents the following attribute in the schema: :idcntr
    #[xml(attr = "idcntr")]
    pub center_shape_id: Option<String>,
}
/// Embedded Object Alternate Image Request.
/// When the object is serialized out as xml, it's qualified name is o:LinkType.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "o:LinkType")]
pub struct LinkType {
    #[xml(attr = "o:LinkType")]
    pub child: Option<OleLinkValues>,
}
/// Embedded Object Cannot Be Refreshed.
/// When the object is serialized out as xml, it's qualified name is o:LockedField.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "o:LockedField")]
pub struct LockedField {
    #[xml(text)]
    pub child: bool,
}
/// WordprocessingML Field Switches.
/// When the object is serialized out as xml, it's qualified name is o:FieldCodes.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "o:FieldCodes")]
pub struct FieldCodes {
    #[xml(text)]
    pub child: String,
}
/// Shape Reference.
/// When the object is serialized out as xml, it's qualified name is o:proxy.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "o:proxy")]
pub struct Proxy {
    /// Start Point Connection Flag
    /// Represents the following attribute in the schema: :start
    #[xml(attr = "start")]
    pub start: Option<bool>,
    /// End Point Connection Flag
    /// Represents the following attribute in the schema: :end
    #[xml(attr = "end")]
    pub end: Option<bool>,
    /// Proxy Shape Reference
    /// Represents the following attribute in the schema: :idref
    #[xml(attr = "idref")]
    pub shape_reference: String,
    /// Connection Location
    /// Represents the following attribute in the schema: :connectloc
    #[xml(attr = "connectloc")]
    pub connection_location: i32,
}
/// Most Recently Used Colors.
/// When the object is serialized out as xml, it's qualified name is o:colormru.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "o:colormru")]
pub struct ColorMostRecentlyUsed {
    /// VML Extension Handling Behavior
    /// Represents the following attribute in the schema: v:ext
    #[xml(attr = "v:ext")]
    pub extension: Option<
        crate::schemas::schemas_microsoft_com_vml::ExtensionHandlingBehaviorValues,
    >,
    /// Recent colors
    /// Represents the following attribute in the schema: :colors
    #[xml(attr = "colors")]
    pub colors: Option<String>,
}
/// UI Default Colors.
/// When the object is serialized out as xml, it's qualified name is o:colormenu.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "o:colormenu")]
pub struct ColorMenu {
    /// VML Extension Handling Behavior
    /// Represents the following attribute in the schema: v:ext
    #[xml(attr = "v:ext")]
    pub extension: Option<
        crate::schemas::schemas_microsoft_com_vml::ExtensionHandlingBehaviorValues,
    >,
    /// Default stroke color
    /// Represents the following attribute in the schema: :strokecolor
    #[xml(attr = "strokecolor")]
    pub stroke_color: Option<String>,
    /// Default fill color
    /// Represents the following attribute in the schema: :fillcolor
    #[xml(attr = "fillcolor")]
    pub fill_color: Option<String>,
    /// Default shadow color
    /// Represents the following attribute in the schema: :shadowcolor
    #[xml(attr = "shadowcolor")]
    pub shadow_color: Option<String>,
    /// Default extrusion color
    /// Represents the following attribute in the schema: :extrusioncolor
    #[xml(attr = "extrusioncolor")]
    pub extrusion_color: Option<String>,
}
