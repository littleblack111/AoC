use std::ops::Deref;

#[derive(Ord, PartialOrd, PartialEq, Eq, Debug)]
pub struct Joltage {
    inner: u8,
}

impl Joltage {
    pub fn parse(value: u8) -> Result<Self, String> {
        if value > 10 {
            return Err(
                format!(
                    "Joltage value {} is out of range (0-10)",
                    value
                ),
            );
        } else if value == 0 {
            return Err("Joltage value cannot be zero".to_string());
        }
        Ok(
            Joltage {
                inner: value,
            },
        )
    }

    fn value(&self) -> u8 {
        self.inner
    }
}

impl Deref for Joltage {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl From<Joltage> for u8 {
    fn from(value: Joltage) -> Self {
        value.inner
    }
}
