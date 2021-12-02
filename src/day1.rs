/*
The first order of business is to figure out how quickly the depth increases, just so you know what you're dealing with - you never know if the keys will get carried into deeper water by an ocean current or a fish or something.

Solution: 1754
*/
pub(crate) fn solve_part_one(input: Vec<i32>) {
    let mut previous: i32 = i32::MAX;

    let mut counter: i32 = 0;
    for i in input {
        if i > previous {
            counter = counter + 1;
        }

        previous = i;
    }

    println!("{} measurements increased from the previous one", counter)
}
/*
Considering every single measurement isn't as useful as you expected: there's just too much noise in the data.

Instead, consider sums of a three-measurement sliding window. Again considering the above example:

199  A
200  A B
208  A B C
210    B C D
200  E   C D
207  E F   D
240  E F G
269    F G H
260      G H
263        H
Start by comparing the first and second three-measurement windows. The measurements in the first window are marked A (199, 200, 208); their sum is 199 + 200 + 208 = 607. The second window is marked B (200, 208, 210); its sum is 618. The sum of measurements in the second window is larger than the sum of the first, so this first comparison increased.

Your goal now is to count the number of times the sum of measurements in this sliding window increases from the previous sum. So, compare A with B, then compare B with C, then C with D, and so on. Stop when there aren't enough measurements left to create a new three-measurement sum.


*/
pub(crate) fn solve_part_two(input: Vec<i32>) {
    let mut counter: i32 = 0;

    let mut previous: i32 = input[0] + input[1] + input[2];

    for i in 2..input.len() {
        let sum = input[i] + input[i - 1] + input[i - 2];
        // take the values of i, i + 1 and i + 2
        // check for out of bounds

        if sum > previous {
            counter = counter + 1;
        }

        previous = sum;
    }

    println!("{} measurements increased from the previous one", counter)
}
