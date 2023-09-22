use core::fmt;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum EffectType {
    Anti,
    IgnoresCover,
    Psychic,
    RapidFire,
    Torrent,
    DevastatingWounds,
    Precision,
    Hazardous,
    Pistol,
    ExtraAttacks,
    Command,
}

impl fmt::Display for EffectType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            EffectType::Anti => write!(f, "anti"),
            EffectType::IgnoresCover => write!(f, "ignores_cover"),
            EffectType::Psychic => write!(f, "psychic"),
            EffectType::RapidFire => write!(f, "rapid_fire"),
            EffectType::Torrent => write!(f, "torrent"),
            EffectType::DevastatingWounds => write!(f, "devastating_wounds"),
            EffectType::Precision => write!(f, "precision"),
            EffectType::Hazardous => write!(f, "hazardous"),
            EffectType::Pistol => write!(f, "pistol"),
            EffectType::ExtraAttacks => write!(f, "extra_attacks"),
            EffectType::Command => write!(f, "command"),
        }
    }
}