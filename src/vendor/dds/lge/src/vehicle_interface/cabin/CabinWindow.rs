#[derive(Debug, dust_dds::topic_definition::type_support::DdsType)]
pub struct CabinLeftWindowControl {
    pub command: i32,
}
#[derive(Debug, dust_dds::topic_definition::type_support::DdsType)]
pub struct CabinLeftWindowStatus {
    pub command: i32,
    pub status: i32,
    pub progress: i32,
    pub uistatus: i32,
}
#[derive(Debug, dust_dds::topic_definition::type_support::DdsType)]
pub struct CabinRightWindowControl {
    pub command: i32,
}
#[derive(Debug, dust_dds::topic_definition::type_support::DdsType)]
pub struct CabinRightWindowStatus {
    pub command: i32,
    pub status: i32,
    pub progress: i32,
    pub uistatus: i32,
}
