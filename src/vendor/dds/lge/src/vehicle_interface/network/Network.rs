#[derive(Debug, dust_dds::topic_definition::type_support::DdsType)]
pub struct NodeTimeOffset {
    pub deviceId: i32,
    pub offset: i32,
}
#[derive(Debug, dust_dds::topic_definition::type_support::DdsType)]
pub struct NodeDisconnectionStatus {
    pub value: bool,
}
