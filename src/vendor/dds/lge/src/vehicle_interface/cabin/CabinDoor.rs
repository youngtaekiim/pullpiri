#[derive(Debug, dust_dds::topic_definition::type_support::DdsType)]
pub struct CabinLeftDoorControl {
    pub command: i32,
}
#[derive(Debug, dust_dds::topic_definition::type_support::DdsType)]
pub struct CabinLeftDoorStatus {
    pub command: i32,
    pub status: i32,
    pub progress: i32,
    pub uistatus: i32,
}
#[derive(Debug, dust_dds::topic_definition::type_support::DdsType)]
pub struct CabinRightDoorControl {
    pub command: i32,
}
#[derive(Debug, dust_dds::topic_definition::type_support::DdsType)]
pub struct CabinRightDoorStatus {
    pub command: i32,
    pub status: i32,
    pub progress: i32,
    pub uistatus: i32,
}
