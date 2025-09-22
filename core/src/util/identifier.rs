use bitfield_struct::bitfield;

#[bitfield(u64)]
pub struct Snowflake {
    field: u64,
}
