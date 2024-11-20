#[derive(Debug, dust_dds::topic_definition::type_support::DdsType)]
pub struct BodyTrunkControl {
    pub command: i32,
}
#[derive(Debug, dust_dds::topic_definition::type_support::DdsType)]
pub struct BodyTrunkStatus {
    pub command: i32,
    pub status: i32,
    pub progress: i32,
    pub uistatus: i32,
}
