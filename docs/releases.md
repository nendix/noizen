# Releases with cargo-dist

Configured with **cargo-dist 0.33.1**. No release or Homebrew formula has been
published yet; the GitHub workflow and real Homebrew installation remain unverified.

## Maintain the workflow

```sh
brew install cargo-dist
dist generate
dist generate --check
```

Edit `dist-workspace.toml` and `.github/dist-build-setup.yml`, then regenerate.
Do not hand-edit `.github/workflows/release.yml`. Keep the local dist version in
sync with `cargo-dist-version`. The setup hook installs ALSA before compiling,
runs quality checks, and prepares recording notices and static completions.

Local archive check (the CI setup hook is not run by local `dist build`):

```sh
mkdir -p target/release-extras/assets target/release-extras/completions
cp assets/README.md assets/CC-BY-4.0.txt target/release-extras/assets/
for shell in bash zsh fish; do
  cargo run --locked --profile dist -- completions "$shell" > "target/release-extras/completions/noizen.$shell"
done
dist build --artifacts=all --target aarch64-apple-darwin
```

Choose your native target. Outputs go to `target/distrib/`; this command does not
publish anything. Source archives reflect committed files, so commit before
checking their contents.

## Manually release

1. Set the Cargo version, run checks and push the reviewed code to the default
   branch. The workflow must be present there before GitHub can dispatch it.
2. Add repository secret `HOMEBREW_TAP_TOKEN`: a token scoped to
   `nendix/homebrew-tap` with Contents read/write access. It enables the generated
   publisher to commit the formula to the existing tap. Do not commit the token.
3. Run **Actions → Release → Run workflow** with tag **`dry-run`** first:

   ```sh
   gh workflow run release.yml --repo nendix/noizen --ref main -f tag=dry-run
   ```

   This builds downloadable workflow artifacts without publishing a release or
   updating the tap. Inspect checksums/notices and test the actual binaries.
4. When ready, manually run again with the matching version, e.g. `v0.1.0`.
   **This publishes a GitHub Release and updates the Homebrew tap**, not merely a
   draft. Unlike the previous custom workflow, cargo-dist can create the tag for
   the selected workflow ref; a pre-existing tag is not required. If using an
   existing tag, select that same ref, not a different commit.

Pushing code or tags alone never publishes. Pull requests run release-plan
checks only. The default dispatch tag is `dry-run`. Prerelease versions do not
update Homebrew by default. Keep MVP acceptance limitations visible in release
notes; packaging does not prove audible playback.

## Artifacts and installation

Four native GNU/macOS archives, per-archive SHA-256 files, `sha256.sum`, a source
archive and a Homebrew formula are generated. Binary archives contain
`assets/README.md`, the CC BY legal text, shell completion scripts, and
documentation. Homebrew retains extras under `$(brew --prefix)/share/noizen/`;
install completions using [the shell setup guide](completions.md).

After the first successful published release and tap update, macOS users can run:

```sh
brew install nendix/tap/noizen
```

| Target | Native build runner |
| --- | --- |
| aarch64-apple-darwin | macOS 15 ARM |
| x86_64-apple-darwin | macOS 15 Intel |
| aarch64-unknown-linux-gnu | Ubuntu 24.04 ARM |
| x86_64-unknown-linux-gnu | Ubuntu 24.04 x86_64 |

Linux users should use the matching archive and verify its checksum. GNU builds
require compatible glibc (Ubuntu 24.04 uses 2.39), `libasound.so.2` and configured
OS audio output; Alpine/musl is unsupported. Homebrew's generated formula also
contains Linux downloads, but Linuxbrew dependency resolution is unverified and
is not the supported Linux installation path yet. macOS uses system frameworks.
Older macOS compatibility and Gatekeeper behavior remain unverified; no
Developer ID signing/notarization is configured. Retain recording notices when
redistributing. No Rust toolchain, player or codec command is needed at runtime.

## Migration status

The custom Python packager/tests were removed; cargo-dist owns archive,
checksum, formula and workflow generation. There were no published releases to
migrate. Reverting this migration commit restores the previous pipeline; never
replace assets or move tags for an already published version.
