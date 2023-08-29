pub mod other;

pub fn parse_version(version_str: &str) -> semver::VersionReq {
    semver::VersionReq::parse(version_str).unwrap()
}
