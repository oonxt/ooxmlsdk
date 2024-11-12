#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum SizeRelativeHorizontallyValues {
    #[default]
    Margin,
    Page,
    LeftMargin,
    RightMargin,
    InsideMargin,
    OutsideMargin,
}
crate::__string_enum! {
    SizeRelativeHorizontallyValues { Margin = "margin", Page = "page", LeftMargin =
    "leftMargin", RightMargin = "rightMargin", InsideMargin = "insideMargin",
    OutsideMargin = "outsideMargin", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum SizeRelativeVerticallyValues {
    #[default]
    Margin,
    Page,
    TopMargin,
    BottomMargin,
    InsideMargin,
    OutsideMargin,
}
crate::__string_enum! {
    SizeRelativeVerticallyValues { Margin = "margin", Page = "page", TopMargin =
    "topMargin", BottomMargin = "bottomMargin", InsideMargin = "insideMargin",
    OutsideMargin = "outsideMargin", }
}
/// Defines the PercentagePositionHeightOffset Class.
/// When the object is serialized out as xml, it's qualified name is wp14:pctPosHOffset.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "wp14:pctPosHOffset")]
pub struct PercentagePositionHeightOffset {
    #[xml(text)]
    pub child: i32,
}
/// Defines the PercentagePositionVerticalOffset Class.
/// When the object is serialized out as xml, it's qualified name is wp14:pctPosVOffset.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "wp14:pctPosVOffset")]
pub struct PercentagePositionVerticalOffset {
    #[xml(text)]
    pub child: i32,
}
/// Defines the RelativeWidth Class.
/// When the object is serialized out as xml, it's qualified name is wp14:sizeRelH.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "wp14:sizeRelH")]
pub struct RelativeWidth {
    /// relativeFrom
    /// Represents the following attribute in the schema: :relativeFrom
    #[xml(attr = "relativeFrom")]
    pub object_id: SizeRelativeHorizontallyValues,
    /// _
    #[xml(child = "wp14:pctWidth")]
    pub percentage_width: PercentageWidth,
}
/// Defines the RelativeHeight Class.
/// When the object is serialized out as xml, it's qualified name is wp14:sizeRelV.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "wp14:sizeRelV")]
pub struct RelativeHeight {
    /// relativeFrom
    /// Represents the following attribute in the schema: :relativeFrom
    #[xml(attr = "relativeFrom")]
    pub relative_from: SizeRelativeVerticallyValues,
    /// _
    #[xml(child = "wp14:pctHeight")]
    pub percentage_height: PercentageHeight,
}
/// Defines the PercentageWidth Class.
/// When the object is serialized out as xml, it's qualified name is wp14:pctWidth.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "wp14:pctWidth")]
pub struct PercentageWidth {
    #[xml(text)]
    pub child: i32,
}
/// Defines the PercentageHeight Class.
/// When the object is serialized out as xml, it's qualified name is wp14:pctHeight.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "wp14:pctHeight")]
pub struct PercentageHeight {
    #[xml(text)]
    pub child: i32,
}
