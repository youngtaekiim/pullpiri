pub mod adas;
pub mod body;
pub mod cabin;
pub mod exterior;
pub mod network;
pub mod powertrain;
pub mod ui;

use crate::{DdsData, Piccoloable};
use dust_dds::{
    configuration::DustDdsConfigurationBuilder, dds_async::domain_participant_factory::DomainParticipantFactoryAsync, infrastructure::{error::DdsResult, qos::QosKind, status::NO_STATUS}, subscription::sample_info::{ANY_INSTANCE_STATE, ANY_SAMPLE_STATE, ANY_VIEW_STATE}, topic_definition::type_support::{DdsDeserialize, DdsHasKey, DdsKey, DdsTypeXml}
};
use tokio::sync::mpsc::Sender;

pub async fn receive_dds<
    T: Piccoloable + DdsKey + DdsHasKey + DdsTypeXml + for<'a> DdsDeserialize<'a> + 'static,
>(
    tx: Sender<DdsData>,
) {
    let (topic_name, type_name, domain_id) = (T::topic_name(), T::type_name(), 0);
    let configuration= DustDdsConfigurationBuilder::new().interface_name(Some(String::from("acrn-br0"))).build().unwrap();
    let participant_factory = DomainParticipantFactoryAsync::new();
    
    participant_factory.set_configuration(configuration).await.unwrap();

    let participant = participant_factory
        .create_participant(domain_id, QosKind::Default, None, NO_STATUS)
        .await
        .unwrap();

    let subscriber = participant
        .create_subscriber(QosKind::Default, None, NO_STATUS)
        .await
        .unwrap();

    let topic = participant
        .create_topic::<T>(&topic_name, &type_name, QosKind::Default, None, NO_STATUS)
        .await
        .unwrap();
    let reader = subscriber
        .create_datareader::<T>(&topic, QosKind::Default, None, NO_STATUS)
        .await
        .unwrap();

    println!("make loop - {topic_name}");
    loop {
        if let Ok(data_samples) = reader
            .take(50, ANY_SAMPLE_STATE, ANY_VIEW_STATE, ANY_INSTANCE_STATE)
            .await
        {
            let data: DdsResult<T> = data_samples[0].data();
            if data.is_err() {
                continue;
            }
            let msg = data.unwrap().to_piccolo_dds_data();
            println!("Received: {:?}", msg);
            let _ = tx.send(msg).await;
        }
        std::thread::sleep(std::time::Duration::from_millis(500));
    }
}

/* vehicle-interface-v0.3 DDS topic list
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
        pub const CURRENT_GEAR: &str = "PowertrainTransmissionCurrentGear";
        pub const MR_UI_CTRL: &str = "MRUiControl";
        pub const TIME_OFFSET: &str = "NetworkTimesyncStatus";
        pub const NODE_DISCONNECT: &str = "NetworkNodeDIsocnnnectionStatus";
    }

    pub enum CurrentUsedTopic {
        BATTERY_COVER_STAT,
        CURRENT_GEAR,
        HEADLAMP_STAT,
        PHOTO_RESISTOR,
        LWINDOW_STAT,
        RWINDOW_STAT,
        LDOOR_STAT,
        RDOOR_STAT,
        TRUNK_STAT,
        OBSTACLE_DETECT,
    }
*/
