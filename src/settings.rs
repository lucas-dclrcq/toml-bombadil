use crate::dots::Dot;
use crate::hook::Hook;
use anyhow::Result;
use config::{Config, ConfigError, File};
use std::path::PathBuf;

/// The Global bombadil configuration
#[derive(Debug, Deserialize, Serialize)]
pub struct Settings {
    /// User define dotfiles directory, usually your versioned dotfiles
    pub(crate) dotfiles_dir: PathBuf,
    /// A list of symlink to edit
    pub dot: Vec<Dot>,
    /// Post install hook commands
    pub hook: Option<Vec<Hook>>,
    /// Variables to use in templates
    pub var: Option<Vec<Var>>,
    /// Meta variables to replace variables definitions
    pub meta: Option<Vec<Var>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Var {
    pub path: PathBuf,
}

impl Settings {
    /// Resolve bombadil settings against its standard xdg path :
    /// `$XDG_CONFIG_DIR/bombadil.toml`
    pub fn get() -> Result<Self> {
        match Self::bombadil_config_xdg_path() {
            Ok(path) => {
                if path.exists() {
                    let mut s = Config::new();
                    s.merge(File::from(path))?;
                    s.try_into()
                        .map_err(|err| anyhow!("Config format error : {}", err))
                } else {
                    Err(anyhow!(
                        "Unable to find bombadil config file {}",
                        path.display()
                    ))
                }
            }
            Err(err) => Err(anyhow!("Config error : {}", err)),
        }
    }

    /// Resolve the bombadil XDG settings path : `$XDG_CONFIG_DIR/bombadil.toml
    pub fn bombadil_config_xdg_path() -> Result<PathBuf, ConfigError> {
        dirs::config_dir()
            .ok_or_else(|| {
                ConfigError::NotFound("Unable to find `$XDG_CONFIG/bombadil.toml`".into())
            })
            .map(|path| path.join("bombadil.toml"))
    }
}
