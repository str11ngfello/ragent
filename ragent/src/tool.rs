use serde_json::Value;

pub trait Tool {
    const NAME: &'static str;
    const DESCRIPTION: &'static str;

    type Error;
    type Args;
    type Output;

    fn name() -> String {
        Self::NAME.to_string()
    }

    fn description() -> String {
        Self::DESCRIPTION.to_string()
    }

    fn schema(&self) -> Option<Value>;
    fn schema_stringify(&self) -> Option<String> {
        if let Some(schema) = self.schema() {
            Some(schema.to_string())
        } else {
            None
        }
    }

    fn run(&self, args: Self::Args) -> Result<Self::Output, Box<Self::Error>>;
}
