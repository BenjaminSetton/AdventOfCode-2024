
use std::fs;

type MapType = Vec<Vec<u8>>;

fn IsGuard(c : char) -> bool
{
    return c == '^' || c == 'v' || c == '<' || c == '>';
}

fn IsObstacle(c : char) -> bool
{
    return c == '#' || c == 'O';
}

fn GetOffsetFromGuardDir(guardDir : char, off_x : &mut i32, off_y : &mut i32)
{
    match guardDir
    {
        '^' => { *off_x =  0; *off_y = -1; },
        'v' => { *off_x =  0; *off_y =  1; },
        '<' => { *off_x = -1; *off_y =  0; },
        '>' => { *off_x =  1; *off_y =  0; },
        _ => todo!()
    }
}

// Rotates by 90 degrees to the right
fn RotateGuardDir(currDir : char) -> char
{
    match currDir
    {
        '^' => '>',
        '>' => 'v',
        'v' => '<',
        '<' => '^',
        _ => todo!()
    }
}

fn IsValidCoord(map : &MapType, x : i32, y : i32) -> bool
{
    let max_x : i32 = map[0].len() as i32;
    let max_y : i32 = map.len() as i32;
    
    return (x >= 0 && x < max_x) && (y >= 0 && y < max_y);
}

fn GetCharAt(map : &MapType, x : i32, y : i32) -> Option<char>
{
    if !IsValidCoord(map, x, y)
    {
        return None;
    }

    let c : char = map[y as usize][x as usize] as char;
    return Some(c);
}

fn SetCharAt(map : &mut MapType, x : i32, y : i32, newChar : char)
{
    if IsValidCoord(map, x, y)
    {
        map[y as usize][x as usize] = newChar as u8;
    }
}

// Returns true if guard is in map, false otherwise. If true is returned
// the guard's coords are returned as out parameters
fn FindGuard(map : &MapType, x : &mut i32, y : &mut i32) -> bool
{
    *x = 0;
    *y = 0;
    for line in map
    {
        *x = 0;
        for c in line
        {
            if IsGuard(*c as char)
            {
                return true;
            }
            *x += 1;
        }
        *y += 1;
    }

    return false;
}

fn SimulateStep(map : &mut MapType) -> bool
{
    // Find position of guard
    let mut x : i32 = 0;
    let mut y : i32 = 0;
    if !FindGuard(map, &mut x, &mut y)
    {
        // Guard is already off the map
        return true;
    }

    //println!("Checking chars at ({x}, {y})");
    let guard : char = GetCharAt(map, x, y).unwrap();
        
    // Move one step forward according to rules
    let mut off_x : i32 = 0;
    let mut off_y : i32 = 0;
    //println!("Found guard char of {guard}");
    GetOffsetFromGuardDir(guard, &mut off_x, &mut off_y);

    let new_x : i32 = x + off_x;
    let new_y : i32 = y + off_y;

    let nextSpot : Option<char> = GetCharAt(map, new_x, new_y);
    match nextSpot
    {
        None => 
        {
            // Guard will move off map. Replace guard char with 'X' and return
            SetCharAt(map, x, y, 'X');
            return true;
        }
        Some(c) =>
        {
            if IsObstacle(c)
            {
                // Turn right 90 degrees
                SetCharAt(map, x, y, RotateGuardDir(guard));
            }
            else 
            {
                // Mark old pos with 'X' and move guard forward one spot
                SetCharAt(map, x, y, 'X');
                SetCharAt(map, new_x, new_y, guard);
            }
        }
    }

    return false;
}

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

fn Part1() -> i32
{
    let file_path = "../../src/day6/input6_1.txt";
    let mut contents : String = fs::read_to_string(file_path).expect("Failed to open input file!");

    let mut map : MapType = MapType::new();
    let mut mapBinding = contents.clone();
    for line in mapBinding.lines()
    {
        map.push(Vec::from(line.to_ascii_lowercase().as_bytes()));
    }

    let mut isDone : bool = SimulateStep(&mut map);
    while !isDone
    {
        isDone = SimulateStep(&mut map);
    }

    println!("------------------------------------------");
    PrintMap(&map);
    println!("------------------------------------------");

    // Count number of 'X' in map
    let mut sum : i32 = 0;
    for line in map
    {
        for c in line
        {
            if c == 'X' as u8
            {
                sum += 1;
            }
        }
    }

    return sum;
}

fn Part2() -> i32
{
    let file_path = "../../src/day6/input6_2.txt";
    let mut contents : String = fs::read_to_string(file_path).expect("Failed to open input file!");

    let mut map : MapType = MapType::new();
    let mut mapBinding = contents.clone();
    for line in mapBinding.lines()
    {
        map.push(Vec::from(line.to_ascii_lowercase().as_bytes()));
    }

    let mapSize : u64 = (map.len() * map[0].len()) as u64;
    let mut numPossibleObstacles : u32 = 0;

    for (y, line) in map.iter().enumerate()
    {
        for (x, c) in line.iter().enumerate()
        {
            // Insert a new obstacle if possible
            let currChar : char = GetCharAt(&map, x as i32, y as i32).unwrap();
            if !IsObstacle(currChar) || !IsGuard(currChar)
            {
                let mut newMap : MapType = map.clone();
                SetCharAt(&mut newMap, x as i32, y as i32, 'O');

                let mut isDone : bool = SimulateStep(&mut newMap);
                let mut counter : u64 = 0;

                println!("({x}, {y}) - Calculating possible obstacle");

                // Loop until guard exits or we find an infinite loop. Super jank way of finding
                // it is to simply have a counter with a big magic number...lol
                while !isDone
                {
                    if counter > 5 * mapSize
                    {
                        println!("({x}, {y}) - Found possible obstacle!");
                        numPossibleObstacles += 1;
                        break;
                    }

                    isDone = SimulateStep(&mut newMap);
                    counter += 1;
                }
            }
        }
    }

    // println!("------------------------------------------");
    // PrintMap(&map);
    // println!("------------------------------------------");

    return numPossibleObstacles as i32;
}

fn main() {
    let res1 : i32 = Part1();
    println!("Part 1 res: {res1}");

    let res2 : i32 = Part2();
    println!("Part 2 res: {res2}");
}
