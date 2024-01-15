use std::env;
use std::ops::ControlFlow;
use std::path::Path;
use std::sync::OnceLock;

use anyhow::{Context, Error, Result};
use config::{Config, Environment, File, Map, Source};
use log::debug;

use crate::common::config::model::Model;
use crate::common::error::Errors;

#[derive(Default)]
pub struct AppConfig {}

impl ConfigInit for AppConfig {}

pub fn settings<'a>() -> std::result::Result<&'a Model, Error> { CONFIG.get().with_context(|| "Settings are not set".to_string()) }

static CONFIG: OnceLock<Model> = OnceLock::new();
static CONFIG1: OnceLock<Model> = OnceLock::new();

pub trait ConfigInit {
    fn init_with_files<'a, T: AsRef<Path>>(&self, sources: &[T], env_override: bool) -> Result<&'a Model, Errors> {
        let t2: Vec<(&T, bool)> = sources.iter()
            .map(|t| (t, true))
            .collect();
        self.init_with_files_and_required(&t2, env_override)
    }
    fn init_with_files_and_required<'a, T: AsRef<Path>>(&self, sources: &[(T, bool)], env_override: bool) -> Result<&'a Model, Errors> {
        let sources = sources.iter()
            .try_fold(Ok(Vec::new()),
                      |res, t2| {
                          let (path, required) = t2;
                          match private::get_type(path) {
                              Ok(format) => {
                                  let res = res.map(|mut v| {
                                      let file = File::new(path.as_ref().to_str().unwrap(), format).required(*required);
                                      v.push(file);
                                      v
                                  });
                                  ControlFlow::Continue(res)
                              }
                              Err(e) => ControlFlow::Break(Err(e)),
                          }
                      });

        let sources = match sources {
            ControlFlow::Continue(res) => res,
            ControlFlow::Break(res) => res
        }?;
        self.init_with_sources(sources, env_override)
    }
    fn init_with_sources<'a, T: Source + Send + Sync + 'static>(&self, sources: Vec<T>, env_override: bool) -> Result<&'a Model, Errors> {
        assert!(CONFIG.get().is_none(), "CONFIG is already initialized");

        let mut builder = sources.into_iter()
            .fold(Config::builder(), |b,
                                      source| b.add_source(source));

        if env_override {
            let env_map = Some(env::vars().collect::<Map<String, String>>());
            builder = builder.add_source(Environment::default().source(env_map));
        }
        builder
            .build()
            .map(|setting| setting.try_deserialize::<Model>().map_err(Errors::from))?
            .map(|app_config| CONFIG1.get_or_init(|| app_config))
            .map(|app_config| {
                debug!("Processed config {:?}", app_config);
                app_config
            })
    }
}

mod private {
    use std::ffi::OsStr;
    use std::path::Path;

    use config::FileFormat;
    use config::FileFormat::{Ini, Json, Toml, Yaml};

    use crate::common::error::Errors;

    pub fn get_type<T: AsRef<Path>>(path: T) -> Result<FileFormat, Errors> {
        path.as_ref().extension()
            .and_then(OsStr::to_str)
            .map(str::to_lowercase)
            .map(|ext| match ext.as_str() {
                "yaml" => Ok(Yaml),
                "ini" => Ok(Ini),
                "toml" => Ok(Toml),
                "json" => Ok(Json),
                _ => Err(Errors::Parse(format!("Extension {ext} is not supported, path {}", path.as_ref().to_str().unwrap_or("no extension"))))
            })
            .unwrap_or_else(|| Err(Errors::Parse(format!("error for path {}", path.as_ref().to_str().unwrap_or("no extension")))))
    }
}
