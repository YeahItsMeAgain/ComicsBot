use strum::{Display, EnumIter, EnumString};

pub mod cyanide_and_happiness;

#[derive(Debug, Clone, Copy, EnumString, Display, EnumIter)]
#[strum(serialize_all = "snake_case")]
pub enum ComicsProviders {
    CyanideAndHappiness,
}
