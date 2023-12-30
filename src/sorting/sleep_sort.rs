use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub fn sleep_sort(vec: &[usize]) -> Vec<usize> {
    // Get the length of the input vector
    let len = vec.len();

    // Create a channel for communication between threads
    let (tx, rx) = mpsc::channel();

    // Iterate over each element (denoted by &x) in the input vector using a for_each loop
    vec.iter().for_each(|&x| {
        // Clone the sender (tx) for each thread to send data to the receiver
        let tx = tx.clone();

        // Spawn a new thread for each element in the vector
        thread::spawn(move || {
            // Simulate a delay based on the value of the element
            thread::sleep(Duration::from_millis((20 * x) as u64));

            // Send the element to the receiver through the channel
            tx.send(x).expect("Failed to send data");
        });
    });

    // Collect the sorted elements from the receiver into a vector and return it
    rx.into_iter().take(len).collect()
}

fn main() {
    let vec = vec![5, 3, 7, 10, 1, 0, 8];
    let sorted_vec = sleep_sort(&vec);
    println!("{:?}", sorted_vec);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_element() {
        let res = sleep_sort(&[1]);
        assert_eq!(res, &[1]);
    }

    #[test]
    fn unsorted_array() {
        let res = sleep_sort(&[3, 4, 2, 1]);
        assert_eq!(res, &[1, 2, 3, 4]);
    }

    #[test]
    fn repeated_elements() {
        let res = sleep_sort(&[1, 1, 1, 1]);
        assert_eq!(res, &[1, 1, 1, 1]);
    }

    #[test]
    fn random_elements() {
        let res = sleep_sort(&[5, 3, 7, 10, 1, 0, 8]);
        assert_eq!(res, &[0, 1, 3, 5, 7, 8, 10]);
    }

    #[test]
    fn empty() {
        let res = sleep_sort(&[]);
        assert_eq!(res, &[]);
    }

    #[test]
    fn large_number_of_elements() {
        let res = sleep_sort(&[1; 1000]);
        assert_eq!(res, &[1; 1000]);
    }

    #[test]
    fn error() {
        let res = sleep_sort(&[1, 2, 3, 4]);
        assert_ne!(res, &[1, 2, 3, 5]);
    }
}
