# Releases and Homebrew

Release automation is prepared, **not published or tested on GitHub yet**.
The existing `nendix/homebrew-tap` has no noizen formula yet. Nothing is pushed
or published by local development commands.

## Manual release process

1. Update `Cargo.toml`/`Cargo.lock` to the intended version and push the reviewed
   code, including `.github/workflows/release.yml`, to the default branch.
2. Create and push the matching tag, then explicitly start the workflow:

   ```sh
   git tag v0.1.0
   git push origin v0.1.0
   gh workflow run release.yml --repo nendix/noizen --ref main -f tag=v0.1.0
   ```

   Alternatively use **Actions → Release → Run workflow**, entering the tag.
   Pushing code or tags alone does **not** trigger a release. The workflow checks
   out the existing tag and requires it to match the Cargo version.

3. Inspect the workflow results and **draft** release. It contains four archives,
   `SHA256SUMS`, and a generated macOS Homebrew formula (`noizen.rb`). Each archive
   includes the binary, static completions, documentation and recording notices.
   All four builds run formatting, clippy, Rust and packaging tests, plus a
   `--version` smoke check. None proves audible playback.
4. Download the draft assets using `gh release download`, check the archive
   contents/checksums and test the real binaries on their intended platforms.
   Keep the release an MVP candidate while listening acceptance is outstanding.
5. Publish explicitly when ready:

   ```sh
   gh release edit v0.1.0 --repo nendix/noizen --draft=false
   ```

6. Copy the generated `noizen.rb` into `Formula/noizen.rb` in a checkout of
   `nendix/homebrew-tap`, review it, commit and push that tap separately. The
   workflow deliberately has no cross-repository token or automatic tap push.
   Once updated, verify `brew install nendix/tap/noizen` and
   `brew test nendix/tap/noizen` on both supported Mac architectures.

If a build fails, no draft is created. Re-run failed jobs after diagnosing it;
do not move an already published version tag or replace its checksummed assets.
Creating a release for an already existing release tag fails rather than
silently replacing it.

## User installation, after publication

macOS:

```sh
brew install nendix/tap/noizen
```

The formula installs the matching prebuilt binary, completions and recording
notices. It does not build Rust or initialize audio. Homebrew support here is
macOS-only; Linux users use the release archive.

For an archive install, choose the matching target from GitHub Releases:

| Target | Platform / CPU | Build runner |
| --- | --- | --- |
| aarch64-apple-darwin | macOS / Apple Silicon | macOS 15 ARM |
| x86_64-apple-darwin | macOS / Intel | macOS 15 Intel |
| aarch64-unknown-linux-gnu | Linux / ARM64 | Ubuntu 24.04 ARM |
| x86_64-unknown-linux-gnu | Linux / Intel or AMD 64-bit | Ubuntu 24.04 x86_64 |

Verify with `shasum -a 256 -c SHA256SUMS` (macOS) or `sha256sum -c SHA256SUMS`
(Linux), with all four archives present, then extract the appropriate archive
and copy `noizen` to a directory on PATH. Keep its `assets/` notices when
redistributing. Archive completions can be installed using
[the shell setup guide](completions.md).

Linux GNU builds require glibc compatible with the Ubuntu 24.04 build environment
(glibc 2.39), `libasound.so.2`, and configured audio output. They are not static
binaries and are not suitable for Alpine/musl. macOS builds use system audio
frameworks; older macOS compatibility and Gatekeeper behavior are not verified.
No signing/notarization is configured. Rust and codec/player commands are not
needed by the installed application.
