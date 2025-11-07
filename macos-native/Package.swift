// swift-tools-version:5.9
import PackageDescription

let package = Package(
    name: "BossmanNative",
    platforms: [
        .macOS(.v13)
    ],
    products: [
        .executable(name: "BossmanNative", targets: ["BossmanNative"])
    ],
    dependencies: [],
    targets: [
        .executableTarget(
            name: "BossmanNative",
            dependencies: []
        )
    ]
)
