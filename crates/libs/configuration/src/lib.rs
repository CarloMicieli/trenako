//! the service configuration settings

use config::{Config, Environment, File};
use secrecy::{ExposeSecret, SecretString};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use sqlx::postgres::{PgConnectOptions, PgPoolOptions, PgSslMode};

/// It represents the settings for the service
#[derive(Serialize, Deserialize, Debug)]
pub struct Settings {
    /// the database settings
    pub database: DatabaseSettings,
    /// the HTTP server settings
    pub server: ServerSettings,
    /// the logging and tracing settings
    pub logging: LoggingSettings,
}

impl Settings {
    /// Returns the server address (host and port)
    pub fn address(&self) -> String {
        format!("{}:{}", self.server.host, self.server.port)
    }

    /// Returns the postgres connection options
    pub fn pg_connection_options(&self) -> PgConnectOptions {
        self.database.pg_connection_options()
    }

    /// Load the settings from the configuration file (configuration/application.yaml)
    /// and environment variables.
    pub fn load() -> Result<Settings, config::ConfigError> {
        Self::load_from_path("config/application")
    }

    /// Load the settings from the configuration file and environment variables.
    pub fn load_from_path(config_file: &str) -> Result<Settings, config::ConfigError> {
        let s = Config::builder()
            .add_source(File::with_name(config_file).required(false))
            .add_source(Environment::default().separator("__").ignore_empty(true))
            .build()?;
        s.try_deserialize()
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub enum LoggingFormat {
    Full,
    Compact,
    Pretty,
    Json,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub enum LoggingLevel {
    Debug,
    Error,
    Info,
    Warn,
    Trace,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LoggingSettings {
    /// the logging format
    pub format: LoggingFormat,
    pub level: LoggingLevel,
}

/// It contains the server configuration
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ServerSettings {
    /// the server host name
    pub host: String,
    /// the server port number
    pub port: u16,
}

/// It contains the database connection settings
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DatabaseSettings {
    /// the username
    pub username: String,
    /// the password
    #[serde(skip_serializing)]
    pub password: SecretString,
    /// the host name
    pub host: String,
    /// the port number
    pub port: u16,
    /// the database name
    pub name: String,
    /// the connection pool min number of connections
    pub min_connections: u32,
    /// the connection pool max number of connections
    pub max_connections: u32,
    /// the SSL mode for the connection
    pub require_ssl: bool,
}

impl DatabaseSettings {
    /// Creates the database settings
    pub fn new(username: &str, password: &str, host: &str, port: u16, name: &str) -> DatabaseSettings {
        DatabaseSettings {
            username: username.to_owned(),
            password: SecretString::from(password.to_owned()),
            host: host.to_owned(),
            port,
            name: name.to_owned(),
            min_connections: 5,
            max_connections: 10,
            require_ssl: false,
        }
    }

    /// Creates a new postgres connection pool using the database connection settings.
    pub fn get_connection_pool(&self) -> PgPool {
        PgPoolOptions::new()
            .min_connections(self.min_connections)
            .max_connections(self.max_connections)
            .acquire_timeout(std::time::Duration::from_secs(2))
            .connect_lazy_with(self.pg_connection_options())
    }

    /// Returns the postgres connection options
    pub fn pg_connection_options(&self) -> PgConnectOptions {
        let ssl_mode = if self.require_ssl {
            PgSslMode::Require
        } else {
            PgSslMode::Prefer
        };

        PgConnectOptions::new()
            .application_name("trenako")
            .host(&self.host)
            .database(&self.name)
            .username(&self.username)
            .password(self.password.expose_secret())
            .port(self.port)
            .ssl_mode(ssl_mode)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod settings {
        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn it_should_return_the_server_address() {
            let settings = Settings {
                database: DatabaseSettings::new("postgres", "pa$$word", "database-host", 5432, "database-name"),
                server: ServerSettings {
                    host: String::from("127.0.0.1"),
                    port: 8080,
                },
                logging: LoggingSettings {
                    level: LoggingLevel::Debug,
                    format: LoggingFormat::Full,
                },
            };

            assert_eq!("127.0.0.1:8080", settings.address());
        }

        #[test]
        fn it_should_build_the_pg_connection_options() {
            let settings = Settings {
                database: DatabaseSettings::new("postgres", "pa$$word", "database-host", 5432, "database-name"),
                server: ServerSettings {
                    host: String::from("127.0.0.1"),
                    port: 8080,
                },
                logging: LoggingSettings {
                    level: LoggingLevel::Debug,
                    format: LoggingFormat::Full,
                },
            };

            let pg_connection_options = settings.pg_connection_options();
            assert_eq!(Some("database-name"), pg_connection_options.get_database());
        }
    }
}
