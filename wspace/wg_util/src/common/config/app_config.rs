use std::env;
use std::path::Path;
use std::sync::OnceLock;

use config::{Config, Environment, File, Source};
use log::debug;

use crate::{Result, ResultExt};
use crate::common::config::model::Model;
use crate::common::result_ext::ResultTap;

#[derive(Default)]
pub struct AppConfig {}

impl Init for AppConfig {}

static CONFIG: OnceLock<Model> = OnceLock::new();

pub fn settings<'a>() -> Result<&'a Model> {
    CONFIG.get()
          .ok_or("Settings are not initialized, call AppConfig::default().init variant ")
          .into_std_error()
}

pub trait Init {
    fn init_with_files<T: AsRef<Path>>(&self, sources: &[T], env_override: bool) -> Result<&Model> {
        self.init_with_files_and_required(sources.iter()
                                                 .map(|t| (t, true))
                                                 .collect::<Vec<_>>()
                                                 .as_slice(),
                                          env_override)
    }

    fn init_with_files_and_required<T: AsRef<Path>>(&self, sources: &[(T, bool)], env_override: bool) -> Result<&Model> {
        self.init_with_sources(sources.iter()
                                      .map(|(path, required)| File::from(path.as_ref()).required(*required))
                                      .collect::<Vec<_>>(),
                               env_override)
    }

    fn init_with_sources<'a, T: Source + Send + Sync + 'static>(&self, sources: Vec<T>, env_override: bool) -> Result<&'a Model> {
        assert!(CONFIG.get().is_none(), "CONFIG is already initialized");

        let builder = Config::builder();
        let builder = sources.into_iter()
                             .fold(builder, |b, src| b.add_source(src));

        let builder = if env_override {
            builder.add_source(Environment::default().source(Some(env::vars().collect())))
        } else { builder };
        builder
            .build()
            .map(|setting| setting.try_deserialize::<Model>().unwrap_or_else(|e| panic!("{}", e)))
            .map(|app_config| CONFIG.get_or_init(|| app_config))
            .tap(|app_config| debug!("Processed config\n{}", serde_json::to_string_pretty(&app_config).unwrap()))
            .into_std_error()
    }
}