use semver::{BuildMetadata, Prerelease};

fn read_arg_version() -> String {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        panic!("Invalid arguments. Usage: vincrementer version_string");
    }
    args[1].clone()
}

fn increment_patch_semantic(mut version: semver::Version) -> String {
    version.patch = version.patch + 1;
    version.build = BuildMetadata::EMPTY;
    version.pre = Prerelease::EMPTY;
    version.to_string()
}

fn increment_patch_arbitrary(version: &String) -> String {
    match version.rsplit_once('.') {
        None => {
            let mut result = String::new();
            result.push_str(&version);
            result.push_str(".1");
            result
        }
        Some((prefix, postfix)) => {
            let mut result = String::new();
            result.push_str(prefix);
            match postfix.parse::<i32>() {
                Ok(number) => {
                    result.push_str(".");
                    let number = number + 1;
                    result.push_str(&number.to_string());
                    result
                },
                Err(_err) => {
                    result.push_str(".");
                    result.push_str(postfix);
                    result.push_str(".1");
                    result
                }
            }
        }
    }
}

fn increment_patch(version: &String) -> String {
    match semver::Version::parse(version) {
        Ok(semantic_version) => increment_patch_semantic(semantic_version),
        Err(_err) => increment_patch_arbitrary(version),
    }
}

fn main() {
    let input_version = read_arg_version();
    let output_version = increment_patch(&input_version);
    println!("{output_version}");
}
