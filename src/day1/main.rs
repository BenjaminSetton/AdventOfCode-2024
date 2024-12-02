
use std::fs;
use std::collections::HashMap;

fn Part1() -> i32
{
    let file_path = "../src/day1/input1_1.txt";
    let contents = fs::read_to_string(file_path).expect("Failed to open input file!");

    //println!("Found input: \n\n{contents}");

    let mut list1 : Vec<i32> = Vec::with_capacity(100);
    let mut list2 : Vec<i32> = Vec::with_capacity(100);

    // Split input into separate lists
    for (i, str) in contents.split_whitespace().enumerate()
    {
        let num : i32 = str.parse::<i32>().unwrap();
        if i % 2 == 0
        {
            list1.push(num);
        }
        else 
        {
            list2.push(num); 
        }
    }

    // Sort the lists
    list1.sort();
    list2.sort();

    // Find distance between all entries
    let mut total_diff : i32 = 0;
    for i in 0..list1.len()
    {
        let value1 = list1.get(i);
        let value2 = list2.get(i);
        if !value1.is_none() && !value2.is_none()
        {
            let diff = (value1.unwrap() - value2.unwrap()).abs();
            total_diff += diff;
        }
    }

    return total_diff;
}

fn Part2() -> i32
{
    let file_path = "../src/day1/input1_2.txt";
    let contents = fs::read_to_string(file_path).expect("Failed to open input file!");

    // Create lists from input
    let mut list1 : Vec<i32> = Vec::with_capacity(100);
    let mut list2 : Vec<i32> = Vec::with_capacity(100);

    // Split input into separate lists
    for (i, str) in contents.split_whitespace().enumerate()
    {
        let num : i32 = str.parse::<i32>().unwrap();
        if i % 2 == 0
        {
            list1.push(num);
        }
        else 
        {
            list2.push(num); 
        }
    }

    // Create hash map of right light
    let mut right_list_hash = HashMap::new();
    for i in 0..list2.len()
    {
        if let Some(val) = list2.get(i)
        {
            if right_list_hash.contains_key(val)
            {
                let old_value : i32 = *right_list_hash.get(val).unwrap();
                right_list_hash.insert(val, old_value + 1);
            }
            else 
            {
                right_list_hash.insert(val, 1);
            }
        }
    }

    // Iterate over left list to find similarity score
    let mut total_similarity = 0;
    for i in 0..list1.len()
    {
        let mut similarity = 0;
        let curr_num : i32 = *list1.get(i).unwrap();
        if right_list_hash.contains_key(&curr_num)
        {
            similarity = curr_num * right_list_hash.get(&curr_num).unwrap();
        }

        total_similarity += similarity;
    }

    return total_similarity;
}

fn main()
{
    println!("Hello rust!");

    // let total_diff = Part1();
    // println!("Total diff: {total_diff}");

    // PART 2
    let total_sim = Part2();
    println!("Total sim: {total_sim}");

    // // DEBUG - Print the lists
    // for i in list1
    // {
    //     println!("{i}");
    // }
    // println!("---------------------------");
    // for i in list2
    // {
    //     println!("{i}");
    // }
    // // DEBUG - Print the lists
}