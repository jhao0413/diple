//! Small helpers for locating external command-line tools.

use std::env;
use std::ffi::{OsStr, OsString};
use std::path::{Path, PathBuf};
use std::process::Command;

/// Usual Unix host bin dirs a stripped pane PATH may omit. Windows inherits its PATH unchanged:
/// adding Unix spellings there would create relative drive paths rather than useful fallbacks.
#[cfg(unix)]
const COMMON_BINS: &[&str] = &["/opt/homebrew/bin", "/usr/local/bin", "/usr/bin", "/bin"];
#[cfg(windows)]
const COMMON_BINS: &[&str] = &[];

fn host_path() -> OsString {
    prepended_path(env::var_os("PATH").as_deref())
}

fn prepended_path(inherited: Option<&OsStr>) -> OsString {
    join_paths(COMMON_BINS.iter().map(PathBuf::from).chain(path_entries(inherited)), inherited)
}

fn appended_path(inherited: Option<&OsStr>) -> OsString {
    join_paths(path_entries(inherited).chain(COMMON_BINS.iter().map(PathBuf::from)), inherited)
}

fn path_entries(path: Option<&OsStr>) -> impl Iterator<Item = PathBuf> + '_ {
    path.filter(|p| !p.is_empty()).into_iter().flat_map(env::split_paths)
}

fn join_paths(entries: impl IntoIterator<Item = PathBuf>, fallback: Option<&OsStr>) -> OsString {
    // Entries produced by `split_paths` and the static fallbacks are valid path-list members.
    // Preserve the inherited spelling only as a defensive fallback for an unusual host value.
    env::join_paths(entries).unwrap_or_else(|_| fallback.unwrap_or_default().to_os_string())
}

fn executable_file(path: &Path) -> Option<PathBuf> {
    if path.is_file() {
        return Some(path.to_path_buf());
    }
    #[cfg(windows)]
    if path.extension().is_none() {
        // `Command` honors PATHEXT on Windows; mirror the standard executable suffixes here so
        // probes and editor lookup make the same choice. Include the defaults even when a
        // customized PATHEXT accidentally omits one.
        const DEFAULT_EXTENSIONS: &[&str] = &[".COM", ".EXE", ".BAT", ".CMD"];
        let configured = env::var("PATHEXT").unwrap_or_default();
        for extension in configured.split(';').chain(DEFAULT_EXTENSIONS.iter().copied()) {
            if extension.is_empty() {
                continue;
            }
            let extension = if extension.starts_with('.') {
                extension.to_owned()
            } else {
                format!(".{extension}")
            };
            let mut candidate = path.as_os_str().to_os_string();
            candidate.push(extension);
            let candidate = PathBuf::from(candidate);
            if candidate.is_file() {
                return Some(candidate);
            }
        }
    }
    None
}

fn resolve_on(path: &OsStr, name: &OsStr) -> Option<PathBuf> {
    let as_path = Path::new(name);
    if as_path.is_absolute() || as_path.parent().is_some_and(|p| !p.as_os_str().is_empty()) {
        return executable_file(as_path);
    }
    env::split_paths(path).find_map(|dir| executable_file(&dir.join(name)))
}

/// Resolve `program` on the host PATH — the common host bins first, the inherited PATH after —
/// and give the child that same PATH.
///
/// For the tools Diple runs for itself. [`user_command`] is the other way round, for the
/// reviewer's own.
pub(crate) fn command(program: impl AsRef<OsStr>) -> Command {
    let program = program.as_ref();
    let path = host_path();
    let mut cmd = resolve_on(&path, program).map_or_else(|| Command::new(program), Command::new);
    cmd.env("PATH", path);
    cmd
}

/// Resolve `program` the way the reviewer's own shell would: their `PATH` first, the common
/// host bins only as a fallback. The child is given that same PATH. `None` when the name
/// resolves to nothing, so a caller can say so before it acts.
///
/// The opposite order from [`command`], and deliberately. `git` and the forge CLIs are the
/// host's tools, so a stripped pane PATH must not hide them. The editor is the reviewer's own,
/// so a version-managed shim on their `PATH` has to win over a stale copy in a common bin, and
/// so must every tool the editor goes on to launch — its language servers, its formatters, its
/// runtime.
pub(crate) fn user_command(program: impl AsRef<OsStr>) -> Option<Command> {
    let program = program.as_ref();
    let path = appended_path(env::var_os("PATH").as_deref());
    let mut cmd = Command::new(resolve_on(&path, program)?);
    cmd.env("PATH", path);
    Some(cmd)
}

/// Whether `name` resolves to an executable on the host PATH — a dependency-free `which` that
/// also honors Windows executable suffixes. Shared by the clipboard probe (`export.rs`) and the
/// URL-opener probe (`browser.rs`).
#[must_use]
pub fn on_path(name: &str) -> bool {
    resolve_on(&host_path(), OsStr::new(name)).is_some()
}

#[cfg(test)]
mod tests {
    use super::{COMMON_BINS, appended_path, prepended_path, resolve_on};
    use std::env;
    use std::ffi::OsStr;
    use std::path::{Path, PathBuf};

    #[test]
    fn prepended_path_puts_the_common_bins_in_front_of_the_inherited_path() {
        let inherited = [PathBuf::from("inherited-one"), PathBuf::from("inherited-two")];
        let inherited_path = env::join_paths(&inherited).unwrap();
        let got = prepended_path(Some(&inherited_path));
        let parts: Vec<PathBuf> = env::split_paths(&got).collect();
        let mut expected: Vec<PathBuf> = COMMON_BINS.iter().map(PathBuf::from).collect();
        expected.extend(inherited);
        assert_eq!(parts, expected);
    }

    #[test]
    fn prepended_path_keeps_the_common_bins_when_nothing_is_inherited() {
        let got = prepended_path(None);
        let parts: Vec<PathBuf> = env::split_paths(&got).collect();
        let expected: Vec<PathBuf> = COMMON_BINS.iter().map(PathBuf::from).collect();
        assert_eq!(parts, expected);
    }

    #[test]
    fn appended_path_leaves_the_reviewers_own_entries_in_front() {
        // The editor's own tools have to resolve the way its shell would resolve them, so a
        // version-managed shim wins and the common bins only backstop a stripped PATH.
        let inherited = [PathBuf::from("reviewer-first"), PathBuf::from("reviewer-second")];
        let inherited_path = env::join_paths(&inherited).unwrap();
        let got = appended_path(Some(&inherited_path));
        let parts: Vec<PathBuf> = env::split_paths(&got).collect();
        let mut expected = inherited.to_vec();
        expected.extend(COMMON_BINS.iter().map(PathBuf::from));
        assert_eq!(parts, expected);

        let bare: Vec<PathBuf> = env::split_paths(&appended_path(None)).collect();
        assert_eq!(bare, COMMON_BINS.iter().map(PathBuf::from).collect::<Vec<_>>());

        // A set-but-empty PATH is the same as none. Joined instead, its empty entry would put
        // the reviewed repository's own working directory ahead of every real bin dir.
        assert_eq!(appended_path(Some(OsStr::new(""))), appended_path(None));
    }

    #[test]
    fn resolve_on_finds_a_bare_name_in_a_path_directory() {
        let dir = tempfile::tempdir().unwrap();
        let bin = dir.path().join(if cfg!(windows) { "gh.exe" } else { "gh" });
        std::fs::write(&bin, []).unwrap();
        let path = env::join_paths([dir.path(), Path::new("elsewhere")]).unwrap();
        assert_eq!(resolve_on(&path, OsStr::new("gh")).as_deref(), Some(bin.as_path()));
        assert!(resolve_on(&path, OsStr::new("missing")).is_none());
    }
}
