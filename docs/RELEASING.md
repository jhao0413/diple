# Releasing Diple

A pushed `v*` tag creates a draft GitHub release, builds signed binaries for macOS, Linux, and Windows,
attaches checksums and provenance, then publishes the release after every target succeeds.

## Release checklist

1. Add a matching version section to `CHANGELOG.md`.
2. Bump `package.version` in `Cargo.toml`.
3. Run `cargo check` so `Cargo.lock` records the version.
4. Run `just ci`.
5. Commit the release changes, then tag and push:

   ```sh
   git tag -a vX.Y.Z -m "Diple vX.Y.Z"
   git push origin main vX.Y.Z
   ```

The release workflow publishes `diple-<target>.tar.gz` on Unix and
`diple-x86_64-pc-windows-msvc.zip` on Windows, each with a SHA-256 sidecar.

The public `jhao0413/homebrew-tap` repository reads those release assets and updates
`Formula/diple.rb` from its own `sync.yml` workflow. After a release, trigger it immediately:

```sh
gh workflow run sync.yml --repo jhao0413/homebrew-tap
```

Its scheduled run is the fallback. Verify the published formula with:

```sh
brew update
brew install jhao0413/tap/diple
diple --version
```

For a new Diple version, update the matching `Jhao0413.Diple` manifest in
`microsoft/winget-pkgs` with the Windows ZIP URL and SHA-256 checksum. The initial submission and
every update happen only after the immutable release is public.
