# Bucket Sort

## Overview
Bucket Sort is a distribution-based sorting algorithm that divides input elements into buckets and sorts each bucket individually.

## Steps
1. Create `n` empty buckets.
2. Distribute numbers into buckets based on value ranges.
3. Sort each bucket individually.
4. Merge all buckets to form the final sorted list.

## Time Complexity
| Case | Time |
|------|------|
| Best | O(n + k) |
| Average | O(n + k) |
| Worst | O(n²) |
| Space | O(n + k) |

## Rust Example

```rust
pub fn bucket_sort(arr: &mut [f32]) {
    let n = arr.len();
    let mut buckets: Vec<Vec<f32>> = vec![Vec::new(); n];

    for &value in arr.iter() {
        let index = (value * n as f32) as usize;
        buckets[index.min(n - 1)].push(value);
    }

    for bucket in buckets.iter_mut() {
        bucket.sort_by(|a, b| a.partial_cmp(b).unwrap());
    }

    let mut idx = 0;
    for bucket in buckets {
        for value in bucket {
            arr[idx] = value;
            idx += 1;
        }
    }
}
