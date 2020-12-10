use std::str::FromStr;
use std::collections::{HashMap, HashSet};

static BAG_RULES: &'static str = include_str!("../data/data_07.txt");

pub fn part_1() -> usize {
    let (container_map, _) = get_maps();

    // Query the container map for all bags able to directly or indirectly hold a "shiny gold" bag.
    containers("shiny gold", &container_map).len()
}

pub fn part_2() -> u32 {
    let (_, contents_map) = get_maps();

    // Recursively query what bags are required inside a "shiny gold" bag.
    contents("shiny gold", &contents_map)
        .into_iter()
        .map(|(_bag, count)| count)
        .sum()
}

fn get_maps() -> (HashMap<String, Vec<String>>, HashMap<String, Vec<(u32, String)>>) {
    // When given a bag, returns all bags that can directly contain it.
    let mut container_map = HashMap::new();

    // When given a bag, returns all its required inner bags and their quantities.
    let mut contents_map = HashMap::new();


    // Populate the maps from the rules data.
    BAG_RULES.lines()
        .for_each(|rule| {
            let (container, contents) = parse_rule(rule);

            // Populate container_map for part 1.
            contents.iter()
                .for_each(|(_qty, bag)| {
                    // println!("Storing relationship: A {:?} bag can be held by a {:?} bag.", bag, container);
                    let containers = container_map.entry(bag.clone()).or_insert(vec![]);
                    containers.push(container.clone())
                });

            // Populate contents_map for part 2.
            let prev = contents_map.insert(container, contents);
            assert!(prev.is_none());  // Something went wrong if we already had an entry,
        });

    (container_map, contents_map)
}

/// When given a bag colour, give back all bags that can directly or indirectly hold it.
fn containers(bag: &str, container_map: &HashMap<String, Vec<String>>)
    -> HashSet<String>
{
    containers_(bag, container_map, HashSet::new())
}

fn containers_(bag: &str, container_map: &HashMap<String, Vec<String>>, mut acc: HashSet<String>)
    -> HashSet<String>
{
    if let Some(containers) = container_map.get(bag) {
        // println!("Found that a {:?} can be directly held by {} different bags", bag, containers.len());
        for bag in containers {
            acc.insert(String::from(bag));
            acc = containers_(bag, container_map, acc);
        }
    }

    acc
}

fn contents(bag: &str, contents_map: &HashMap<String, Vec<(u32, String)>>)
    -> HashMap<String, u32>
{
    contents_(bag, contents_map, 1, HashMap::new())
}

fn contents_(bag: &str, contents_map: &HashMap<String, Vec<(u32, String)>>, factor: u32, mut acc: HashMap<String, u32>)
    -> HashMap<String, u32>
{
    if let Some(contents) = contents_map.get(bag) {
        // println!("Found that {} needs to contain {:?}", bag, contents);

        // Recursively add child bag contents.
        for (qty, bag) in contents {
            let child_count = *qty * factor;
            let count = acc.entry(String::from(bag)).or_insert(0);
            *count = *count + child_count;
            acc = contents_(bag, contents_map, child_count, acc);
        }
    }

    acc
}

fn parse_rule(rule: &str) -> (String, Vec<(u32, String)>) {
    let mut words = rule.split_whitespace();
    let container = String::from(words.next().unwrap()) + " " + words.next().unwrap();

    let mut contents = vec![];

    loop {
        if let Some(s) = words.next() {
            if let Ok(qty) = u32::from_str(s) {
                let bag = String::from(words.next().unwrap()) + " " + words.next().unwrap();
                contents.push((qty, bag));
            }
        } else {
            break;
        }
    }

    (container, contents)
}