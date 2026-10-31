//! **`tools/qa.sh` renders what the crate ships, and says what it could not.**
//!
//! The script is the one command a QA round is: every recorder, terminal example and autopiloted
//! windowed example, into one directory, with `warnings.txt`, `digests.txt` and `missing.txt` beside
//! them. Two things broke it without a line of it changing (issue #5). 0.6.0 took `vfx` out of the
//! default features, so a default build stopped making six of its examples and the round ended at the
//! first of them; and `capture_skinned`, added after the roster was written, was never on it, so the
//! recorder for blood on a skinned body could not be named at all.
//!
//! So these tests run the script — a dry round (`QA_DRY=1`, which prints every command it would run)
//! and a real recorders-only round against an empty binary directory — and hold what it plans and what
//! it reports to `Cargo.toml` and `examples/`, not to a copy of its roster.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

/// Examples that are deliberately on no list. `bench_carnage` is a timing run, not a rendered surface:
/// the loop runs it three times before and after a change, and it refuses a noisy reading, which is
/// no use inside a round that renders everything else at the same time.
const NOT_ON_THE_ROSTER: &[&str] = &["bench_carnage"];

fn crate_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

/// The manifest's default features, and each example's `required-features`.
struct Manifest {
    default: BTreeSet<String>,
    required: Vec<(String, BTreeSet<String>)>,
}

fn quoted(s: &str) -> BTreeSet<String> {
    s.split('"').skip(1).step_by(2).map(str::to_string).collect()
}

fn manifest() -> Manifest {
    let text = std::fs::read_to_string(crate_root().join("Cargo.toml")).expect("Cargo.toml");
    let mut section = String::new();
    let mut default = BTreeSet::new();
    let mut required = Vec::new();
    let mut name: Option<String> = None;
    for raw in text.lines() {
        let line = raw.split('#').next().unwrap_or("").trim();
        if line.starts_with('[') {
            section = line.to_string();
            name = None;
            continue;
        }
        let Some((key, value)) = line.split_once('=') else {
            continue;
        };
        let (key, value) = (key.trim(), value.trim());
        match (section.as_str(), key) {
            ("[features]", "default") => default = quoted(value),
            ("[[example]]", "name") => name = quoted(value).into_iter().next(),
            ("[[example]]", "required-features") => {
                let n = name.clone().expect("an [[example]] names itself before its features");
                required.push((n, quoted(value)));
            }
            _ => {}
        }
    }
    assert!(!default.is_empty(), "Cargo.toml must declare `default = [...]` under [features]");
    Manifest { default, required }
}

impl Manifest {
    /// What `name` needs beyond the features in `on`; empty when a build with `on` makes it.
    fn missing(&self, name: &str, on: &BTreeSet<String>) -> BTreeSet<String> {
        self.required
            .iter()
            .filter(|(n, _)| n == name)
            .flat_map(|(_, f)| f.iter())
            .filter(|f| !self.default.contains(*f) && !on.contains(*f))
            .cloned()
            .collect()
    }
}

fn examples() -> BTreeSet<String> {
    std::fs::read_dir(crate_root().join("examples"))
        .expect("examples/")
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.extension().is_some_and(|x| x == "rs"))
        .filter_map(|p| p.file_stem().map(|s| s.to_string_lossy().into_owned()))
        .collect()
}

/// A scratch directory of this test's own: an empty `bin/`, an `out/`, and an empty file to stand in
/// for the font, which the script checks exists before anything else and a dry round never opens.
struct Scratch(PathBuf);

impl Scratch {
    fn new(tag: &str) -> Self {
        let dir = std::env::temp_dir().join(format!("bevy_carnage_qa_{tag}_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(dir.join("bin")).expect("scratch bin/");
        std::fs::write(dir.join("font.ttf"), b"").expect("scratch font");
        Scratch(dir)
    }
    fn bin(&self) -> PathBuf {
        self.0.join("bin")
    }
    fn out(&self) -> PathBuf {
        self.0.join("out")
    }
}

impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

fn qa(scratch: &Scratch, env: &[(&str, &str)], names: &[&str]) -> Output {
    let mut cmd = Command::new("bash");
    cmd.arg(crate_root().join("tools/qa.sh"))
        .arg(scratch.out())
        .args(names)
        .env("FONT", scratch.0.join("font.ttf"))
        .env("QA_BIN", scratch.bin());
    for var in ["QA_DRY", "QA_PHASES", "QA_FEATURES", "QA_KEEP_MISSING", "QA_TIMEOUT"] {
        cmd.env_remove(var);
    }
    for (k, v) in env {
        cmd.env(k, v);
    }
    cmd.output().expect("bash must run tools/qa.sh")
}

fn text(bytes: &[u8]) -> String {
    String::from_utf8_lossy(bytes).into_owned()
}

/// **A default round runs every example its own build makes, and names each one it leaves out.**
/// Nothing it plans needs a feature its build line does not turn on — that is the run that ended a
/// round at `capture_carnage` — and nothing the crate ships is silently off the roster, which is how
/// `capture_skinned` and `skinned` went unrendered.
#[test]
fn a_default_round_runs_every_example_it_builds_and_names_the_rest() {
    let m = manifest();
    let scratch = Scratch::new("dry");
    let run = qa(&scratch, &[("QA_DRY", "1")], &[]);
    let stdout = text(&run.stdout);
    assert!(run.status.success(), "a dry default round must succeed:\n{}{}", stdout, text(&run.stderr));

    let build = stdout
        .lines()
        .find(|l| l.starts_with("+ cargo build"))
        .expect("a default round builds the examples");
    let words: Vec<&str> = build.split_whitespace().collect();
    let built: BTreeSet<String> = words
        .windows(2)
        .filter(|w| w[0] == "--features")
        .flat_map(|w| w[1].split(','))
        .map(str::to_string)
        .collect();

    let prefix = format!("{}/", scratch.bin().display());
    let planned: BTreeSet<String> = stdout
        .lines()
        .flat_map(str::split_whitespace)
        .filter_map(|t| t.strip_prefix(prefix.as_str()))
        .map(str::to_string)
        .collect();
    let skips: Vec<&str> = stdout.lines().filter(|l| l.starts_with("# skip ")).collect();

    let unbuildable: Vec<String> = planned
        .iter()
        .filter(|ex| !m.missing(ex, &built).is_empty())
        .map(|ex| format!("{ex} (needs {:?})", m.missing(ex, &built)))
        .collect();
    assert!(unbuildable.is_empty(), "the round runs examples its build line `{build}` does not make: {unbuildable:?}");

    for ex in examples() {
        if NOT_ON_THE_ROSTER.contains(&ex.as_str()) {
            assert!(!planned.contains(&ex), "{ex} is declared off the roster but the round runs it");
            continue;
        }
        if m.missing(&ex, &built).is_empty() {
            assert!(planned.contains(&ex), "examples/{ex}.rs builds by default but no round runs it");
        } else {
            let named = skips
                .iter()
                .any(|l| l.split_whitespace().any(|t| t.trim_end_matches(':') == ex));
            assert!(named, "examples/{ex}.rs needs {:?}; the round must say it skipped it", m.missing(&ex, &built));
        }
    }
}

/// **An example the build was not asked for is skipped with a line, and the round goes on.** Before,
/// a recorders round named `capture_carnage` died with "no built binary" before writing a summary.
#[test]
fn a_round_skips_an_example_its_build_leaves_out_and_goes_on() {
    let scratch = Scratch::new("gated");
    let run = qa(&scratch, &[("QA_PHASES", "recorders")], &["capture_carnage"]);
    assert!(
        run.status.success(),
        "the round must go on past a gated example:\n{}{}",
        text(&run.stdout),
        text(&run.stderr)
    );
    let missing = std::fs::read_to_string(scratch.out().join("missing.txt")).unwrap_or_default();
    let lines: Vec<&str> = missing.lines().collect();
    assert_eq!(lines.len(), 1, "one missing.txt line for one skipped example: {missing:?}");
    assert!(
        lines[0].contains("capture_carnage") && lines[0].contains("vfx"),
        "the line names the example and the feature that would bring it back: {:?}",
        lines[0]
    );
    assert!(Path::new(&scratch.out().join("digests.txt")).exists(), "the round reached its summaries");
}

/// **A binary the build should have made, and did not, still ends the round.** The skip above is for
/// a feature nobody asked to build; an example the default features make that has no binary is a build
/// that lost it, and `need_bin` refusing to carry on is the point of `need_bin`.
#[test]
fn a_round_still_stops_when_the_build_lost_an_example() {
    let scratch = Scratch::new("lost");
    let run = qa(&scratch, &[("QA_PHASES", "recorders")], &["capture_holes"]);
    assert!(!run.status.success(), "a missing default-feature binary must end the round");
    assert!(
        text(&run.stderr).contains("no built binary"),
        "and say why: {}",
        text(&run.stderr)
    );
}
