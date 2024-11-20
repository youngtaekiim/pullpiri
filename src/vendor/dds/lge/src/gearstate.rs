// SPDX-License-Identifier: Apache-2.0

// TOPIC NAME = /rt/piccolo/Gear_State
pub mod GearState {
    #[derive(Debug, dust_dds::topic_definition::type_support::DdsType)]
    pub struct DataType {
        pub state: String,
    }
}
