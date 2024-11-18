#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application_port: u16,
}

#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    // Initialize our configuration reader
    let mut settings = config::Config::default();

    // Add configuration values from a file named `configuration`
    // It will look for any top-level file with an expression
    // that `config` knows how to pars: yaml, json, etc
    settings.merge(config::File::with_name("configuration"))?;

    // Try to convert the configuration values it read into
    // our Settings type
    settings.try_into()
}
