pub mod checker;

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
}
