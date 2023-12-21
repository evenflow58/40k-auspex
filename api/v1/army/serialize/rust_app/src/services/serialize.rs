use super::data::{get_armies, save_army_list};
use regex::Regex;
use std::error::Error;
use tracing::info;
use utils::models::{army_list::ArmyList, enhancment::Enhancement, unit::Unit};

const BATTLE_SIZE: [&str; 3] = ["Onslaught", "Incursion", "Strike Force"];

fn find_locations<'a>(search_string: &'a str, search_terms: &Vec<String>) -> Vec<(usize, &'a str)> {
    let mut locations = Vec::new();
    for term in search_terms {
        let term_locations: Vec<_> = search_string.match_indices(term).collect();
        if term_locations.len() > 0 {
            locations.push(term_locations[0]);
        }
    }
    return locations;
}

pub async fn serialize_army(
    user_id: &str,
    name: &str,
    army_string: &str,
) -> Result<(), Box<dyn Error>> {
    let armies = get_armies().await?;
    info!("Army {:?}", armies);

    let army_names: Vec<String> = armies.clone().into_iter().map(|a| a.name).collect();
    info!("Army names {:?}", army_names);
    let detachments: Vec<String> = armies
        .clone()
        .into_iter()
        .flat_map(|a| {
            a.detachments
                .clone()
                .into_iter()
                .map(|d| d.name)
                .collect::<Vec<String>>()
        })
        .collect();
    info!("Detachment names {:?}", detachments);

    let army_re = Regex::new(&format!(
        r#".*\(\d+ Points\) (?<army>{armies})(?<all>.*)"#,
        armies = army_names.join("|"),
    ))
    .unwrap();

    let Some(army_caps) = army_re.captures(army_string) else {
        panic!("no match");
    };

    let army = armies
        .clone()
        .into_iter()
        .find(|x| x.name == army_caps["army"])
        .unwrap();

    let re = Regex::new(
        &format!(
            r#"(?<detachment>{detachments}) (?<battle_size>{battle_size}) \(\d+ Points\) CHARACTERS (?<units>.*)"#,
            detachments = detachments.join("|"),
            battle_size = BATTLE_SIZE.join("|"),
        )
    ).unwrap();
    let Some(caps) = re.captures(&army_caps["all"]) else {
        panic!("no match");
    };

    let army_units = army
        .units
        .clone()
        .into_iter()
        .map(|x| Unit::new(x.name, &x.abilities, None))
        .collect::<Vec<Unit>>();
    let army_enhancements = army
        .enhancements
        .clone()
        .into_iter()
        .map(|x| Enhancement::new(x.name, &x.tags))
        .collect::<Vec<Enhancement>>();

    let units = find_locations(
        &caps["units"],
        &army_units
            .clone()
            .into_iter()
            .map(|x| x.name)
            .collect::<Vec<String>>(),
    )
    .iter()
    .map(|x| (x.0, x.1, "Unit"))
    .collect::<Vec<_>>();
    let enhancements = find_locations(
        &caps["units"],
        &army_enhancements
            .clone()
            .into_iter()
            .map(|x| x.name)
            .collect::<Vec<String>>(),
    )
    .iter()
    .map(|x| (x.0, x.1, "Enhancement"))
    .collect::<Vec<_>>();

    let mut all_locations = [&units[..], &enhancements[..]].concat();
    all_locations.sort_by(|a, b| a.cmp(&b));

    let units: Vec<Unit> = all_locations.into_iter().fold(Vec::new(), |mut acc, x| {
        if x.2 == "Unit" {
            let abilities = &army_units
                .iter()
                .find(|unit| unit.name == x.1.to_string())
                .unwrap()
                .abilities;
            acc.push(Unit::new(x.1.to_string(), &abilities, None))
        } else if x.2 == "Enhancement" {
            acc.last_mut().unwrap().enhancement = army_enhancements
                .iter()
                .find(|enhancement| enhancement.name == x.1.to_string())
                .cloned();
        }

        return acc;
    });

    let army_list: ArmyList = ArmyList::new(
        army_caps["army"].to_string(),
        caps["detachment"].to_string(),
        units,
    );

    match save_army_list(user_id.to_string(), name.to_string(), army_list).await {
        Ok(()) => Ok(()),
        Err(err) => panic!("{}", err),
    }
}
