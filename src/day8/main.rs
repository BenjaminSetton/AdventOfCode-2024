
use std::fs;
use std::collections::HashMap;
use itertools::Itertools;

type MapType = Vec<Vec<u8>>;

fn PrintMap(map : &MapType)
{
    for line in map
    {
        for c in line
        {
            print!("{}", *c as char);
        }

        print!("\n");
    }
}

fn IsAntenna(c : char) -> bool
{
    c != '.'
}

fn AreEqual(pt0 : (i32, i32), pt1 : (i32, i32)) -> bool
{
    return (pt0.0 == pt1.0) && (pt0.1 == pt1.1);
}

fn Part1() -> i32
{
    let file_path = "../../src/day8/input8_1.txt";
    let mut contents : String = fs::read_to_string(file_path).expect("Failed to open input file!");

    let mut map : MapType = Vec::new();
    for line in contents.lines()
    {
        map.push(Vec::from(line.as_bytes()));
    }

    PrintMap(&map);

    // Build a map of antennas to their coords
    let mut y : i32 = 0;
    let mut antennas : HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    for line in &map
    {
        let mut x : i32 = 0;
        for c in line
        {
            let chr : char = *c as char;

            if IsAntenna(chr)
            {
                if !antennas.contains_key(&chr)
                {
                    // Create a new vector
                    antennas.insert(chr, Vec::new());
                }

                antennas.get_mut(&chr).unwrap().push((x, y));
            }

            x += 1;
        }

        y += 1;
    }

    //println!("{antennas:?}");

    let mut result : MapType = vec![vec!['.' as u8; map.len()]; map[0].len()];

    // Loop over all frequencies
    for freq in antennas.keys()
    {
        let antennaPositions = antennas.get(freq);

        // Loop over every point in the map
        let mut y : i32 = 0;
        for line in &map
        {
            let mut x : i32 = 0;
            for c in line
            {
                let chr : char = *c as char;
                if chr == *freq
                {
                    continue;
                }

                let currPoint : (i32, i32) = (x, y);

                // Loop over all line permutations
                let antennaPairs = antennaPositions.unwrap().iter().permutations(2);
                for antennaPair in antennaPairs
                {
                    let a0 : (i32, i32) = (antennaPair[0].0, antennaPair[0].1);
                    let a1 : (i32, i32) = (antennaPair[1].0, antennaPair[1].1);

                    // Vector pointing from a0 to a1
                    let distAntennas : (i32, i32) = (a1.0 - a0.0, a1.1 - a0.1);
                    let potential0 : (i32, i32) = (a0.0 - distAntennas.0, a0.1 - distAntennas.1);
                    let potential1 : (i32, i32) = (a1.0 + distAntennas.0, a1.1 + distAntennas.1);
                    if AreEqual(currPoint, potential0) || AreEqual(currPoint, potential1)
                    {
                        result[y as usize][x as usize] = 'X' as u8;
                    }
                }

                x += 1;
            }

            y += 1;
        }
    }

    println!("Result:");
    PrintMap(&result);

    let mut total : i32 = 0;
    for line in result
    {
        for c in line
        {
            if c == 'X' as u8
            {
                total += 1;
            }
        }
    }

    return total;
}

fn Part2() -> i32
{
    let file_path = "../../src/day8/input8_2.txt";
    let mut contents : String = fs::read_to_string(file_path).expect("Failed to open input file!");

    let mut map : MapType = Vec::new();
    for line in contents.lines()
    {
        map.push(Vec::from(line.as_bytes()));
    }

    PrintMap(&map);

    // Build a map of antennas to their coords
    let mut y : i32 = 0;
    let mut antennas : HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    for line in &map
    {
        let mut x : i32 = 0;
        for c in line
        {
            let chr : char = *c as char;

            if IsAntenna(chr)
            {
                if !antennas.contains_key(&chr)
                {
                    // Create a new vector
                    antennas.insert(chr, Vec::new());
                }

                antennas.get_mut(&chr).unwrap().push((x, y));
            }

            x += 1;
        }

        y += 1;
    }

    //println!("{:?}", antennas.keys());

    let mut result : MapType = vec![vec!['.' as u8; map.len()]; map[0].len()];

    // Loop over all frequencies
    for freq in antennas.keys()
    {
        let antennaPositions = antennas.get(freq);

        // Loop over every point in the map
        let mut y : i32 = 0;
        for line in &map
        {
            let mut x : i32 = 0;
            for c in line
            {
                let chr : char = *c as char;
                if chr == *freq
                {
                    continue;
                }

                let currPoint : (i32, i32) = (x, y);

                // Loop over all line permutations
                let antennaPairs = antennaPositions.unwrap().iter().permutations(2);
                for antennaPair in antennaPairs
                {
                    let a0 : (i32, i32) = (antennaPair[0].0, antennaPair[0].1);
                    let a1 : (i32, i32) = (antennaPair[1].0, antennaPair[1].1);

                    if AreEqual(a0, currPoint) || AreEqual(a1, currPoint)
                    {
                        result[y as usize][x as usize] = 'X' as u8;
                        break;
                    }
                    else 
                    {
                        let currPoint : (f32, f32) = (x as f32, y as f32);

                        let a0 : (f32, f32) = (antennaPair[0].0 as f32, antennaPair[0].1 as f32);
                        let a1 : (f32, f32) = (antennaPair[1].0 as f32, antennaPair[1].1 as f32);

                        // Dot two vectors: one pointing from currPoint to any antenna, and another representing the antenna line
                        // If 0 then they're colinear and at an anti-node
                        let mut vec0 : (f32, f32) = ((a0.0 - currPoint.0) as f32, (a0.1 - currPoint.1) as f32);
                        let len0 : f32 = f32::sqrt(f32::powf(vec0.0, 2.0) + f32::powf(vec0.1, 2.0));
                        vec0 = (vec0.0 / len0, vec0.1 / len0);
    
                        let mut vec1 : (f32, f32) = ((a1.0 - a0.0) as f32, (a1.1 - a0.1) as f32);
                        let len1 : f32 = f32::sqrt(f32::powf(vec1.0, 2.0) + f32::powf(vec1.1, 2.0));
                        vec1 = (vec1.0 / len1, vec1.1 / len1);

                        // {
                        //     let tl : f32 = f32::sqrt(f32::powf(vec1.0, 2.0) + f32::powf(vec1.1, 2.0));
                        //     println!("{tl}");
                        // }
    
                        let dot : f32 = (vec0.0 * vec1.0) + (vec0.1 * vec1.1);
                        if f32::abs(dot) >= (1.0 - f32::EPSILON)
                        {
                            result[y as usize][x as usize] = 'X' as u8;
                            break;
                        }
                    }


                }

                x += 1;
            }

            y += 1;
        }
    }

    println!("Result:");
    PrintMap(&result);

    let mut total : i32 = 0;
    for line in result
    {
        for c in line
        {
            if c == 'X' as u8
            {
                total += 1;
            }
        }
    }

    return total;
}

fn main() 
{
    // let res1 : i32 = Part1();
    // println!("Part 1 res: {res1}");

    let res2 : i32 = Part2();
    println!("Part 2 res: {res2}");
}
