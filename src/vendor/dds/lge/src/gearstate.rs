// TOPIC NAME = /rt/piccolo/Gear_State
#[allow(non_snake_case)]
pub mod GearState {
    #[derive(Debug, dust_dds::topic_definition::type_support::DdsType)]
    pub struct DataType {
        pub state: String,
    }
}