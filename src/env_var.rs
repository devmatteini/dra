// https://github.com/clap-rs/clap/blob/eadcc8f66c128272ea309fed3d53d45b9c700b6f/clap_builder/src/util/str_to_bool.rs#L2
const TRUE_LITERALS: [&str; 6] = ["y", "yes", "t", "true", "on", "1"];

pub fn boolean(name: &str) -> bool {
    std::env::var(name)
        .ok()
        .map(|x| {
            let value = x.to_lowercase();
            TRUE_LITERALS.contains(&value.as_str())
        })
        .unwrap_or(false)
}

pub fn string(name: &str) -> Option<String> {
    std::env::var(name)
        .ok()
        .and_then(|x| if x.is_empty() { None } else { Some(x) })
}
