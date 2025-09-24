use std::{path::Path, rc::Rc};

pub struct Table {}

#[derive(Error, Debug)]
pub enum OpenError {
    #[error("OS Io error: {0:?}")]
    Io(#[from] std::io::Error),

    #[error("Path ({0:?}) does not exist")]
    NonExistent(Rc<Path>),
}

type OpenResult<T> = Result<T, (Rc<str>, OpenError)>;

impl Table {
    pub fn try_open(path: Rc<Path>) -> Option<OpenResult<Table>> {}
}

impl perspectivedb_core::persistence::Table for Table {}
