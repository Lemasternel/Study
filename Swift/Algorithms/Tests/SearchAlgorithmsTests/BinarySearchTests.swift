import Testing
@testable import SearchAlgorithms

@Test func int_binary_search_return_a_valid_index_for_existing_element() async throws {
    let array = [-2,-1,0,1,2]

    #expect(binarySearch(array, -2) == 0)
    #expect(binarySearch(array, -1) == 1)
    #expect(binarySearch(array, 0) == 2)
    #expect(binarySearch(array, 1) == 3)
    #expect(binarySearch(array, 2) == 4)
}

@Test func int_binary_search_return_nil_for_non_existing_element() async throws {
    let array = [-2,-1,0,1,2]

    #expect(binarySearch(array, -2372) == nil)
    #expect(binarySearch(array, 2372) == nil)
}

@Test func char_binary_search_return_a_valid_index_for_existing_element() async throws {
    let array: [Character] = ["a","b","c","d","e"]

    #expect(binarySearch(array, "a") == 0)
    #expect(binarySearch(array, "b") == 1)
    #expect(binarySearch(array, "c") == 2)
    #expect(binarySearch(array, "d") == 3)
    #expect(binarySearch(array, "e") == 4)
}

@Test func char_binary_search_return_nil_for_non_existing_element() async throws {
    let array: [Character] = ["b","c","d","e"]

    #expect(binarySearch(array, "a") == nil)
    #expect(binarySearch(array, "z") == nil)
    #expect(binarySearch(array, "f") == nil)
    #expect(binarySearch(array, "2") == nil)
    #expect(binarySearch(array, "@") == nil)
}

@Test func double_binary_search_return_a_valid_index_for_existing_element() async throws {
    let array: [Double] = [-2.5,-1.0,0.2,1.3,2.0]

    #expect(binarySearch(array, -2.5) == 0)
    #expect(binarySearch(array, -1.0) == 1)
    #expect(binarySearch(array, 0.2) == 2)
    #expect(binarySearch(array, 1.3) == 3)
    #expect(binarySearch(array, 2.0) == 4)
}

@Test func double_binary_search_return_nil_for_non_existing_element() async throws {
    let array: [Double] = [-2.5,-1.0,0.2,1.3,2.0]

    #expect(binarySearch(array, -2372.2) == nil)
    #expect(binarySearch(array, -2.0) == nil)
    #expect(binarySearch(array, 0) == nil)
    #expect(binarySearch(array, 1.5) == nil)
}

@Test func binary_search_return_nil_for_empty_array() async throws {
    let array: [Int] = []

    #expect(binarySearch(array, 0) == nil)
}

@Test func binary_search_handle_single_element_array() async throws {
    let array = [7]

    #expect(binarySearch(array, 7) == 0)
    #expect(binarySearch(array, 6) == nil)
    #expect(binarySearch(array, 8) == nil)
}

@Test func binary_search_distinguish_index_zero_from_nil() async throws {
    let array = [1,2,3]

    #expect(binarySearch(array, 1) == 0)
    #expect(binarySearch(array, 0) == nil)
}

@Test func binary_search_return_a_matching_index_for_duplicate_elements() async throws {
    let array = [1,2,2,2,3]

    let result = try #require(binarySearch(array, 2))
    #expect((1...3).contains(result))
}