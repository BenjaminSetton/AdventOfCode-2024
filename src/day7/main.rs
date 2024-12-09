
use std::fs;
use itertools::Itertools;

// Credits: ChatGPT
fn permutations_with_repetition(elements: &[char], k: usize) -> Vec<Vec<char>> {
    if k == 0 {
        return vec![Vec::new()];
    }

    let mut result = Vec::new();
    for &element in elements {
        for mut sub_permutation in permutations_with_repetition(elements, k - 1) {
            sub_permutation.push(element);
            result.push(sub_permutation);
        }
    }

    result
}

fn count_num_digits(num : i64) -> i64
{
    let mut num_cpy : i64 = num.clone();
    let mut digits : i64 = 1;
    while num_cpy >= 10
    {
        num_cpy /= 10;
        digits += 1;
    }

    digits
}

fn Part2() -> i64
{
        let file_path = "../../src/day7/input7_2.txt";
    let mut contents : String = fs::read_to_string(file_path).expect("Failed to open input file!");

    let mut equations : Vec<(i64, Vec<i64>)> = Vec::new();
    for line in contents.lines()
    {
        let mut parsedLine = line.split(':');
        
        // Iterator values are consumed when retrieved. That's why we use nth(0) in both cases, since the first value
        // will get popped off once retrieved, making the second value reside in nth(0) as well!
        let res : i64 = parsedLine.nth(0).unwrap().parse().unwrap();
        let values : Vec<i64> = parsedLine.nth(0).unwrap().split_whitespace().map(|s| s.parse().unwrap()).collect();
        equations.push((res, values.clone()));

        println!("{res}: {values:?}");
    }

    let mut totalSum : i64 = 0;

    let ops : Vec<char> = vec!['+', '*', '|'];
    for eqPair in equations
    {
        let result : i64 = eqPair.0;
        let values : Vec<i64> = eqPair.1;
        println!("Testing combinations for {result}");

        //let opsIter = ops.clone().into_iter().cartesian_product(values.iter());
        let opsVec = permutations_with_repetition(&ops, values.len() - 1);
        for ops in opsVec
        {
            let mut currentRes : i64 = values[0];
            let mut i : i64 = 0;
            for op in &ops
            {
                let operandL : i64 = currentRes;
                let operandR : i64 = values[(i + 1) as usize];
                let op : char = ops[i as usize];
    
                if op == '+'
                {
                    currentRes = (operandL + operandR);
                    //println!("Adding {operandL} to {operandR} = {currentRes}");
                }
                else if op == '*'
                {
                    currentRes = (operandL * operandR);
                    //println!("Multiplying {operandL} to {operandR} = {currentRes}");
                }
                else // '||', shortened to char '|' for simplicity
                {
                    currentRes = (currentRes * i64::pow(10, count_num_digits(operandR) as u32) + operandR);
                    //println!("Concat {operandL} to {operandR} = {currentRes}");
                }
    
                i += 1;
            }

            if currentRes == result
            {
                // Possible to calculate result from combination of operations!
                println!("Found good combo for {result}");
                totalSum += result;
                break;
            }
        }
    }

    return totalSum;
}

fn Part1() -> i64
{
    let file_path = "../../src/day7/input7_1.txt";
    let mut contents : String = fs::read_to_string(file_path).expect("Failed to open input file!");

    let mut equations : Vec<(i64, Vec<i64>)> = Vec::new();
    for line in contents.lines()
    {
        let mut parsedLine = line.split(':');
        
        // Iterator values are consumed when retrieved. That's why we use nth(0) in both cases, since the first value
        // will get popped off once retrieved, making the second value reside in nth(0) as well!
        let res : i64 = parsedLine.nth(0).unwrap().parse().unwrap();
        let values : Vec<i64> = parsedLine.nth(0).unwrap().split_whitespace().map(|s| s.parse().unwrap()).collect();
        equations.push((res, values.clone()));

        println!("{res}: {values:?}");
    }

    let mut totalSum : i64 = 0;

    let ops : Vec<char> = vec!['+', '*'];
    for eqPair in equations
    {
        let result : i64 = eqPair.0;
        let values : Vec<i64> = eqPair.1;
        println!("Testing combinations for {result}");

        //let opsIter = ops.clone().into_iter().cartesian_product(values.iter());
        let opsVec = permutations_with_repetition(&ops, values.len() - 1);
        for ops in opsVec
        {
            //println!("{ops:?}");

            let mut currentRes : i64 = values[0];
            let mut i : i64 = 0;
            for op in &ops
            {
                let operandL : i64 = currentRes;
                let operandR : i64 = values[(i + 1) as usize];
                let op : char = ops[i as usize];
    
                if op == '+'
                {
                    currentRes = (operandL + operandR);
                    //println!("Adding {operandL} to {operandR} = {currentRes}");
                }
                else // '*'
                {
                    currentRes = (operandL * operandR);
                    //println!("Multiplying {operandL} to {operandR} = {currentRes}");
                }
    
                i += 1;
            }

            if currentRes == result
            {
                // Possible to calculate result from combination of operations!
                println!("Found good combo for {result}");
                totalSum += result;
                break;
            }
        }
    }

    return totalSum;
}

fn main() {
    // let res1 : i64 = Part1();
    // println!("Part 1 res: {res1}");

    let res2 : i64 = Part2();
    println!("Part 2 res: {res2}");
}
