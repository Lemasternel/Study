#!/usr/bin/env swift

//https://leetcode.com/problems/two-sum?envType=problem-list-v2&envId=02blwr75
func twoSum(_ nums: [Int], _ target: Int) -> [Int] {
    
    var candidates: [Int:Int] = [:] // Index is the value we need to complete the sum.
                                    // Value stores the actual index.

    for (index, value) in nums.enumerated() {
        let valueNeededToCompleteTheSum = target-value

        // Check if the value we need is already in the candidates.
        if let indexMatchedToTheValueNeeded = candidates[value] {
            return [indexMatchedToTheValueNeeded, index]
        }

        candidates[valueNeededToCompleteTheSum] = index
    }
    return []

}