#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum BorderValues {
    #[default]
    None,
    Single,
    Thick,
    Double,
    Hairline,
    Dot,
    Dash,
    DotDash,
    DashDotDot,
    Triple,
    ThinThickSmall,
    ThickThinSmall,
    ThickBetweenThinSmall,
    ThinThick,
    ThickThin,
    ThickBetweenThin,
    ThinThickLarge,
    ThickThinLarge,
    ThickBetweenThinLarge,
    Wave,
    DoubleWave,
    DashedSmall,
    DashDotStroked,
    ThreeDEmboss,
    ThreeDEngrave,
    HtmlOutset,
    HtmlInset,
}
crate::__string_enum! {
    BorderValues { None = "none", Single = "single", Thick = "thick", Double = "double",
    Hairline = "hairline", Dot = "dot", Dash = "dash", DotDash = "dotDash", DashDotDot =
    "dashDotDot", Triple = "triple", ThinThickSmall = "thinThickSmall", ThickThinSmall =
    "thickThinSmall", ThickBetweenThinSmall = "thickBetweenThinSmall", ThinThick =
    "thinThick", ThickThin = "thickThin", ThickBetweenThin = "thickBetweenThin",
    ThinThickLarge = "thinThickLarge", ThickThinLarge = "thickThinLarge",
    ThickBetweenThinLarge = "thickBetweenThinLarge", Wave = "wave", DoubleWave =
    "doubleWave", DashedSmall = "dashedSmall", DashDotStroked = "dashDotStroked",
    ThreeDEmboss = "threeDEmboss", ThreeDEngrave = "threeDEngrave", HtmlOutset =
    "htmlOutset", HtmlInset = "htmlInset", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum WrapValues {
    #[default]
    TopAndBottom,
    Square,
    None,
    Tight,
    Through,
}
crate::__string_enum! {
    WrapValues { TopAndBottom = "topAndBottom", Square = "square", None = "none", Tight =
    "tight", Through = "through", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum WrapSideValues {
    #[default]
    Both,
    Left,
    Right,
    Largest,
}
crate::__string_enum! {
    WrapSideValues { Both = "both", Left = "left", Right = "right", Largest = "largest",
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum HorizontalAnchorValues {
    #[default]
    Margin,
    Page,
    Text,
}
crate::__string_enum! {
    HorizontalAnchorValues { Margin = "margin", Page = "page", Text = "text", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum VerticalAnchorValues {
    #[default]
    Margin,
    Page,
    Text,
}
crate::__string_enum! {
    VerticalAnchorValues { Margin = "margin", Page = "page", Text = "text", }
}
/// Top Border.
/// When the object is serialized out as xml, it's qualified name is w10:bordertop.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w10:bordertop")]
pub struct TopBorder {
    /// Border Style
    /// Represents the following attribute in the schema: :type
    #[xml(attr = "type")]
    pub r#type: Option<BorderValues>,
    /// Border Width
    /// Represents the following attribute in the schema: :width
    #[xml(attr = "width")]
    pub width: Option<i32>,
    /// Border shadow
    /// Represents the following attribute in the schema: :shadow
    #[xml(attr = "shadow")]
    pub shadow: Option<bool>,
}
/// Left Border.
/// When the object is serialized out as xml, it's qualified name is w10:borderleft.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w10:borderleft")]
pub struct LeftBorder {
    /// Border Style
    /// Represents the following attribute in the schema: :type
    #[xml(attr = "type")]
    pub r#type: Option<BorderValues>,
    /// Border Width
    /// Represents the following attribute in the schema: :width
    #[xml(attr = "width")]
    pub width: Option<i32>,
    /// Border shadow
    /// Represents the following attribute in the schema: :shadow
    #[xml(attr = "shadow")]
    pub shadow: Option<bool>,
}
/// Right Border.
/// When the object is serialized out as xml, it's qualified name is w10:borderright.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w10:borderright")]
pub struct RightBorder {
    /// Border Style
    /// Represents the following attribute in the schema: :type
    #[xml(attr = "type")]
    pub r#type: Option<BorderValues>,
    /// Border Width
    /// Represents the following attribute in the schema: :width
    #[xml(attr = "width")]
    pub width: Option<i32>,
    /// Border shadow
    /// Represents the following attribute in the schema: :shadow
    #[xml(attr = "shadow")]
    pub shadow: Option<bool>,
}
/// Bottom Border.
/// When the object is serialized out as xml, it's qualified name is w10:borderbottom.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w10:borderbottom")]
pub struct BottomBorder {
    /// Border Style
    /// Represents the following attribute in the schema: :type
    #[xml(attr = "type")]
    pub r#type: Option<BorderValues>,
    /// Border Width
    /// Represents the following attribute in the schema: :width
    #[xml(attr = "width")]
    pub width: Option<i32>,
    /// Border shadow
    /// Represents the following attribute in the schema: :shadow
    #[xml(attr = "shadow")]
    pub shadow: Option<bool>,
}
/// Defines the BorderType Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct BorderType {
    /// Border Style
    /// Represents the following attribute in the schema: :type
    #[xml(attr = "type")]
    pub r#type: Option<BorderValues>,
    /// Border Width
    /// Represents the following attribute in the schema: :width
    #[xml(attr = "width")]
    pub width: Option<i32>,
    /// Border shadow
    /// Represents the following attribute in the schema: :shadow
    #[xml(attr = "shadow")]
    pub shadow: Option<bool>,
}
/// Text Wrapping.
/// When the object is serialized out as xml, it's qualified name is w10:wrap.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w10:wrap")]
pub struct TextWrap {
    /// Wrapping type
    /// Represents the following attribute in the schema: :type
    #[xml(attr = "type")]
    pub r#type: Option<WrapValues>,
    /// Wrapping side
    /// Represents the following attribute in the schema: :side
    #[xml(attr = "side")]
    pub side: Option<WrapSideValues>,
    /// Horizontal Positioning Base
    /// Represents the following attribute in the schema: :anchorx
    #[xml(attr = "anchorx")]
    pub anchor_x: Option<HorizontalAnchorValues>,
    /// Vertical Positioning Base
    /// Represents the following attribute in the schema: :anchory
    #[xml(attr = "anchory")]
    pub anchor_y: Option<VerticalAnchorValues>,
}
/// Anchor Location Is Locked.
/// When the object is serialized out as xml, it's qualified name is w10:anchorlock.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w10:anchorlock")]
pub struct AnchorLock {}
