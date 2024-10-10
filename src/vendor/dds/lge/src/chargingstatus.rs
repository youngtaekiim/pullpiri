// SPDX-License-Identifier: Apache-2.0

// TOPIC NAME = /rt/piccolo/Charging_Status
#[allow(non_snake_case)]
pub mod ChargingStatus{
    #[derive(Debug, dust_dds::topic_definition::type_support::DdsType)]
    pub struct ChargingStatusMsg {
        pub state:String,
    }
}

