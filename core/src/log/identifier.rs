use crate::util::identifier::Snowflake;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct EntryId(Snowflake);
