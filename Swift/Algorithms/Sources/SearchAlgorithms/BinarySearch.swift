/// Searches for an element in a sorted array using binary search and returns its index.
///
/// The given array must be sorted in ascending order. If it is not sorted, the result may be incorrect.
/// `nil` is returned if the element is not found in the array.
///
/// - Parameter array: A sorted ascending array of elements that conform to `Comparable`.
/// - Parameter target: The element to search for, conforming to `Comparable`.
/// - Returns: The index of the element if it is found; otherwise, `nil`.
public func binarySearch<T: Comparable>(_ array: [T], _ target: T) -> Int? {
    var low = 0 // The lower bound of the search range.
    var high = array.count // The upper bound of the search range.

    while low < high { // Search is finished when bounds meet each other.
        let mid = low + (high - low) / 2 // The midpoint index of the current search range.
                                              // '(high + low) / 2' can cause overflow in edge case.

        if array[mid] == target { // The answer we need! Target matches the midpoint value.
            return mid
        } else if array[mid] < target { // If the element at the midpoint is less than the target element.
            low = mid + 1 // Then search in the right half of the array.
        } else { // If the element at the midpoint is greater than the target element.
            high = mid // Then search in the left half of the array.
        }
    }

    return nil
}