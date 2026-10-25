// swift-tools-version: 6.4
// The swift-tools-version declares the minimum version of Swift required to build this package.

import PackageDescription

let package = Package(
    name: "Algorithms",
    products: [
        .library(
            name: "Algorithms",
            targets: ["SearchAlgorithms",
                      "SortAlgorithms"]
        ),
    ],
    targets: [
        .target(
            name: "SearchAlgorithms",
            swiftSettings: [
                .enableUpcomingFeature("ApproachableConcurrency"),
            ],
        ),
        .testTarget(
            name: "SearchAlgorithmsTests",
            dependencies: ["SearchAlgorithms"],
            swiftSettings: [
                .enableUpcomingFeature("ApproachableConcurrency"),
            ],
        ),

        .target(
            name: "SortAlgorithms",
            swiftSettings: [
                .enableUpcomingFeature("ApproachableConcurrency"),
            ],
        ),
        .testTarget(
            name: "SortAlgorithmsTests",
            dependencies: ["SortAlgorithms"],
            swiftSettings: [
                .enableUpcomingFeature("ApproachableConcurrency"),
            ],
        ),
    ]
)
