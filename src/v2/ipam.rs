// This is not a normal Rust module! It's included directly into v2.rs,
// possibly after build-time preprocessing.  See v2.rs for an explanation
// of how this works.

/// Information about a network's IPAM settings.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct Ipam {
    /// The IPAM config blocks
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub config: Vec<IpamConfig>,

    //XXXkhuey TODO: driver, options.

    /// PRIVATE.  Mark this struct as having unknown fields for future
    /// compatibility.  This prevents direct construction and exhaustive
    /// matching.  This needs to be be public because of
    /// http://stackoverflow.com/q/39277157/12089
    #[doc(hidden)]
    #[serde(default, skip_serializing, skip_deserializing)]
    pub _hidden: (),
}

derive_standard_impls_for!(Ipam, {
    config, _hidden
});

/// An IPAM config block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct IpamConfig {
    /// The subject in CIDR format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet: Option<RawOr<String>>,

    /// The IPv4 or IPv6 gateway for the master subnet.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway: Option<RawOr<String>>,

    /// PRIVATE.  Mark this struct as having unknown fields for future
    /// compatibility.  This prevents direct construction and exhaustive
    /// matching.  This needs to be be public because of
    /// http://stackoverflow.com/q/39277157/12089
    #[doc(hidden)]
    #[serde(default, skip_serializing, skip_deserializing)]
    pub _hidden: (),
}

derive_standard_impls_for!(IpamConfig, {
    subnet, gateway, _hidden
});

