// The strategy
pub trait HuntStrategy {
    fn execute(&self) -> &str;
}
