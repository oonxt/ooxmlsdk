#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum ExtendedBrushPropertyName {
    #[default]
    InkEffects,
    AnchorX,
    AnchorY,
    ScaleFactor,
}
crate::__string_enum! {
    ExtendedBrushPropertyName { InkEffects = "inkEffects", AnchorX = "anchorX", AnchorY =
    "anchorY", ScaleFactor = "scaleFactor", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum InkEffectsType {
    #[default]
    None,
    Pencil,
    Rainbow,
    Galaxy,
    Gold,
    Silver,
    Lava,
    Ocean,
    Rosegold,
    Bronze,
}
crate::__string_enum! {
    InkEffectsType { None = "none", Pencil = "pencil", Rainbow = "rainbow", Galaxy =
    "galaxy", Gold = "gold", Silver = "silver", Lava = "lava", Ocean = "ocean", Rosegold
    = "rosegold", Bronze = "bronze", }
}
