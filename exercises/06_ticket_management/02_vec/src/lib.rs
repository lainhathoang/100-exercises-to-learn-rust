// Given a number `n`, return the `n+1`th number in the Fibonacci sequence.
//
// The Fibonacci sequence is defined as follows:
//
// - The first number of the sequence is 0.
// - The second number of the sequence is 1.
// - Every subsequent number is the sum of the two preceding numbers.
//
// So the sequence goes: 0, 1, 1, 2, 3, 5, 8, 13, 21, and so on.
//
// We expect `fibonacci(0)` to return `0`, `fibonacci(1)` to return `1`,
// `fibonacci(2)` to return `1`, and so on.
pub fn fibonacci(n: u32) -> u32 {
    // TODO: implement the `fibonacci` function
    //
    // Hint: use a `Vec` to memoize the results you have already calculated
    // so that you don't have to recalculate them several times.
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        let mut arr: Vec<u32> = Vec::new();
        let mut i = 0;

        while i <= n {
            if i == 0 {
                arr.push(0);
            } else if i == 1 {
                arr.push(1);
            } else {
                // let value = arr.get(i as usize). - arr.get(i as usize - 1).unwrap();
                // let value = arr[i as usize - 1] + arr[i as usize - 2];
                let value = match (arr.get(i as usize - 1), arr.get(i as usize - 2)) {
                    (Some(&a), Some(&b)) => a + b,
                    _ => 0,
                };
                arr.push(value);
                // arr.push(arr[i as usize - 1] - arr[i as usize - 2]);
            }
            i += 1;
        }
        *arr.last().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::fibonacci;

    #[test]
    fn first() {
        assert_eq!(fibonacci(0), 0);
    }

    #[test]
    fn second() {
        assert_eq!(fibonacci(1), 1);
    }

    #[test]
    fn third() {
        assert_eq!(fibonacci(2), 1);
    }

    #[test]
    fn tenth() {
        assert_eq!(fibonacci(10), 55);
    }

    #[test]
    fn thirtieth() {
        assert_eq!(fibonacci(30), 832040);
    }
}
