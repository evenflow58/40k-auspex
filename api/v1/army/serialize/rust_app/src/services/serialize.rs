use regex::Regex;
// use regex_split::RegexSplit;
use std::error::Error;
use tracing::info;
use serde::{Deserialize, Serialize};

use crate::models::{display_unit::DisplayUnit};

// const GREY_KNIGHTS: &str = "Grey Knights";
const ARMIES: [&str; 2] = ["Grey Knights", "Orks"];
const FACTIONS: [&str; 1] = ["Teleport Strike Force"];
const ARMY_TYPE: [&str; 3] = ["Onslaught", "Incursion", "Strike Force"];
const GREY_KNIGHTS_UNITS: [&str;8] = [
    "Brotherhood Librarian",
    "Grand Master in Nemesis Dreadknight",
    "Kaldor Draigo",
    "Brotherhood Terminator Squad",
    "Strike Squad",
    "Paladin Squad",
    "Purifier Squad",
    "Nemesis Dreadknight",
    ];
const GREY_KNIGHTS_ENHANCEMENTS: [&str; 2] = [
    "Domina Liber Daemonica",
    "Sigil of Exigence"
];

// fn get_units(units_string: &str) {
//     const GREY_KNIGHTS_UNITS: &str = "Brotherhood Librarian|Grand Master in Nemesis Dreadknight|Kaldor Draigo|Brotherhood Terminator Squad|Strike Squad|Paladin Squad|Purifier Squad|Nemesis Dreadknight";
//     let re = Regex::new(r"(?<unit>) \(\d+ Points\)")
// }

fn find_locations<'a>(search_string: &'a str, search_terms: &'a [&'a str]) -> Vec<(usize, &'a str)> {
    let mut locations = Vec::new();
    for term in search_terms {
        let term_locations: Vec<_> = search_string.match_indices(term).collect();
        if term_locations.len() > 0 {
            locations.push(term_locations[0]);
        }
    }
    return locations;
}

pub fn serialize_army(army_string: &str) -> Result<(), Box<dyn Error>> {
    let re = Regex::new(
        &format!(
            r#".*\(\d+ Points\) (?<army>{armies}) (?<faction>.*) (?<army_type>{army_types}) \(\d+ Points\) CHARACTERS (?<units>.*)"#,
            armies = ARMIES.join("|"),
            army_types = ARMY_TYPE.join("|"),
        )
    ).unwrap();
    let Some(caps) = re.captures(army_string) else {
        panic!("no match");
    };

    let units = find_locations(&caps["units"], &GREY_KNIGHTS_UNITS)
        .iter()
        .map(|x| (x.0, x.1, "Unit"))
        .collect::<Vec<_>>();
    let enhancements = find_locations(&caps["units"], &GREY_KNIGHTS_ENHANCEMENTS)
        .iter()
        .map(|x| (x.0, x.1, "Enhancement"))
        .collect::<Vec<_>>();

    let mut all_locations = [&units[..], &enhancements[..]].concat();
    all_locations.sort_by(|a, b| a.cmp(&b));
    
    // let mapped_locations: Vec<_> = all_locations.iter().map(|x| x.1).collect();
    let mapped_locations: Vec<Unit> = all_locations
        .into_iter()
        .fold(Vec::new(), |mut acc, x| {
            if x.2 == "Unit" {
                acc.push(DisplayUnit::new(x.1.to_string()))
            } else if x.2 == "Enhancement" {
                acc
                    .last_mut()
                    .unwrap()
                    .enhancement = Some(x.1.to_string());
            }

            return acc;
        });
    info!("Mapped locations {:?}", &mapped_locations);
    
    return Ok(());
}
