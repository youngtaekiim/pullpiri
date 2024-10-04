use crate::listener::DdsData;
use std::str::FromStr;

#[derive(Debug)]
pub struct Filter {
    pub name: String,
    pub express: String,
    pub target_value: String,
    pub topic: String,
    pub action_key: String,
}

impl Filter {
    pub async fn new(name: &str) -> Self {
        let action_key = name.to_string();
        let conditions = common::etcd::get(&format!("{name}/conditions"))
            .await
            .unwrap();
        let condition: common::spec::scenario::Condition =
            serde_yaml::from_str(&conditions).unwrap();

        Filter {
            name: name.to_string(),
            express: condition.get_express(),
            target_value: condition.get_value(),
            topic: condition.get_operand_value(),
            action_key,
        }
    }

    pub async fn check(&mut self, data: DdsData) -> bool {
        println!("{} {}", data.name, data.value);
        println!(
            "{} {} {} {} {}",
            self.name, self.express, self.target_value, self.topic, self.action_key
        );

        let topic = self.topic.clone();

        match data.name.as_str() {
            topic => {
                match self.express.as_str() {
                    "lt" => {
                        let target_v = f32::from_str(self.target_value).unwrap;
                        let current_v = f32::from_str(data.value).unwrap;
                        if target_v < current_v {
                            true
                        }else{
                            false
                        }
                    }
                    "le" => {
                        let target_v = f32::from_str(self.target_value).unwrap;
                        let current_v = f32::from_str(data.value).unwrap;
                        if target_v <= current_v {
                            true
                        }else{
                            false
                        }
                    }
                    "eq" => {
                        if self.target_value.eq(data.value) {
                            true
                        }else{
                            false
                        }
                    }
                    "ge" => {
                        let target_v = f32::from_str(self.target_value).unwrap;
                        let current_v = f32::from_str(data.value).unwrap;
                        if target_v >= current_v {
                            true
                        }else{
                            false
                        }
                    }
                    "gt" => {
                        let target_v = f32::from_str(self.target_value).unwrap;
                        let current_v = f32::from_str(data.value).unwrap;
                        if target_v > current_v {
                            true
                        }else{
                            false
                        }
                    }
                    _ => false
                }
            }
        }
    }
}
