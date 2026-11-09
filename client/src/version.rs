/// The distributor of this Celler client.
///
/// Common values include `nixpkgs`, `celler` and `dev`.
pub const CELLER_DISTRIBUTOR: &str = if let Some(distro) = option_env!("CELLER_DISTRIBUTOR") {
    distro
} else {
    "unknown"
};
