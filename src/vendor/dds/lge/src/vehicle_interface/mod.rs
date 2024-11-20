pub mod adas;
pub mod body;
pub mod cabin;
pub mod exterior;
pub mod network;
pub mod powertrain;
pub mod ui;

pub mod topic {
    pub const OBSTACLE_DETECT: &str = "ADASObstacleDetectionIsEnabled";
    pub const HEADLAMP_CTRL: &str = "BodyLightsHeadLampControl";
    pub const HEADLAMP_STAT: &str = "BodyLightsHeadLampStatus";
    pub const TRUNK_CTRL: &str = "BodyTrunkControl";
    pub const TRUNK_STAT: &str = "BodyTrunkStatus";
    pub const LDOOR_CTRL: &str = "CabinLeftDoorControl";
    pub const LDOOR_STAT: &str = "CabinLeftDoorStatus";
    pub const RDOOR_CTRL: &str = "CabinRightDoorControl";
    pub const RDOOR_STAT: &str = "CabinRightDoorStatus";
    pub const LWINDOW_CTRL: &str = "CabinLeftWindowControl";
    pub const LWINDOW_STAT: &str = "CabinLeftWindowStatus";
    pub const RWINDOW_CTRL: &str = "CabinRightWindowControl";
    pub const RWINDOW_STAT: &str = "CabinRightWindowStatus";
    pub const PHOTO_RESISTOR: &str = "ExteriorLightIntensity";
    pub const BATTERY_COVER_CTRL: &str = "PowerTrainBatteryChargingChargePortFlapControl";
    pub const BATTERY_COVER_STAT: &str = "PowerTrainBatteryChargingChargePortFlapStatus";
    pub const MISSION_CURRENT: &str = "PowertrainTransmissionCurrentGear";
    pub const MR_UI_CTRL: &str = "MRUiControl";
    pub const TIME_OFFSET: &str = "NetworkTimesyncStatus";
    pub const NODE_DISCONNECT: &str = "NetworkNodeDIsocnnnectionStatus";
}
