pub trait Describable {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
}
