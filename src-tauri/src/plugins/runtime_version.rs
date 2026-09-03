//! Host runtime version gate for installed plugins.
//!
//! A plugin manifest may declare `min_runtime_version`, the first Tabularis
//! release that ships every host feature the plugin relies on. The registry
//! validates the field, but nothing stopped an older host from loading such a
//! plugin and failing later at runtime. This module refuses the plugin up
//! front with a message that names both versions.

use semver::Version;

/// Version of the running Tabularis host.
pub const HOST_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Verify that `host_version` satisfies a plugin's `min_runtime_version`.
///
/// `None` or an empty string means the plugin declares no floor. A floor or a
/// host version that is not valid semver is logged and treated as compatible:
/// the registry already validates the field, and a local typo must not brick
/// plugin loading. Comparison follows semver precedence, so a prerelease host
/// such as `0.23.0-nightly.1` does not satisfy a `0.23.0` floor.
pub fn check_min_runtime_version(
    plugin_id: &str,
    min_runtime_version: Option<&str>,
    host_version: &str,
) -> Result<(), String> {
    let Some(floor) = min_runtime_version
        .map(str::trim)
        .filter(|value| !value.is_empty())
    else {
        return Ok(());
    };

    let required = match Version::parse(floor.trim_start_matches('v')) {
        Ok(version) => version,
        Err(err) => {
            log::warn!(
                "Plugin '{}' declares a non-semver min_runtime_version {:?} ({}); skipping the runtime version check",
                plugin_id,
                floor,
                err
            );
            return Ok(());
        }
    };
    let host = match Version::parse(host_version.trim().trim_start_matches('v')) {
        Ok(version) => version,
        Err(err) => {
            log::warn!(
                "Host version {:?} is not semver ({}); skipping the runtime version check for plugin '{}'",
                host_version,
                err,
                plugin_id
            );
            return Ok(());
        }
    };

    if host < required {
        return Err(format!(
            "Plugin '{}' requires Tabularis {} or newer, but this is Tabularis {}. Update Tabularis to use this plugin.",
            plugin_id, required, host
        ));
    }
    Ok(())
}
