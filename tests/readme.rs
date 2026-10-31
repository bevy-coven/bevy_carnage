//! **The README says what the manifest says.**
//!
//! `README.md` is also the crate-root rustdoc (`src/lib.rs` is `#![doc = include_str!("../README.md")]`),
//! so a line in it goes to docs.rs and to crates.io as the install contract. Issue #6 found four of
//! them false at once: "Requires Bevy 0.19" over a manifest on 0.20, `bevy_carnage = "0.5"` over `0.7.0`,
//! `vfx` ticked as a default feature that is not one, and a Status paragraph naming `0.5.64`. Each is a fact
//! the manifest also states, so each is read from both files and compared here; a release that moves one
//! number fails `cargo test` until the README moves with it.
//!
//! Since `af13bb4` the README is the org guide's short page and three of the four facts live in the guide pages it
//! links: the install line is still in `README.md`; `Requires **Bevy X.Y**` and the feature table are in
//! `docs/guide/install.md`; the Status paragraph is in `docs/guide/history.md`. The assertions are the same.
//!
//! Read with `std::fs` and plain string scanning, the way `tests/leaf.rs` reads the manifest: a parser
//! dependency for four lines would be a second convention. Every extraction panics, naming the line it
//! could not find, rather than passing on a README that no longer states the fact at all.

use std::path::PathBuf;

fn read(name: &str) -> String {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(name);
    std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("cannot read {}: {e}", path.display()))
}

/// The lines of one `[section]` of the manifest, comments included, up to the next table header.
fn manifest_section<'a>(manifest: &'a str, header: &str) -> Vec<&'a str> {
    let mut inside = false;
    let mut lines = Vec::new();
    for line in manifest.lines() {
        if line.starts_with('[') {
            inside = line.trim_end() == header;
            continue;
        }
        if inside {
            lines.push(line);
        }
    }
    assert!(!lines.is_empty(), "Cargo.toml has no {header} section");
    lines
}

/// `"A.B"` of `"A.B.C"`.
fn major_minor(version: &str) -> String {
    let mut parts = version.split('.');
    match (parts.next(), parts.next()) {
        (Some(a), Some(b)) => format!("{a}.{b}"),
        _ => panic!("`{version}` is not a version with a major and a minor"),
    }
}

/// The text inside the first pair of quotes after `key` on `line`.
fn quoted_after(line: &str, key: &str) -> String {
    let at = line.find(key).unwrap_or_else(|| panic!("`{key}` is not on `{line}`"));
    let rest = &line[at + key.len()..];
    let open = rest.find('"').unwrap_or_else(|| panic!("no opening quote after `{key}` on `{line}`"));
    let rest = &rest[open + 1..];
    let close = rest.find('"').unwrap_or_else(|| panic!("no closing quote after `{key}` on `{line}`"));
    rest[..close].to_string()
}

/// `[package] version`, in full.
fn package_version(manifest: &str) -> String {
    let lines = manifest_section(manifest, "[package]");
    let line = lines
        .iter()
        .find(|l| l.starts_with("version = \""))
        .expect("Cargo.toml's [package] has no `version = \"..\"` line");
    quoted_after(line, "version = ")
}

/// The `version` of `bevy` under `[dependencies]` (not `[dev-dependencies]`, which is the full umbrella
/// for the examples), in full.
fn bevy_requirement(manifest: &str) -> String {
    let lines = manifest_section(manifest, "[dependencies]");
    let line = lines
        .iter()
        .find(|l| l.starts_with("bevy = {"))
        .expect("Cargo.toml's [dependencies] has no `bevy = { .. }` line");
    quoted_after(line, "version = ")
}

/// The names in `default = [..]` under `[features]`, sorted.
fn default_features(manifest: &str) -> Vec<String> {
    let lines = manifest_section(manifest, "[features]");
    let at = lines
        .iter()
        .position(|l| l.starts_with("default = ["))
        .expect("Cargo.toml's [features] has no `default = [..]`");
    let mut list = String::new();
    for line in &lines[at..] {
        list.push_str(line);
        if line.contains(']') {
            break;
        }
    }
    let mut names: Vec<String> =
        list.split('"').skip(1).step_by(2).map(str::to_string).collect();
    names.sort();
    names
}

#[test]
fn the_readme_names_the_bevy_the_manifest_depends_on() {
    // Moved from the README to `docs/guide/install.md` when the README was cut to the org's guide (`af13bb4`).
    let readme = read("docs/guide/install.md");
    let manifest = read("Cargo.toml");

    let stated: Vec<&str> = readme
        .lines()
        .filter_map(|l| {
            let rest = &l[l.find("Requires **Bevy ")? + "Requires **Bevy ".len()..];
            Some(&rest[..rest.find("**")?])
        })
        .collect();
    assert_eq!(stated.len(), 1, "docs/guide/install.md must state `Requires **Bevy X.Y**` exactly once, found {stated:?}");

    let wanted = major_minor(&bevy_requirement(&manifest));
    assert_eq!(
        stated[0], wanted,
        "docs/guide/install.md says Requires Bevy {}, Cargo.toml's [dependencies] asks for bevy {wanted}",
        stated[0]
    );
}

#[test]
fn the_install_line_names_the_version_the_manifest_declares() {
    let readme = read("README.md");
    let manifest = read("Cargo.toml");

    // The first ```toml block is the install block; take the dependency line inside it.
    let block = readme
        .split("```toml")
        .nth(1)
        .and_then(|rest| rest.split("```").next())
        .expect("README.md has no ```toml install block");
    let lines: Vec<&str> = block.lines().filter(|l| l.starts_with("bevy_carnage = \"")).collect();
    assert_eq!(lines.len(), 1, "the install block must carry one `bevy_carnage = \"A.B\"` line, found {lines:?}");

    let stated = quoted_after(lines[0], "bevy_carnage = ");
    let wanted = major_minor(&package_version(&manifest));
    assert_eq!(
        stated, wanted,
        "the README installs bevy_carnage \"{stated}\", Cargo.toml's [package] version is {} ({wanted})",
        package_version(&manifest)
    );
}

#[test]
fn the_features_the_readme_ticks_are_the_manifests_defaults() {
    // Moved with the install section to `docs/guide/install.md` (`af13bb4`).
    let readme = read("docs/guide/install.md");
    let manifest = read("Cargo.toml");

    let header = "| feature | default | what it does |";
    let table: Vec<&str> = readme
        .lines()
        .skip_while(|l| l.trim_end() != header)
        .skip(2) // the header and its `|---|` rule
        .take_while(|l| l.starts_with('|'))
        .collect();
    assert!(!table.is_empty(), "docs/guide/install.md has no `{header}` table");

    let mut ticked: Vec<String> = table
        .iter()
        .filter_map(|row| {
            let cells: Vec<&str> = row.split('|').map(str::trim).collect();
            // `| feature | default | what |` splits to ["", feature, default, what, ""].
            let (name, default) = (cells.get(1)?, cells.get(2)?);
            default.contains('✅').then(|| name.trim_matches('`').to_string())
        })
        .collect();
    ticked.sort();

    assert_eq!(
        ticked,
        default_features(&manifest),
        "docs/guide/install.md's feature table ticks {ticked:?} as default, Cargo.toml's `default` is {:?}",
        default_features(&manifest)
    );
}

#[test]
fn the_status_paragraph_names_the_version_in_the_repository() {
    // Moved to `docs/guide/history.md` (`af13bb4`); the README keeps a one-line `**Status:**`.
    let readme = read("docs/guide/history.md");
    let manifest = read("Cargo.toml");

    let status = readme
        .split("\n## Status")
        .nth(1)
        .expect("docs/guide/history.md has no `## Status` section");
    let line = status
        .lines()
        .find(|l| l.contains("` in the repository"))
        .expect("the Status section must say which version is `X.Y.Z` in the repository");
    let before = &line[..line.find("` in the repository").unwrap_or(0)];
    let stated = &before[before.rfind('`').map_or(0, |i| i + 1)..];

    assert_eq!(
        stated,
        package_version(&manifest),
        "the Status section says {stated} is in the repository, Cargo.toml's [package] version is {}",
        package_version(&manifest)
    );
}
