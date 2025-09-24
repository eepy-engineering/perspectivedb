pub mod table;
pub mod util;

use std::{fs::{create_dir_all, FileType}, io, path::Path, rc::Rc};

use perspectivedb_core::{config::DatabaseConfig, persistence::Persistence};
use thiserror::Error;

use crate::persistence::fs::{table::Table, util::path_safe_slug};

pub struct FsPersistence {
    directory: Rc<Path>,
    config: DatabaseConfig,
}

#[derive(Error, Debug)]
pub enum OpenError {
    #[error("OS Io error: {0:?}")]
    Io(#[from] std::io::Error),

    #[error("Path ({0:?}) does not exist")]
    NonExistent(Rc<Path>),

    #[error("Table {0:?} failed to open: {}")]
    TableFailed(Rc<str>, table::OpenError),

    #[error("Path ({0:?}) does not contain a `config.json` file")]
    NoConfig(Rc<Path>),
}

type OpenResult<T> = Result<T, OpenError>;

impl FsPersistence {
    fn read_config(path: &Path) -> io::Result<DatabaseConfig> {
        Ok(DatabaseConfig::default())
    }

    pub fn create(path: impl Into<Path>) -> OpenResult<Self> {
        let path = Rc::new(path.into());

        if !path.exists() {
            create_dir_all(&path)?
        }

        Self::open(path)
    }

    pub fn open(path: impl Into<Path>) -> OpenResult<Self> {
        let path = Rc::new(path.into());

        if !path.exists() {
            return Err(OpenError::NonExistent(path.clone()));
        }

        let mut config: Option<DatabaseConfig> = None;
        let mut tables: Vec<Table> = vec![];

        for ancestor in path.read_dir()? {
            match (ancestor?.file_name(), ancestor?.file_type()?.is_file()) {
                ("config.json", true) => config = Some(Self::read_config(&ancestor?.path())?),
                (_, false) if let Some(table_result) = Table::try_open(Rc::new(ancestor?.path())) => {
                    tables.push(table_result
                        .map_err(|(n, e)| OpenError::TableFailed(n, e))?)
                }
            }
        }

        Ok(Self {
            config: config.ok_or_else(|| OpenError::NoConfig(path.clone()))?,
            directory: path,
        })
    }
}

impl Persistence for FsPersistence {
    type Table = Table;

    fn config(&self) -> &perspectivedb_core::config::DatabaseConfig {
        &self.config
    }

    fn table(&self, table: &str) -> Option<&Self::Table> {
        Some(self.directory.join(path_safe_slug(table, 64))).filter(|v| v.exists())
        .
    }
}
