use super::hunt_strategy::HuntStrategy;

// The context
pub struct Feline {
    name: String,
    strategy: Box<dyn HuntStrategy>,
}

impl Feline {
    pub fn new(name: impl Into<String>, strategy: Box<dyn HuntStrategy>) -> Self {
        Feline {
            name: name.into(),
            strategy,
        }
    }

    pub fn hunt(&self, prey: &str) -> String {
        format!(
            "The {} is {} the {}",
            self.name,
            self.strategy.execute(),
            prey
        )
    }

    pub fn set_strategy(&mut self, strategy: Box<dyn HuntStrategy>) {
        self.strategy = strategy;
    }
}
