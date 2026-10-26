#!/usr/bin/env python3
"""Validate release archives; generate checksums and a macOS Homebrew formula."""
import hashlib
from pathlib import Path
import re
import sys
import tarfile

TARGETS = (
    "aarch64-apple-darwin", "x86_64-apple-darwin",
    "x86_64-unknown-linux-gnu", "aarch64-unknown-linux-gnu",
)


def generate(tag, directory):
    if not re.fullmatch(r"v[0-9]+\.[0-9]+\.[0-9]+", tag):
        raise ValueError("release tag must be vMAJOR.MINOR.PATCH")
    directory = Path(directory)
    checksums = {}
    for target in TARGETS:
        stem = f"noizen-{tag}-{target}"
        archive = directory / f"{stem}.tar.gz"
        with tarfile.open(archive, "r:gz") as bundle:
            for name in ("noizen", "assets/README.md", "assets/CC-BY-4.0.txt",
                         "completions/noizen.bash", "completions/noizen.zsh",
                         "completions/noizen.fish", "README.md", "AGENTS.md",
                         "docs/completions.md"):
                member = bundle.getmember(f"{stem}/{name}")
                if not member.isfile() or member.size == 0:
                    raise ValueError(f"{archive.name}: invalid or empty {name}")
                if name == "noizen" and not member.mode & 0o111:
                    raise ValueError(f"{archive.name}: binary is not executable")
        checksums[target] = hashlib.sha256(archive.read_bytes()).hexdigest()

    formula = '''class Noizen < Formula
  desc "Offline ambient sound mixer"
  homepage "https://github.com/nendix/noizen"
  version "%s"
  depends_on :macos

  on_macos do
''' % tag[1:]
    for arch, target in (("arm", TARGETS[0]), ("intel", TARGETS[1])):
        formula += f'''    on_{arch} do
      url "https://github.com/nendix/noizen/releases/download/{tag}/noizen-{tag}-{target}.tar.gz"
      sha256 "{checksums[target]}"
    end
'''
    formula += '''  end

  def install
    bin.install "noizen"
    bash_completion.install "completions/noizen.bash" => "noizen"
    zsh_completion.install "completions/noizen.zsh" => "_noizen"
    fish_completion.install "completions/noizen.fish"
    pkgshare.install "assets", "docs", "README.md", "AGENTS.md"
  end

  test do
    assert_match "white", shell_output("#{bin}/noizen sounds")
  end
end
'''
    (directory / "SHA256SUMS").write_text("".join(
        f"{checksums[target]}  noizen-{tag}-{target}.tar.gz\n" for target in TARGETS
    ))
    (directory / "noizen.rb").write_text(formula)


if __name__ == "__main__":
    if len(sys.argv) != 3:
        sys.exit("usage: release.py vMAJOR.MINOR.PATCH ARCHIVE_DIRECTORY")
    generate(sys.argv[1], sys.argv[2])
