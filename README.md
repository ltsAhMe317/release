Release
auto release tool

how to use 

create file release.toml

release.toml:

shell = ["cargo build --release"]
[[targets]]
name = "release-linux"
path = ["./target/release/release"]
