// SPDX-License-Identifier: Apache-2.0

// TOPIC NAME = /rt/piccolo/Day_Time
pub mod DayTime {
    #[derive(Debug, dust_dds::topic_definition::type_support::DdsType)]
    pub struct DataType {
        pub day: bool,
    }
}
