use lazy_static::lazy_static;
use std::env;
use std::fs::create_dir_all;
use std::path::{Path, PathBuf};

pub struct SiliconProjectDirs {
    cache_dir: PathBuf,
    config_dir: PathBuf,
}

impl SiliconProjectDirs {
    fn new() -> Option<Self> {
        let cache_dir = Self::get_cache_dir()?;

        #[cfg(target_os = "macos")]
        let config_dir_op = env::var_os("XDG_CONFIG_HOME")
            .map(PathBuf::from)
            .filter(|p| p.is_absolute())
            .or_else(|| dirs::home_dir().map(|d| d.join(".config")));

        #[cfg(not(target_os = "macos"))]
        let config_dir_op = dirs::config_dir();

        let config_dir = config_dir_op.map(|d| d.join("silicon"))?;

        create_dir_all(&config_dir).expect("cannot create config dir");
        create_dir_all(&cache_dir).expect("cannot create cache dir");

        Some(Self {
            cache_dir,
            config_dir,
        })
    }

    fn get_cache_dir() -> Option<PathBuf> {
        // on all OS prefer SILICON_CACHE_PATH if set
        let cache_dir_op = env::var_os("SILICON_CACHE_PATH").map(PathBuf::from);
        if cache_dir_op.is_some() {
            return cache_dir_op;
        }

        #[cfg(target_os = "macos")]
        let cache_dir_op = env::var_os("XDG_CACHE_HOME")
            .map(PathBuf::from)
            .filter(|p| p.is_absolute())
            .or_else(|| dirs::home_dir().map(|d| d.join(".cache")));

        #[cfg(not(target_os = "macos"))]
        let cache_dir_op = dirs::cache_dir();

        cache_dir_op.map(|d| d.join("silicon"))
    }

    pub fn cache_dir(&self) -> &Path {
        &self.cache_dir
    }

    pub fn config_dir(&self) -> &Path {
        &self.config_dir
    }
}

lazy_static! {
    pub static ref PROJECT_DIRS: SiliconProjectDirs =
        SiliconProjectDirs::new().expect("Could not get home directory");
}
