use super::hunt_strategy::HuntStrategy;

// The concrete strategy
pub struct Chase;

impl HuntStrategy for Chase {
    fn execute(&self) -> &str {
        "chasing"
    }
}
