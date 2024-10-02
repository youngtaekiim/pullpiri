// TOPIC NAME = /rt/piccolo/Day_Time
#[allow(non_snake_case)]
pub mod DayTime {
    #[derive(Debug, dust_dds::topic_definition::type_support::DdsType)]
    pub struct DataType {
        pub day: bool,
    }
}
