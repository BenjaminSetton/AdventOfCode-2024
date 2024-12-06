
use std::fs;

static xmas_str : &str = "XMAS";

fn is_valid_coord(x : i32, y : i32, max_x : i32, max_y : i32) -> bool
{
    return (x >= 0 && x < max_x) && (y >= 0 && y < max_y);
}

fn find_xmas_rec(input : &Vec<&str>, x : i32, y : i32, off_x : i32, off_y : i32, max_x : i32, max_y : i32, curr_ix : i32, res : &mut i32)
{
    if curr_ix >= 3
    {
        // Found an XMAS!
        *res += 1;
        //println!("Found XMAS at ({x}, {y})! {res}");
        return;
    }

    let new_x : i32 = x + off_x;
    let new_y : i32 = y + off_y;
    if is_valid_coord(new_x, new_y, max_x, max_y)
    {
        let neighbor : char = input[new_y as usize].chars().nth(new_x as usize).unwrap();
        let new_ix : usize = (curr_ix as usize) + 1;

        if neighbor == xmas_str.chars().nth(new_ix).unwrap()
        {
            //println!("REC - Found neighbor at coords ({new_x}, {new_y}): '{neighbor}'");
            find_xmas_rec(& input, new_x, new_y, off_x, off_y, max_x, max_y, curr_ix + 1, res);
        }
    }
}

fn find_xmas(input : &Vec<&str>, x : i32, y : i32, max_x : i32, max_y : i32) -> i32
{
    let mut res : i32 = 0;

    // Search 8 spaces around (x, y)
    for off_y in -1..=1
    {
        for off_x in -1..=1
        {
            let new_x : i32 = x + off_x;
            let new_y : i32 = y + off_y;
            if (off_x == 0) && (off_y == 0)
            {
                // Skip ourselves
                continue;
            }

            if is_valid_coord(new_x, new_y, max_x, max_y)
            {
                let neighbor : char = input[new_y as usize].chars().nth(new_x as usize).unwrap();

                if neighbor == xmas_str.chars().nth(1).unwrap()
                {
                    //println!("Found neighbor at coords ({new_x}, {new_y}): '{neighbor}'");
                    find_xmas_rec(&input, new_x, new_y, off_x, off_y, max_x, max_y, 1, &mut res);
                }
            }
        }
    }

    return res;
}

fn Part1() -> i32
{
    let file_path = "../../src/day4/input4_1.txt";
    let mut contents = fs::read_to_string(file_path).expect("Failed to open input file!");

    let lines : Vec<_> = contents.split("\r\n").collect();

    let mut total_xmas : i32 = 0;
    let mut y : i32 = 0;
    let mut x : i32 = 0;

    for line in &lines
    {
        for c in line.chars()
        {
            // Start a search for "XMAS" if first is found
            if c == 'X'
            {
                total_xmas += find_xmas(&lines, x, y, lines.len() as i32, line.len() as i32);
            } 

            x = (x + 1) % (lines.len() as i32);
        }
        y = (y + 1) % (line.len() as i32);
    }

    return total_xmas;
}

//------------------------------------------------------------------

// Returns opposite char, where 'M' is opposite to 'S' and vice-versa
fn GetOppositeChar(c : char) -> char
{
    if (c == 'M') 
    { 
        return 'S';
    } 
    else 
    { 
        return 'M';
    };

    return ' ';
}

fn IsValidCornerChar(c : char) -> bool
{
    return (c == 'M' || c == 'S');
}

// Assumes valid bounds
fn GetCharAt(input : &Vec<&str>, x : i32, y : i32) -> char
{
    return (input[y as usize].chars().nth(x as usize).unwrap());
}

fn IsXMASCross(input : &Vec<&str>, x : i32, y : i32, max_x : i32, max_y : i32) -> bool
{
    // Check TL-BR diagonal first
    let tl_x : i32 = x - 1;
    let tl_y : i32 = y - 1;
    let mut is_TLBR_cross : bool = false;
    if is_valid_coord(tl_x, tl_y, max_x, max_y)
    {
        let tl : char = GetCharAt(&input, tl_x, tl_y);
        if IsValidCornerChar(tl)
        {
            // Now check if bottom-right has corresponding char
            let br_x : i32 = x + 1;
            let br_y : i32 = y + 1;
            if is_valid_coord(br_x, br_y, max_x, max_y)
            {
                let br : char = GetCharAt(&input, br_x, br_y);
                if br == GetOppositeChar(tl)
                {
                    is_TLBR_cross = true;
                }
            }
        }
    }

    // Then check TR-BL diagonal
    let tr_x : i32 = x + 1;
    let tr_y : i32 = y - 1;
    let mut is_TRBL_cross : bool = false;
    if is_valid_coord(tr_x, tr_y, max_x, max_y)
    {
        let tr : char = GetCharAt(&input, tr_x, tr_y);
        if IsValidCornerChar(tr)
        {
            // Now check if bottom-right has corresponding char
            let bl_x : i32 = x - 1;
            let bl_y : i32 = y + 1;
            if is_valid_coord(bl_x, bl_y, max_x, max_y)
            {
                let bl : char = GetCharAt(&input, bl_x, bl_y);
                if bl == GetOppositeChar(tr)
                {
                    is_TRBL_cross = true;
                }
            }
        }
    }

    return (is_TLBR_cross && is_TRBL_cross);
}

fn Part2() -> i32
{
    let file_path = "../../src/day4/input4_2.txt";
    let mut contents = fs::read_to_string(file_path).expect("Failed to open input file!");

    let lines : Vec<_> = contents.split("\r\n").collect();

    let mut total : i32 = 0;
    let mut y : i32 = 0;
    let mut x : i32 = 0;

    for line in &lines
    {
        for c in line.chars()
        {
            // Look for 'anchor'. This is the center of the cross
            if c == 'A'
            {
                if IsXMASCross(&lines, x, y, lines.len() as i32, line.len() as i32)
                {
                    total += 1;
                }
            }

            x = (x + 1) % (lines.len() as i32);
        }

        y = (y + 1) % (line.len() as i32);
    }

    return total;
}

fn main() {

    let p1 : i32 = Part1();
    println!("Part 1 answer: {p1}");

    let p2 : i32 = Part2();
    println!("Part 2 answer: {p2}");
}
