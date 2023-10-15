use core::fmt;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum Tag {
    DeepStrike,
    Leader,
    Teleport,
    WoundTarget,
    PerBattle,
    PerTurn,
    Charge,
    Fight,
    Shoot,
    Daemon,
    DeadlyDemise,
    Strategem,
    OnDeath,
    Aura,
    BattleShock,
    LeaderShip,
    ObjectiveControl,
    Pregame
}

impl fmt::Display for Tag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Tag::DeepStrike => write!(f, "deep_strike"),
            Tag::Leader => write!(f, "leader"),
            Tag::Teleport => write!(f, "teleport"),
            Tag::WoundTarget => write!(f, "wound_target"),
            Tag::PerTurn => write!(f, "per_turn"),
            Tag::PerBattle => write!(f, "per_battle"),
            Tag::Charge => write!(f, "charge"),
            Tag::Fight => write!(f, "fight"),
            Tag::Shoot => write!(f, "shoot"),
            Tag::Daemon => write!(f, "daemon"),
            Tag::DeadlyDemise => write!(f, "deadly_demise"),
            Tag::Strategem => write!(f, "strategem"),
            Tag::OnDeath => write!(f, "on_death"),
            Tag::Aura => write!(f, "aura"),
            Tag::BattleShock => write!(f, "battle_shock"),
            Tag::LeaderShip => write!(f, "leader_ship"),
            Tag::ObjectiveControl => write!(f, "objective_control"),
            Tag::Pregame => write!(f, "pregame"),
        }
    }
}