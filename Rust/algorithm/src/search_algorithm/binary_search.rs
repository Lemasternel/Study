//! Binary search over sorted slices.
//!
//! Binary search finds the position of a value in a slice that is already
//! sorted in ascending order by repeatedly halving the range that may still
//! contain the target.

use std::cmp::Ordering;

/// Searches a sorted slice for `target` using binary search.
///
/// The slice must be sorted in ascending order according to [`Ord`]. If it is
/// not, the result is unspecified: the function may return `None` even when
/// `target` is present.
///
/// # Arguments
///
/// * `array` - The sorted slice to search.
/// * `target` - The value to look for.
///
/// # Returns
///
/// * `Some(index)` - the index of an element equal to `target`. If several
///   elements are equal, any one of their indices may be returned.
/// * `None` - `target` is not in `array`.
///
/// # Complexity
///
/// Runs in *O(log n)* time and uses *O(1)* extra space, where `n` is the
/// length of `array`.
///
/// # Examples
///
/// ```
/// use algorithms::search_algorithm::binary_search::search;
/// let numbers = [1, 3, 5, 7, 9, 11];
/// assert_eq!(search(&numbers, &7), Some(3));
/// assert_eq!(search(&numbers, &4), None);
/// ```
pub fn search<T: Ord> (array: &[T], target: &T) -> Option<usize> {
    // Half-open search range `[low, high)` that may still contain `target`.
    let mut low: usize = 0;
    let mut high: usize = array.len();

    while low < high {
        // Index of the middle element of the current range.
        let mid: usize = low + (high - low) / 2;

        match array[mid].cmp(target) {
            // The middle element is smaller, so the target can only be to the right.
            Ordering::Less => low = mid + 1,
            // The middle element is larger, so the target can only be to the left.
            Ordering::Greater => high = mid,
            Ordering::Equal => return Some(mid),
        }
    }

    None
}
