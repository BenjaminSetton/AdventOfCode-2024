///////////////////////////////////////////////////
/// 
/// I couldn't think of a solution for day 5 myself, 
/// so I read through the solution from this Reddit post.
/// Learned a lot of new Rust tricks along the way too!
/// 
/// https://www.reddit.com/r/adventofcode/comments/1h7ftiv/2024_day_05_rust_any_tips_for_how_i_could_have/
/// 
///////////////////////////////////////////////////

use std::fs;
use std::collections::HashMap;
use std::collections::HashSet;

fn sort_in_valid_order(pages : &Vec<i32>, rules : &HashMap<i32, HashSet<i32>>) -> Vec<i32>
{
    let mut sorted_pages: Vec<i32> = Vec::new();
    let mut mut_pages = pages.clone();

    while !mut_pages.is_empty() {

        // Copy the rules, and modify them so we only retain the rules that
        // are relevant to the page numbers remaining in the list
        let mut local_rules = rules.clone();
        local_rules.retain(|k, _| mut_pages.contains(k));
        for (_, v) in local_rules.iter_mut() {
            v.retain(|page| mut_pages.contains(page));
        }

        // Discard any empty rule sets
        local_rules.retain(|_, v| !v.is_empty());

        // Build new list backwards. If a page does not have an associated rule in the
        // local rules, then it means it's the largest page (since nothing should come
        // after it).
        for page in &mut_pages.clone() {
            if !local_rules.contains_key(page) {
                sorted_pages.insert(0, *page);
                mut_pages.retain(|p| p != page);
            }
        }
    }

    return sorted_pages;
}

fn is_valid_order(pages : &Vec<i32>, rules : &HashMap<i32, HashSet<i32>>) -> bool
{
    let mut found_pages = HashSet::new();
    for page in pages
    {
        found_pages.insert(page);

        // Process:
        // 1) Get ruleSet for the current page in the rules
        // 2) Loop over all rules in the ruleSet
        // 3) If the found pages already contains an invalid number from the ruleSet, then the page list is invalid (return false)
        for rule in rules.get(page).unwrap_or(&HashSet::new())
        {
            if found_pages.contains(&rule)
            {
                return false;
            }
        }
    }

    return true;
}

fn Part1() -> i32
{
    let file_path = "../../src/day5/input5_1.txt";
    let mut contents = fs::read_to_string(file_path).expect("Failed to open input file!");

    // Split the ordering rules from the page lists
    let mut ruleList : Vec<&str> = Vec::new();
    let mut pageList : Vec<&str> = Vec::new();

    let mut foundNL : bool = false;
    for line in contents.lines()
    {
        if line == ""
        {
            foundNL = true;
            continue;
        }

        if foundNL
        {
            pageList.push(line);
        }
        else
        {
            ruleList.push(line);
        }
    }

    // Parse rules and build a hash map from them
    let mut rules : HashMap<i32, HashSet<i32>> = HashMap::new();
    for ruleStr in ruleList
    {
        let rule : Vec<i32> = ruleStr.split('|').collect::<Vec<_>>().iter().map(|r| r.parse().unwrap()).collect();
        
        // Conditionally insert if key doesn't exist
        rules.entry(rule[0])
             .or_insert(HashSet::new())
             .insert(rule[1]);
    }

    // Parse pages
    let mut updates : Vec<Vec<i32>> = Vec::new();
    for pageStr in pageList
    {
        let mut tempList : Vec<i32> = Vec::from(pageStr.split(',').collect::<Vec<_>>().iter().map(|p| p.parse().unwrap()).collect::<Vec<_>>());
        updates.push(tempList);
    }

    // Sort list according to rules, find middle number and return sum
    let total_sum: i32 = updates
        .iter()
        .filter(|&pages| is_valid_order(pages, &rules)) // Only use the pages which are in the correct order
        .map(|pages| pages[pages.len() / 2])            // map the pages into a list with only the middle numbers
        .sum();                                         // Sum them up


    return total_sum;
}

fn Part2() -> i32
{
    let file_path = "../../src/day5/input5_2.txt";
    let mut contents = fs::read_to_string(file_path).expect("Failed to open input file!");

        // Split the ordering rules from the page lists
    let mut ruleList : Vec<&str> = Vec::new();
    let mut pageList : Vec<&str> = Vec::new();

    let mut foundNL : bool = false;
    for line in contents.lines()
    {
        if line == ""
        {
            foundNL = true;
            continue;
        }

        if foundNL
        {
            pageList.push(line);
        }
        else
        {
            ruleList.push(line);
        }
    }

    // Parse rules and build a hash map from them
    let mut rules : HashMap<i32, HashSet<i32>> = HashMap::new();
    for ruleStr in ruleList
    {
        let rule : Vec<i32> = ruleStr.split('|').collect::<Vec<_>>().iter().map(|r| r.parse().unwrap()).collect();
        
        // Conditionally insert if key doesn't exist
        rules.entry(rule[0])
             .or_insert(HashSet::new())
             .insert(rule[1]);
    }

    // Parse pages
    let mut updates : Vec<Vec<i32>> = Vec::new();
    for pageStr in pageList
    {
        let mut tempList : Vec<i32> = Vec::from(pageStr.split(',').collect::<Vec<_>>().iter().map(|p| p.parse().unwrap()).collect::<Vec<_>>());
        updates.push(tempList);
    }

    let total_sum: i32 = updates
        .iter()
        .filter(|&pages| !is_valid_order(pages, &rules)) // Only use the pages which are NOT in the correct order
        .map(|pages| sort_in_valid_order(pages, &rules)) // Sort the lists in invalid order to a valid order
        .map(|pages| pages[pages.len() / 2])             // Map the pages into a list with only the middle numbers
        .sum();                                          // Sum them up

    return total_sum;
}

fn main() {
    // let res1 : i32 = Part1();
    // println!("Part 1 answer: {res1}");

    let res2 : i32 = Part2();
    println!("Part 2 answer: {res2}");
}
