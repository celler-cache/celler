//! Client configurations.
//!
//! Configuration files are stored under `$XDG_CONFIG_HOME/celler/config.toml`.
//! We automatically write modified configurations back for a good end-user
//! experience (e.g., `celler login`).

use std::collections::HashMap;
use std::fs::{self, read_to_string, OpenOptions, Permissions};
use std::io::Write;
use std::mem::ManuallyDrop;
use std::ops::{Deref, DerefMut};
use std::os::unix::fs::{OpenOptionsExt, PermissionsExt};
use std::path::{Path, PathBuf};

use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use xdg::BaseDirectories;

use crate::cache::{CacheName, CacheRef, ServerName};

/// Application prefix in XDG base directories.
///
/// This will be concatenated into `$XDG_CONFIG_HOME/celler`.
const XDG_PREFIX: &str = "celler";

/// The permission the configuration file should have.
const FILE_MODE: u32 = 0o600;

/// Configuration loader.
#[derive(Debug)]
pub struct Config {
    /// Actual configuration data.
    data: ConfigData,

    /// Path to write modified configurations back to.
    path: Option<PathBuf>,
}

/// Client configurations.
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct ConfigData {
    /// The default server to connect to.
    #[serde(rename = "default-server")]
    pub default_server: Option<ServerName>,

    /// A set of remote servers and access credentials.
    #[serde(default = "HashMap::new")]
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub servers: HashMap<ServerName, ServerConfig>,
}

/// Configuration of a server.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ServerConfig {
    pub endpoint: String,
    #[serde(flatten)]
    pub token: Option<ServerTokenConfig>,
}

impl ServerConfig {
    pub fn token(&self) -> Result<Option<String>> {
        self.token.as_ref().map(|token| token.get()).transpose()
    }
}

/// Configured server token
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ServerTokenConfig {
    Raw {
        token: String,
    },
    File {
        #[serde(rename = "token-file")]
        token_file: String,
    },
}

impl ServerTokenConfig {
    /// Get the token either directly from the config or through the token file
    pub fn get(&self) -> Result<String> {
        match self {
            ServerTokenConfig::Raw { token } => Ok(token.clone()),
            ServerTokenConfig::File { token_file } => Ok(read_to_string(token_file)
                .map(|t| t.trim().to_string())
                .with_context(|| format!("Failed to read token from {token_file}"))?),
        }
    }
}

/// Wrapper that automatically saves the config once dropped.
pub struct ConfigWriteGuard<'a>(&'a mut Config);

impl Config {
    /// Loads the configuration from the system.
    pub fn load() -> Result<Self> {
        let path = get_config_path()
            .map_err(|e| {
                tracing::warn!("Could not get config path: {}", e);
                e
            })
            .ok();

        let data = ConfigData::load_from_path(path.as_ref())?;

        Ok(Self { data, path })
    }

    /// Returns a mutable reference to the configuration.
    pub fn as_mut(&mut self) -> ConfigWriteGuard<'_> {
        ConfigWriteGuard(self)
    }

    /// Saves the configuration back to the system.
    ///
    /// Fails if the configuration path could not be determined or the
    /// file could not be written.
    pub fn save(&self) -> Result<()> {
        let path = self
            .path
            .as_ref()
            .ok_or_else(|| anyhow!("Could not determine the configuration file path"))?;

        let serialized = toml::to_string(&self.data)?;

        Self::write(path, &serialized)
            .with_context(|| format!("Failed to write configuration to {}", path.display()))?;

        tracing::debug!("Saved modified configuration to {:?}", path);

        Ok(())
    }

    fn write(path: &Path, serialized: &str) -> Result<()> {
        // This isn't atomic, so some other process might chmod it
        // to something else before we write. We don't handle this case.
        if path.exists() {
            let permissions = Permissions::from_mode(FILE_MODE);
            fs::set_permissions(path, permissions)?;
        }

        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .mode(FILE_MODE)
            .open(path)?;

        file.write_all(serialized.as_bytes())?;

        Ok(())
    }
}

impl Deref for Config {
    type Target = ConfigData;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl ConfigData {
    fn load_from_path(path: Option<&PathBuf>) -> Result<Self> {
        if let Some(path) = path {
            if path.exists() {
                let contents = fs::read(path)?;
                let s = std::str::from_utf8(&contents)?;
                let data = toml::from_str(s)?;
                return Ok(data);
            }
        }

        Ok(ConfigData::default())
    }

    pub fn default_server(&self) -> Result<(&ServerName, &ServerConfig)> {
        if let Some(name) = &self.default_server {
            let config = self.servers.get(name).ok_or_else(|| {
                anyhow!(
                    "Configured default server \"{}\" does not exist",
                    name.as_str()
                )
            })?;
            Ok((name, config))
        } else if let Some((name, config)) = self.servers.iter().next() {
            Ok((name, config))
        } else {
            Err(anyhow!("No servers are available."))
        }
    }

    pub fn resolve_cache<'a>(
        &'a self,
        r: &'a CacheRef,
    ) -> Result<(&'a ServerName, &'a ServerConfig, &'a CacheName)> {
        match r {
            CacheRef::DefaultServer(cache) => {
                let (name, config) = self.default_server()?;
                Ok((name, config, cache))
            }
            CacheRef::ServerQualified(server, cache) => {
                let config = self
                    .servers
                    .get(server)
                    .ok_or_else(|| anyhow!("Server \"{}\" does not exist", server.as_str()))?;
                Ok((server, config, cache))
            }
        }
    }
}

impl ConfigWriteGuard<'_> {
    /// Saves the modified configuration, reporting any error to the caller.
    ///
    /// Dropping the guard also saves the configuration, but can only log
    /// failures. Use this when the command should fail if the configuration
    /// cannot be persisted.
    pub fn save(self) -> Result<()> {
        // Skip the save in `Drop` since we are saving explicitly.
        let guard = ManuallyDrop::new(self);
        guard.0.save()
    }
}

impl<'a> Deref for ConfigWriteGuard<'a> {
    type Target = ConfigData;

    fn deref(&self) -> &Self::Target {
        &self.0.data
    }
}

impl<'a> DerefMut for ConfigWriteGuard<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0.data
    }
}

impl<'a> Drop for ConfigWriteGuard<'a> {
    fn drop(&mut self) {
        if let Err(e) = self.0.save() {
            tracing::error!("Could not save modified configuration: {}", e);
        }
    }
}

fn get_config_path() -> Result<PathBuf> {
    let xdg_dirs = BaseDirectories::with_prefix(XDG_PREFIX);
    let config_path = xdg_dirs.place_config_file("config.toml")?;

    Ok(config_path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    /// Returns a fresh, empty scratch directory for a test.
    fn scratch_dir(name: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!(
            "celler-config-test-{}-{}",
            std::process::id(),
            name
        ));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        dir
    }

    fn config_with_path(path: Option<PathBuf>) -> Config {
        Config {
            data: ConfigData::default(),
            path,
        }
    }

    fn add_server(config: &mut ConfigData) {
        config.servers.insert(
            ServerName::from_str("test").unwrap(),
            ServerConfig {
                endpoint: "http://localhost:8080".to_string(),
                token: None,
            },
        );
    }

    #[test]
    fn test_save_without_path_fails() {
        let config = config_with_path(None);

        assert!(config.save().is_err());
    }

    #[test]
    fn test_guard_save_reports_write_error() {
        let dir = scratch_dir("write-error");
        // The parent directory doesn't exist, so the file can't be created.
        let path = dir.join("missing").join("config.toml");
        let mut config = config_with_path(Some(path.clone()));

        let mut config_m = config.as_mut();
        add_server(&mut config_m);
        let err = config_m.save().unwrap_err();

        assert!(format!("{err}").contains(&path.display().to_string()));
        assert!(!path.exists());

        fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn test_guard_save_writes_config() {
        let dir = scratch_dir("write-ok");
        let path = dir.join("config.toml");
        let mut config = config_with_path(Some(path.clone()));

        let mut config_m = config.as_mut();
        add_server(&mut config_m);
        config_m.save().unwrap();

        let saved = ConfigData::load_from_path(Some(&path)).unwrap();
        assert_eq!(saved.servers.len(), 1);
        assert_eq!(
            fs::metadata(&path).unwrap().permissions().mode() & 0o777,
            FILE_MODE
        );

        fs::remove_dir_all(dir).unwrap();
    }
}
