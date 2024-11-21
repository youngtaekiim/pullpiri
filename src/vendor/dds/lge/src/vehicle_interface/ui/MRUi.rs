#[derive(Debug, dust_dds::topic_definition::type_support::DdsType)]
pub struct MRUiControl {
    pub command: i32,
}
#[derive(Debug, dust_dds::topic_definition::type_support::DdsType)]
pub struct MRUiStatus {
    pub command: i32,
    pub status: i32,
}
