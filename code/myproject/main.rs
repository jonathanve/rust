/*
    Workspaces: Interrelated packages that evolve together
    Packages: Cargo features to build, test, shara crates
    Crates: Set of modules that produces a binary or library
    Modules: Contain files and scope and paths are defined
    Paths: Naming of items and privacy

    Rules:
    - A package must contain at least 1 crate
    - A package can contain up to 1 binary crate and multiple binary crates
    - Crates can be shared in many projects since they encapsulate common functionality in a scope

    src/main.rs crate root of binary crate with same name as package
    src/lib.rs  crate root of library crate with same name as package
    src/bin dir multiple binary crates here
*/
fn main() {
    println!("Hi, Rust Project!");
}
