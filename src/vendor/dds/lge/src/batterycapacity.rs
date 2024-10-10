// SPDX-License-Identifier: Apache-2.0

// TOPIC NAME = /rt/piccolo/Battery_Capacity
#[allow(non_snake_case)]
pub mod BatteryCapacity{
    #[derive(Debug, dust_dds::topic_definition::type_support::DdsType)]
    pub struct BatteryCapacityMsg {
        pub capacity:String,
    }
}
