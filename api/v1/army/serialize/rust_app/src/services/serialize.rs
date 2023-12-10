use regex::Regex;
// use regex_split::RegexSplit;
use std::error::Error;
use tracing::info;

use super::data::get_armies;
use utils::models::display_unit::DisplayUnit;

const BATTLE_SIZE: [&str; 3] = ["Onslaught", "Incursion", "Strike Force"];
const GREY_KNIGHTS_UNITS: [&str; 8] = [
    "Brotherhood Librarian",
    "Grand Master in Nemesis Dreadknight",
    "Kaldor Draigo",
    "Brotherhood Terminator Squad",
    "Strike Squad",
    "Paladin Squad",
    "Purifier Squad",
    "Nemesis Dreadknight",
];
const GREY_KNIGHTS_ENHANCEMENTS: [&str; 2] = ["Domina Liber Daemonica", "Sigil of Exigence"];

// fn get_units(units_string: &str) {
//     const GREY_KNIGHTS_UNITS: &str = "Brotherhood Librarian|Grand Master in Nemesis Dreadknight|Kaldor Draigo|Brotherhood Terminator Squad|Strike Squad|Paladin Squad|Purifier Squad|Nemesis Dreadknight";
//     let re = Regex::new(r"(?<unit>) \(\d+ Points\)")
// }

fn find_locations<'a>(
    search_string: &'a str,
    search_terms: &'a [&'a str],
) -> Vec<(usize, &'a str)> {
    let mut locations = Vec::new();
    for term in search_terms {
        let term_locations: Vec<_> = search_string.match_indices(term).collect();
        if term_locations.len() > 0 {
            locations.push(term_locations[0]);
        }
    }
    return locations;
}

pub async fn serialize_army(army_string: &str) -> Result<(), Box<dyn Error>> {
    let armies = get_armies().await?;

    let army_names: Vec<String> = armies.clone().into_iter().map(|x| x.name).collect();
    info!("Army names {:?}", army_names);

    let army_re = Regex::new(&format!(
        r#".*\(\d+ Points\) (?<army>{armies})(?<all>.*)"#,
        armies = army_names.join("|"),
    ))
    .unwrap();

    // let re = Regex::new(
    //     &format!(
    //         r#".*\(\d+ Points\) (?<army>{armies}) (?<faction>.*) (?<army_type>{army_types}) \(\d+ Points\) CHARACTERS (?<units>.*)"#,
    //         armies = army_names.join("|"),
    //         army_types = ARMY_TYPE.join("|"),
    //     )
    // ).unwrap();
    let Some(army_caps) = army_re.captures(army_string) else {
        panic!("no match");
    };

    let factions: Vec<_> = armies
        .clone()
        .into_iter()
        .filter(|x| x.name == army_caps["army"])
        .flat_map(|x| x.factions)
        .collect();
    info!("Factions {:?}", factions);

    let re = Regex::new(
        &format!(
            r#"(?<faction>{factions}) (?<battle_size>{battle_size}) \(\d+ Points\) CHARACTERS (?<units>.*)"#,
            factions = factions.join("|"),
            battle_size = BATTLE_SIZE.join("|"),
        )
    ).unwrap();
    let Some(caps) = re.captures(&army_caps["all"]) else {
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
    let mapped_locations: Vec<DisplayUnit> =
        all_locations.into_iter().fold(Vec::new(), |mut acc, x| {
            if x.2 == "Unit" {
                acc.push(DisplayUnit::new(x.1.to_string()))
            } else if x.2 == "Enhancement" {
                acc.last_mut().unwrap().enhancement = Some(x.1.to_string());
            }

            return acc;
        });
    info!("Mapped locations {:?}", &mapped_locations);

    return Ok(());
}
