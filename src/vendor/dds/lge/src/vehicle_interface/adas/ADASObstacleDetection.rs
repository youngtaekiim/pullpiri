#[derive(Debug, dust_dds::topic_definition::type_support::DdsType)]
pub struct ADASObstacleDetectionIsEnabled {
    pub value: bool,
}

impl crate::Piccoloable for ADASObstacleDetectionIsEnabled {
    fn to_piccolo_dds_data(&self) -> crate::DdsData {
        crate::DdsData {
            name: String::from("ADASObstacleDetectionIsEnabled"),
            value: self.value.to_string(),
        }
    }
    fn topic_name() -> String {
        String::from("ADASObstacleDetectionIsEnabled")
    }
    fn type_name() -> String {
        String::from("ADASObstacleDetectionIsEnabled")
    }
}
