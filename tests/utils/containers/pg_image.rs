use std::collections::HashMap;
use testcontainers::core::{Image, WaitFor};

const NAME: &str = "postgres";
const TAG: &str = "16-alpine";

#[derive(Debug)]
pub struct PostgresImage {
    env_vars: HashMap<String, String>,
}

impl Default for PostgresImage {
    fn default() -> Self {
        let mut env_vars = HashMap::new();
        env_vars.insert("POSTGRES_USER".to_owned(), "test_user".to_owned());
        env_vars.insert("POSTGRES_PASSWORD".to_owned(), "test_password".to_owned());
        env_vars.insert("POSTGRES_DB".to_owned(), "postgres".to_owned());
        env_vars.insert("POSTGRES_HOST_AUTH_METHOD".into(), "trust".into());

        Self { env_vars }
    }
}

impl Image for PostgresImage {
    type Args = ();

    fn name(&self) -> String {
        NAME.to_owned()
    }

    fn tag(&self) -> String {
        TAG.to_owned()
    }

    fn ready_conditions(&self) -> Vec<WaitFor> {
        vec![
            WaitFor::StdOutMessage {
                message: "database system is ready to accept connections".to_string(),
            },
            WaitFor::seconds(1),
        ]
    }

    fn env_vars(&self) -> Box<dyn Iterator<Item = (&String, &String)> + '_> {
        Box::new(self.env_vars.iter())
    }
}
