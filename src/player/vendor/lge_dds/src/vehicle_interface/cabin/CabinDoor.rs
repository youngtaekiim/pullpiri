#[derive(Debug, dust_dds::topic_definition::type_support::DdsType)]
pub struct CabinLeftDoorStatus {
    pub command: i32,
    pub status: i32,
    pub progress: i32,
    pub uistatus: i32,
}

#[derive(Debug, dust_dds::topic_definition::type_support::DdsType)]
pub struct CabinRightDoorStatus {
    pub command: i32,
    pub status: i32,
    pub progress: i32,
    pub uistatus: i32,
}

impl crate::Piccoloable for CabinLeftDoorStatus {
    fn to_piccolo_dds_data(&self) -> crate::DdsData {
        let ldoor = match self.uistatus {
            // uistatus: 0 = IS_ACTING, 1 = CHARGING_PORT_IS_CLOSED, 2 = CHARGING_PORT_IS_OPENED
            0 => "is_acting",
            1 => "close",
            2 => "open",
            _ => "unknown",
        };

        crate::DdsData {
            name: String::from("CabinLeftDoorStatus"),
            value: ldoor.to_string(),
        }
    }
    fn topic_name() -> String {
        String::from("CabinLeftDoorStatus")
    }
    fn type_name() -> String {
        String::from("CabinLeftDoorStatus")
    }
}

impl crate::Piccoloable for CabinRightDoorStatus {
    fn to_piccolo_dds_data(&self) -> crate::DdsData {
        let rdoor = match self.uistatus {
            // uistatus: 0 = IS_ACTING, 1 = CHARGING_PORT_IS_CLOSED, 2 = CHARGING_PORT_IS_OPENED
            0 => "is_acting",
            1 => "close",
            2 => "open",
            _ => "unknown",
        };

        crate::DdsData {
            name: String::from("CabinRightDoorStatus"),
            value: rdoor.to_string(),
        }
    }
    fn topic_name() -> String {
        String::from("CabinRightDoorStatus")
    }
    fn type_name() -> String {
        String::from("CabinRightDoorStatus")
    }
}
