use serde::Deserialize;

/// Application configuration loaded from environment variables.
///
/// # Fields
/// - `reverse_proxy_target_url` - The target URL for the reverse proxy,
#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub reverse_proxy_target_url: String, // REVERSE_PROXY_TARGET_URL
}

impl Config {
    /// Loads the configuration from environment variables using the `envy`
    /// crate.
    ///
    /// # Returns
    /// - `Ok(Config)` if the environment variables are successfully parsed into
    ///   a `Config` struct.
    /// - `Err(envy::Error)` if there is an error during parsing.
    ///
    /// # Panics
    /// This function will panic if the configuration cannot be loaded, printing
    /// the error details. ```
    pub fn from_env() -> Result<Self, envy::Error> {
        match envy::from_env::<Config>() {
            Ok(config) => {
                println!("Configuration loaded: {:#?}", config);
                Ok(config)
            }
            Err(error) => panic!("{:#?}", error),
        }
    }
}
