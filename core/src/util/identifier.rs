use rand::{RngCore, TryRngCore};

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Snowflake {
    value: u64,
}

impl Snowflake {
    pub fn new<R: RngCore>(rng: &mut R) -> Snowflake {
        Snowflake {
            value: rng.next_u64(),
        }
    }

    pub fn try_new<R: TryRngCore>(rng: &mut R) -> Result<Snowflake, R::Error> {
        Ok(Snowflake {
            value: rng.try_next_u64()?,
        })
    }
}
