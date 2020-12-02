use std::cmp::Ordering;
use util::{read_lines, Result};

fn main() -> Result<()> {
    let solution1_result = solution_1()?;
    println!("Solution 1 result: {}", solution1_result);

    let solution2_result = solution_2()?;
    println!("Solution 2 result: {}", solution2_result);
    Ok(())
}

fn load_and_sort_values() -> Result<Vec<i32>> {
    let mut values = read_lines("input.txt")?
        .map(|line| line.parse::<i32>())
        .collect::<std::result::Result<Vec<_>, _>>()?;
    values.sort();
    Ok(values)
}

fn solution_1() -> Result<i32> {
    // Initialize a vector of integers sorted in ascending order.
    // - Index zero is considered the "bottom" of the vector
    // - The last index is considered the "top" of the vector
    let values = load_and_sort_values()?;

    // Hold two pointers into the values vector, one starting at the bottom and one starting at the
    // top. Then begin a loop:

    // - If the sum of the values is less than 2020, increment the bottom pointer.
    // - If the sum of the values is greater than 2020, decrement the top pointer.
    // - If the sum is equal to 2020, return the product of the two values.
    // - If the top pointer - the bottom pointer is less than two, return an error.
    let (mut bottom, mut top) = (0_usize, values.len() - 1);
    let total = loop {
        if top - bottom < 2 {
            // didn't find a solution
            break Err("No solution found");
        }

        let (low, high) = (values[bottom], values[top]);
        match 2020.cmp(&(low + high)) {
            Ordering::Greater => bottom += 1,
            Ordering::Equal => break Ok(low * high),
            Ordering::Less => top -= 1,
        }
    }?;
    Ok(total)
}

fn solution_2() -> Result<i32> {
    let mut values = load_and_sort_values()?;

    // As an optimization, pop elements so long as the last element plus the bottom two elements is
    // greater than 2020
    let bottom_sum = values[0] + values[1];
    while bottom_sum + values.last().copied().unwrap() > 2020 {
        values.pop();
    }

    for p_bottom in 0..values.len() - 3 {
        for p_mid in (p_bottom + 1)..values.len() - 2 {
            for p_top in (p_mid..(values.len() - 1)).rev() {
                let (bottom, mid, top) = (values[p_bottom], values[p_mid], values[p_top]);
                if bottom + mid + top == 2020 {
                    return Ok(bottom * mid * top);
                }
            }
        }
    }
    Err("No Solution")?
}
