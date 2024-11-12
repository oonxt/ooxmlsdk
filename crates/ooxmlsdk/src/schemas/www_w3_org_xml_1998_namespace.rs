#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum SpaceProcessingModeValues {
    #[default]
    Default,
    Preserve,
}
crate::__string_enum! {
    SpaceProcessingModeValues { Default = "default", Preserve = "preserve", }
}
