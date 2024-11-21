#[derive(Debug, dust_dds::topic_definition::type_support::DdsType)]
pub struct PowertrainBatteryChargingChargePortFlapControl {
    pub command: i32,
}
#[derive(Debug, dust_dds::topic_definition::type_support::DdsType)]
pub struct PowertrainBatteryChargingChargePortFlapStatus {
    pub command: i32,
    pub status: i32,
    pub progress: i32,
    pub uistatus: i32,
}
#[derive(Debug, dust_dds::topic_definition::type_support::DdsType)]
pub struct PowertrainBatteryStateOfChargeCurrent {
    pub value: f32,
}
#[derive(Debug, dust_dds::topic_definition::type_support::DdsType)]
pub struct PowertrainBatteryRange {
    pub value: i32,
}
