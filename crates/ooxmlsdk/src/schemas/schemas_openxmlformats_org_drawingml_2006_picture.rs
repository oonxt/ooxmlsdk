/// Picture.
/// When the object is serialized out as xml, it's qualified name is pic:pic.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "pic:pic")]
pub struct Picture {
    #[xml(attr = "xmlns", with = "picture_xmlns_derive")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    ///Non-Visual Picture Properties
    #[xml(child = "pic:nvPicPr")]
    pub non_visual_picture_properties: NonVisualPictureProperties,
    ///Picture Fill
    #[xml(child = "pic:blipFill")]
    pub blip_fill: BlipFill,
    ///Shape Properties
    #[xml(child = "pic:spPr")]
    pub shape_properties: ShapeProperties,
    /// _
    #[xml(child = "pic14:style")]
    pub shape_style: Option<
        crate::schemas::schemas_microsoft_com_office_drawing_2010_picture::ShapeStyle,
    >,
    /// _
    #[xml(child = "pic14:extLst")]
    pub office_art_extension_list: Option<
        crate::schemas::schemas_microsoft_com_office_drawing_2010_picture::OfficeArtExtensionList,
    >,
}
mod picture_xmlns_derive {
    pub fn from_xml(mode: &str) -> hard_xml::XmlResult<String> {
        Ok(mode.to_string())
    }
    pub fn to_xml(_: &String) -> hard_xml::XmlResult<&'static str> {
        Ok("http://schemas.openxmlformats.org/drawingml/2006/picture")
    }
}
/// Non-Visual Drawing Properties.
/// When the object is serialized out as xml, it's qualified name is pic:cNvPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "pic:cNvPr")]
pub struct NonVisualDrawingProperties {
    #[xml(attr = "xmlns", with = "non_visual_drawing_properties_xmlns_derive")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
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
mod non_visual_drawing_properties_xmlns_derive {
    pub fn from_xml(mode: &str) -> hard_xml::XmlResult<String> {
        Ok(mode.to_string())
    }
    pub fn to_xml(_: &String) -> hard_xml::XmlResult<&'static str> {
        Ok("http://schemas.openxmlformats.org/drawingml/2006/picture")
    }
}
/// Non-Visual Picture Drawing Properties.
/// When the object is serialized out as xml, it's qualified name is pic:cNvPicPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "pic:cNvPicPr")]
pub struct NonVisualPictureDrawingProperties {
    #[xml(attr = "xmlns", with = "non_visual_picture_drawing_properties_xmlns_derive")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// preferRelativeResize
    /// Represents the following attribute in the schema: :preferRelativeResize
    #[xml(attr = "preferRelativeResize")]
    pub prefer_relative_resize: Option<bool>,
    /// _
    #[xml(child = "a:picLocks")]
    pub picture_locks: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::PictureLocks,
    >,
    /// _
    #[xml(child = "a:extLst")]
    pub non_visual_picture_properties_extension_list: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::NonVisualPicturePropertiesExtensionList,
    >,
}
mod non_visual_picture_drawing_properties_xmlns_derive {
    pub fn from_xml(mode: &str) -> hard_xml::XmlResult<String> {
        Ok(mode.to_string())
    }
    pub fn to_xml(_: &String) -> hard_xml::XmlResult<&'static str> {
        Ok("http://schemas.openxmlformats.org/drawingml/2006/picture")
    }
}
/// Non-Visual Picture Properties.
/// When the object is serialized out as xml, it's qualified name is pic:nvPicPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "pic:nvPicPr")]
pub struct NonVisualPictureProperties {
    #[xml(attr = "xmlns", with = "non_visual_picture_properties_xmlns_derive")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    ///Non-Visual Drawing Properties
    #[xml(child = "pic:cNvPr")]
    pub non_visual_drawing_properties: NonVisualDrawingProperties,
    ///Non-Visual Picture Drawing Properties
    #[xml(child = "pic:cNvPicPr")]
    pub non_visual_picture_drawing_properties: NonVisualPictureDrawingProperties,
}
mod non_visual_picture_properties_xmlns_derive {
    pub fn from_xml(mode: &str) -> hard_xml::XmlResult<String> {
        Ok(mode.to_string())
    }
    pub fn to_xml(_: &String) -> hard_xml::XmlResult<&'static str> {
        Ok("http://schemas.openxmlformats.org/drawingml/2006/picture")
    }
}
/// Picture Fill.
/// When the object is serialized out as xml, it's qualified name is pic:blipFill.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "pic:blipFill")]
pub struct BlipFill {
    #[xml(attr = "xmlns", with = "blip_fill_xmlns_derive")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// DPI Setting
    /// Represents the following attribute in the schema: :dpi
    #[xml(attr = "dpi")]
    pub dpi: Option<u32>,
    /// Rotate With Shape
    /// Represents the following attribute in the schema: :rotWithShape
    #[xml(attr = "rotWithShape")]
    pub rotate_with_shape: Option<bool>,
    #[xml(child = "a:blip", child = "a:srcRect", child = "a:tile", child = "a:stretch")]
    pub children: Vec<BlipFillChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum BlipFillChildChoice {
    #[xml(tag = "a:blip")]
    ABlip(crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Blip),
    #[xml(tag = "a:srcRect")]
    ASrcRect(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::SourceRectangle,
    ),
    #[xml(tag = "a:tile")]
    ATile(crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Tile),
    #[xml(tag = "a:stretch")]
    AStretch(crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Stretch),
}
mod blip_fill_xmlns_derive {
    pub fn from_xml(mode: &str) -> hard_xml::XmlResult<String> {
        Ok(mode.to_string())
    }
    pub fn to_xml(_: &String) -> hard_xml::XmlResult<&'static str> {
        Ok("http://schemas.openxmlformats.org/drawingml/2006/picture")
    }
}
/// Shape Properties.
/// When the object is serialized out as xml, it's qualified name is pic:spPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "pic:spPr")]
pub struct ShapeProperties {
    #[xml(attr = "xmlns", with = "shape_properties_xmlns_derive")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
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
mod shape_properties_xmlns_derive {
    pub fn from_xml(mode: &str) -> hard_xml::XmlResult<String> {
        Ok(mode.to_string())
    }
    pub fn to_xml(_: &String) -> hard_xml::XmlResult<&'static str> {
        Ok("http://schemas.openxmlformats.org/drawingml/2006/picture")
    }
}
