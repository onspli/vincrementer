use semver::{BuildMetadata, Prerelease};

fn read_arg_version() -> String {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        panic!("Invalid arguments. Usage: vincrementer version_string");
    }
    args[1].clone()
}

fn increment_patch_semantic(input_version: &mut semver::Version) -> String {
    input_version.patch = input_version.patch + 1;
    input_version.build = BuildMetadata::EMPTY;
    input_version.pre = Prerelease::EMPTY;
    input_version.to_string()
}

fn increment_patch_arbitrary(_input_version: &String) -> String {
    String::from("not semantic")
}

fn increment_patch(input_version: &String) -> String {
    match semver::Version::parse(input_version) {
        Ok(mut version) => increment_patch_semantic(&mut version),
        Err(_err) => increment_patch_arbitrary(input_version),
    }
}

fn main() {
    let input_version = read_arg_version();
    let output_version = increment_patch(&input_version);
    println!("{output_version}");
}
