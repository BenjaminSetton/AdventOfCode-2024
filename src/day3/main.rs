
use std::fs;
use regex::Regex;

fn Part2() -> i32
{
    let file_path = "../../src/day3/input3_2.txt";
    let contents = fs::read_to_string(file_path).expect("Failed to open input file!");

    // Find mul() instructions
    let re_mul = Regex::new(r"mul\((\d{0,3})\s*?,(\d{0,3})\s*?\)").unwrap();
    let mul_matches : Vec<_> = re_mul.find_iter(&contents).collect();

    // Find do() instruction
    let re_do = Regex::new(r"do\(\)").unwrap();
    let do_matches : Vec<_> = re_do.find_iter(&contents).collect();

    // Find don't() instruction
    let re_dont = Regex::new(r"don't\(\)").unwrap();
    let dont_matches : Vec<_> = re_dont.find_iter(&contents).collect();

    // Collect all valid mul() instructions
    let mut valid_instructions : Vec<&str> = vec![];
    for mul in mul_matches
    {
        // We can either be behind a DO, a DONT or neither
        // A mul instruction is valid if we're behind DO or neither,
        // but not a DONT instruction
        let mul_start : i32 = mul.start() as i32;

        // Find the largest DO and DONT start position which is still less than
        // the mul start position
        let mut do_start : i32 = -1;
        for do_match in &do_matches
        {
            let dms : i32 = do_match.start() as i32;
            if dms < mul_start
            {
                do_start = dms;
            }
        }

        let mut dont_start : i32 = -1;
        for dont_match in &dont_matches
        {
            let dtms : i32 = dont_match.start() as i32;
            if dtms < mul_start
            {
                dont_start = dtms;
            }
        }

        if mul_start > do_start && do_start >= dont_start
        {
            valid_instructions.push(mul.as_str());
        }
    }

    // Finally calculate the mul() instructions' summed value
    let mut total_sum : i32 = 0;
    let simpler_re = Regex::new(r"\((\d{0,3}),(\d{0,3})").unwrap();
    for ins in valid_instructions
    {
        if let Some(captures) = simpler_re.captures(ins)
        {
            let group1 = captures.get(1).map_or("", |m| m.as_str());
            let group2 = captures.get(2).map_or("", |m| m.as_str());

            total_sum += group1.parse::<i32>().unwrap() * group2.parse::<i32>().unwrap();
        }
    }

    return total_sum;
}

fn Part1() -> i32
{
    let file_path = "../../src/day3/input3_1.txt";
    let contents = fs::read_to_string(file_path).expect("Failed to open input file!");

    let re = Regex::new(r"mul\((\d{0,3})\s*?,(\d{0,3})\s*?\)").unwrap();
    let matches : Vec<_> = re.find_iter(&contents).map(|m| m.as_str()).collect();

    let mut total_sum : i32 = 0;
    let simpler_re = Regex::new(r"\((\d{0,3}),(\d{0,3})").unwrap();
    for mat in matches
    {
        if let Some(captures) = simpler_re.captures(mat)
        {
            let group1 = captures.get(1).map_or("", |m| m.as_str());
            let group2 = captures.get(2).map_or("", |m| m.as_str());

            total_sum += group1.parse::<i32>().unwrap() * group2.parse::<i32>().unwrap();
        }
    }

    return total_sum;
}

fn main()
{
    let res1 : i32 = Part1();
    println!("Part 1 result: {res1}");

    let res2 : i32 = Part2();
    println!("Part 2 result: {res2}");
}