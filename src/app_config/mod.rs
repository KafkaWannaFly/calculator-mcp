use config::{Config, ConfigError, Environment, File};
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub http_server: HttpServer,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HttpServer {
    pub port: u16,
}

impl AppConfig {
    /// Load configuration from a specific TOML file path.
    /// Environment variables take priority over the file.
    pub fn new_from_file(file_path: &str) -> Result<Self, ConfigError> {
        let config = Config::builder()
            .add_source(File::from(Path::new(file_path)))
            .add_source(
                Environment::with_prefix("APP")
                    .separator("__")
                    .try_parsing(true),
            )
            .build()?;

        config.try_deserialize()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// RAII guard that automatically removes an environment variable when dropped
    struct EnvGuard {
        key: String,
    }

    impl EnvGuard {
        fn new(key: &str, value: &str) -> Self {
            unsafe {
                std::env::set_var(key, value);
            }
            EnvGuard {
                key: key.to_string(),
            }
        }
    }

    impl Drop for EnvGuard {
        fn drop(&mut self) {
            unsafe {
                std::env::remove_var(&self.key);
            }
        }
    }

    #[test]
    fn test_load_config_from_file() {
        let config = AppConfig::new_from_file("config.toml")
            .expect("Failed to load config from config.toml");

        assert_eq!(config.http_server.port, 8080);
    }

    #[test]
    #[serial_test::serial]
    fn test_env_var_overrides_file() {
        let _guard = EnvGuard::new("APP__HTTP_SERVER__PORT", "9090");

        let config = AppConfig::new_from_file("config.toml")
            .expect("Failed to load config from config.toml");

        assert_eq!(config.http_server.port, 9090);
    }

    #[test]
    #[serial_test::serial]
    fn test_env_var_with_invalid_port() {
        let _guard = EnvGuard::new("APP__HTTP_SERVER__PORT", "invalid");

        let result = AppConfig::new_from_file("config.toml");

        assert!(result.is_err());
    }

    #[test]
    #[serial_test::serial]
    fn test_env_var_multiple_calls() {
        unsafe {
            std::env::remove_var("APP__HTTP_SERVER__PORT");
        }
        let config1 = AppConfig::new_from_file("config.toml")
            .expect("Failed to load config from config.toml");
        assert_eq!(config1.http_server.port, 8080);

        let _guard = EnvGuard::new("APP__HTTP_SERVER__PORT", "5000");
        let config2 = AppConfig::new_from_file("config.toml")
            .expect("Failed to load config from config.toml");
        assert_eq!(config2.http_server.port, 5000);
    }
}
