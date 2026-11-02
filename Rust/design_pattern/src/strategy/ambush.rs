use super::hunt_strategy::HuntStrategy;

// The concrete strategy
pub struct Ambush;

impl HuntStrategy for Ambush {
    fn execute(&self) -> &str {
        "ambushing"
    }
}
