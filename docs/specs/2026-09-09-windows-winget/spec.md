# Native Windows and WinGet

## Goal

Diple is usable from a native Windows terminal without installing Rust. A release publishes a
portable Windows binary and the Microsoft WinGet community source can install, upgrade, and
remove it as the `diple` command.

## User contract

- `winget install Jhao0413.Diple` installs Diple without Cargo, an installer wizard, or an
  administrator-only machine mutation.
- Typing `diple` after installation starts the same TUI and stdout review flow as on Unix.
- The release asset is `diple-x86_64-pc-windows-msvc.zip`; it contains `diple.exe` and has a
  SHA-256 sidecar and GitHub build-provenance attestation.
- `y` copies the review through the Windows system clipboard.
- Links in the PR view open through the Windows default URL handler.
- Captured stdout remains a clean result channel; terminal control bytes go to the Windows
  console.
- Windows support is compiled and tested on every pull request and main-branch push.

## Distribution

The Windows asset is a portable program, not an EXE installer. WinGet owns placement, PATH
aliasing, upgrades, and uninstall. The community manifest uses `InstallerType: zip`,
`NestedInstallerType: portable`, and exposes `diple.exe` as the `diple` command.

The first WinGet manifest is submitted only after the matching immutable GitHub release exists,
because its URL and SHA-256 checksum are part of the manifest.

## Platform behavior

- Executable lookup respects the host path separator and Windows `PATHEXT` extensions.
- Windows does not receive Unix fallback directories such as `/usr/bin`.
- Clipboard export uses the native Unicode clipboard API on Windows; macOS and Linux retain
  their existing tools.
- URL opening uses the Windows URL handler through `rundll32.exe`; macOS and Linux retain their
  existing openers.

## Non-goals

- MSI, MSIX, or a graphical setup wizard.
- Windows ARM64 in the first WinGet release.
- Code-signing infrastructure. WinGet still validates the immutable archive checksum, while
  Authenticode signing can be added independently later.
