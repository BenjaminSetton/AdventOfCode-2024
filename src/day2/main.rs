
use std::fs;

fn Part1() -> i32
{
    let file_path = "../src/day2/input1_1.txt";
    let contents = fs::read_to_string(file_path).expect("Failed to open input file!");

    let mut reports : Vec<Vec<i32>> = Vec::new();
    for (i, report) in contents.split("\n").enumerate()
    {
        let mut curr_report = Vec::new();
        for level in report.split(" ")
        {
            curr_report.push(level.trim().parse::<i32>().unwrap());
        }

        reports.push(curr_report);
    }

    let mut total_safe : i32 = 0;

    for report in reports
    {
        let mut total_diff : i32 = 0;
        let mut is_safe : bool = true;
        for i in 0..report.len() - 1
        {
            let diff : i32 = report[i + 1] - report[i];

            // Two conditions:
            // 1. The levels are either all increasing or all decreasing
            // 2. Any two adjacent levels differ by at least one and at most three
            let abs_diff : i32 = diff.abs();
            if abs_diff < 1 || abs_diff > 3
            {
                is_safe = false;
                break;
            }

            if total_diff == 0
            {
                total_diff = diff;
            }
            else
            {
                if (total_diff.is_positive() && !diff.is_positive()) || (total_diff.is_negative() && !diff.is_negative())
                {
                    is_safe = false;
                    break;
                }
            }

        }

        if is_safe
        {
            println!("Report is safe! {report:?}");
            total_safe += 1;
        }
    }

    return total_safe;
}

fn AnalyzeReport(report : &Vec<i32>) -> bool
{
    let mut total_diff : i32 = 0;
    let mut is_safe : bool = true;
    for i in 0..report.len() - 1
    {
        let diff : i32 = report[i + 1] - report[i];

        // Two conditions:
        // 1. The levels are either all increasing or all decreasing
        // 2. Any two adjacent levels differ by at least one and at most three
        let abs_diff : i32 = diff.abs();
        if abs_diff < 1 || abs_diff > 3
        {
            is_safe = false;
            break;
        }

        if total_diff == 0
        {
            total_diff = diff;
        }
        else
        {
            if (total_diff.is_positive() && !diff.is_positive()) || (total_diff.is_negative() && !diff.is_negative())
            {
                is_safe = false;
                break;
            }
        }

    }

    return is_safe;
}

fn Part2() -> i32
{
    let file_path = "../src/day2/input1_2.txt";
    let contents = fs::read_to_string(file_path).expect("Failed to open input file!");

    let mut reports : Vec<Vec<i32>> = Vec::new();
    for (i, report) in contents.split("\n").enumerate()
    {
        let mut curr_report = Vec::new();
        for level in report.split(" ")
        {
            curr_report.push(level.trim().parse::<i32>().unwrap());
        }

        reports.push(curr_report);
    }

    let mut total_safe : i32 = 0;

    for report in reports
    {
        let is_safe : bool = AnalyzeReport(&report);
        
        // If report doesn't pass, try removing one entry at a time to see if it passes
        if !is_safe
        {
            for i in 0..report.len()
            {
                let mut modified_report : Vec<i32> = report.clone();
                modified_report.remove(i);

                let is_safe_with_modifications : bool = AnalyzeReport(&modified_report);
                if is_safe_with_modifications
                {
                    total_safe += 1;
                    break;
                }
            }
        }
        else
        {
            total_safe += 1;
        }
    }

    return total_safe;
}

fn main()
{
    let safe_reports : i32 = Part2(); 
    println!("Found {safe_reports} safe reports!");
}