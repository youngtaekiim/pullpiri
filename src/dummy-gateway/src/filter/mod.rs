pub struct Filter {
    name: String,
    state: bool,
    gear_target: String,
    day_target: String,
    current_gear: String,
    current_day: String,
    policy: String,
}

impl Drop for Filter {
    fn drop(&mut self) {
        println!("filter {} is dropped", self.name);
    }
}

impl Filter {
    pub async fn new(
        name: &str,
        target_gear: &str,
        target_day: &str,
        current_gear: &str,
        current_day: &str,
        policy: &str,
    ) -> Self {
        let status = target_gear == current_gear && target_day == current_day;
        let state_value = if status { "ACTIVE" } else { "INACTIVE" };
        let _ = common::etcd::put(&format!("scenario/{}", name), state_value).await;

        Filter {
            name: name.to_string(),
            state: status,
            gear_target: target_gear.to_string(),
            day_target: target_day.to_string(),
            current_gear: current_gear.to_string(),
            current_day: current_day.to_string(),
            policy: policy.to_string(),
        }
    }

    pub async fn set_status(&mut self, kind: i32, value: &str) {
        if kind == 0 {
            self.current_gear = value.to_string();
        } else if kind == 1 {
            self.current_day = value.to_string();
        }

        let new_state = (self.day_target.is_empty() || self.current_day == self.day_target)
            && (self.gear_target.is_empty() || self.current_gear == self.gear_target);
        if self.state != new_state {
            println!("{} - Now policy is {}\n", self.name, new_state);
            self.state = new_state;

            let state_value = if new_state { "ACTIVE" } else { "INACTIVE" };
            let _ =
                common::etcd::put(&format!("scenario/{}", self.name.clone()), state_value).await;
        }
    }

    pub async fn receive_light(&mut self, value: &str) {
        if !self.state || self.policy == value {
            return;
        }

        println!("{} - policy is applied and light is {}. send TURN {} LIGHT msg\n", self.name, value, self.policy);
        let dds_sender = crate::sender::dds::DdsEventSender::new().await;
        if value == "OFF" {
            dds_sender.send("on").await;
        } else if value == "ON" {
            dds_sender.send("off").await;
        }
    }
}
