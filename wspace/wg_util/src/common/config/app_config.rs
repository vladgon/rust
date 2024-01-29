use std::env;
use std::ffi::OsStr;
use std::path::Path;
use std::sync::OnceLock;

use anyhow::{bail, Context};
use config::{Config, Environment, File, FileFormat, Map, Source};
use config::FileFormat::Yaml;
use log::debug;

use crate::{Result, ResultExt};
use crate::common::config::model::Model;

#[derive(Default)]
pub struct AppConfig {}

impl Init for AppConfig {}

pub fn settings<'a>() -> Result<&'a Model> {
    CONFIG.get()
        .with_context(|| "Settings are not initialized, call AppConfig::default().init variant ")
        .into_std_error()
}

static CONFIG: OnceLock<Model> = OnceLock::new();

fn get_format<T: AsRef<Path>>(path: T) -> Result<FileFormat> {
    let res = path.as_ref().extension()
        .and_then(OsStr::to_str)
        .map(str::to_lowercase)
        .map(|ext| match ext.as_str() {
            "yaml" => Ok(Yaml),
            _ => bail!("Extension {ext} is not supported, path {}", path.as_ref().to_str().unwrap_or("no extension"))
        })
        .with_context(|| format!(" path {:?}: {}",
                                 path.as_ref(),
                                 "No extension, cannot derive the wg_sample_app format"))?;
    res.into_std_error()
}

pub trait Init {
    fn init_with_files<T: AsRef<Path>>(&self, sources: &[T], env_override: bool) -> Result<&Model> {
        let t2: Vec<(&T, bool)> = sources.iter()
            .map(|t| (t, true))
            .collect();
        self.init_with_files_and_required(&t2, env_override)
    }

    fn init_with_files_and_required<T: AsRef<Path>>(&self, sources: &[(T, bool)], env_override: bool) -> Result<&Model> {
        let sources = sources.iter()
            .try_fold(&mut Vec::new(),
                      |res, (path, required)| {
                          let path_as_str = path.as_ref().to_str().with_context(|| "Cannot convert path to String")?;
                          get_format(path)
                              .map(|format| res.push(File::new(path_as_str, format).required(*required)))
                              .and(Ok(res))
                      },
            )?
            .to_owned();
        self.init_with_sources(sources, env_override)
    }

    fn init_with_sources<'a, T: Source + Send + Sync + 'static>(&self, sources: Vec<T>, env_override: bool) -> Result<&'a Model> {
        assert!(CONFIG.get().is_none(), "CONFIG is already initialized");

        let mut builder = sources.into_iter()
            .fold(Config::builder(), |b, source| b.add_source(source));

        if env_override {
            let env_map = Some(env::vars().collect::<Map<String, String>>());
            builder = builder.add_source(Environment::default().source(env_map));
        }
        builder
            .build()
            .map(|setting| setting.try_deserialize::<Model>().unwrap_or_else(|e| panic!("{}", e)))
            .map(|app_config| CONFIG.get_or_init(|| app_config))
            .map(|app_config| {
                debug!("Processed config {:?}", app_config);
                app_config
            })
            .into_std_error()
    }
}

