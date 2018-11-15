pub fn solve(input: i32) -> i32 {

    /*
    Explanation needed:
    The loop finds two variables: ring and ring_start
        - ring: nth ring = the desired number is on ring n
        - ring_start = the first (lowest) value on this ring
    Then, we find the 4 values of the new ring which are at the center of the sides.
    Afterwards, we calculate the distance of our number to the 4 values.
    We just care for the lowest one (We need to go that many steps to reach one axis).
    Then we walk ring-1 steps to reach the center.
    */

    let mut ii = 1;
    let mut numbers = 1;
    let (ring, ring_start) = loop {
        ii += 1;
        let side_length = 2 * ii - 1;
        // number of new cells in this ring
        // calculated as big square - smaller_square
        let new = side_length * side_length - (side_length - 2) * (side_length -2);
        numbers += new;
        if numbers >= input {
            break (ii, numbers - new + 1);
        }
    };

    let  mids = [   ring_start + (ring - 2),
                    ring_start + (ring - 2) + (2 * ring - 2),
                    ring_start + (ring - 2) + 2 * (2 * ring - 2),
                    ring_start + (ring - 2) + 3 * (2 * ring - 2)
                ];
    let mut diffs: Vec<i32> = mids.iter().map(|x| i32::abs(input - x)).collect();
    diffs.sort();

    ring - 1 + diffs.remove(0)
}
