use testcontainers::core::{Image, WaitFor};

pub struct PostgresImage;

impl Default for PostgresImage {
    fn default() -> Self {
        PostgresImage
    }
}

impl Image for PostgresImage {
    type Args = ();

    fn name(&self) -> String {
        "postgres".to_string()
    }

    fn tag(&self) -> String {
        "latest".to_string()
    }

    fn ready_conditions(&self) -> Vec<WaitFor> {
        vec![] // Define conditions to wait for, e.g., WaitFor::LogMessage { message: "Ready".to_string() }
    }
}
