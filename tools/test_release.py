import hashlib
import io
from pathlib import Path
import tarfile
import tempfile
import unittest

from release import TARGETS, generate


class ReleaseTests(unittest.TestCase):
    def make_archives(self, directory, omit=None, executable=True):
        for target in TARGETS:
            stem = f"noizen-v0.1.0-{target}"
            with tarfile.open(directory / f"{stem}.tar.gz", "w:gz") as archive:
                for name in ("noizen", "assets/README.md", "assets/CC-BY-4.0.txt",
                             "completions/noizen.bash", "completions/noizen.zsh",
                             "completions/noizen.fish", "README.md", "AGENTS.md",
                             "docs/completions.md"):
                    if name == omit:
                        continue
                    content = b"test fixture\n"
                    member = tarfile.TarInfo(f"{stem}/{name}")
                    member.size = len(content)
                    member.mode = 0o755 if name == "noizen" and executable else 0o644
                    archive.addfile(member, io.BytesIO(content))

    def test_checksums_and_formula_use_actual_archives(self):
        with tempfile.TemporaryDirectory() as tmp:
            directory = Path(tmp)
            self.make_archives(directory)
            generate("v0.1.0", directory)
            lines = (directory / "SHA256SUMS").read_text().splitlines()
            self.assertEqual(len(lines), 4)
            for line in lines:
                digest, name = line.split()
                self.assertEqual(digest, hashlib.sha256((directory / name).read_bytes()).hexdigest())
            formula = (directory / "noizen.rb").read_text()
            self.assertIn('version "0.1.0"', formula)
            self.assertIn("depends_on :macos", formula)
            self.assertNotIn("unknown-linux", formula)
            self.assertIn('pkgshare.install "assets"', formula)
            for line in lines[:2]:
                digest, name = line.split()
                self.assertIn(f"/v0.1.0/{name}", formula)
                self.assertIn(f'sha256 "{digest}"', formula)

    def test_invalid_tags_do_not_write_outputs(self):
        with tempfile.TemporaryDirectory() as tmp:
            for tag in ("main", "0.1.0", "v1.2", "v1.2.3/escape", "v1.2.3\n"):
                with self.assertRaises(ValueError):
                    generate(tag, tmp)
            self.assertEqual(list(Path(tmp).iterdir()), [])

    def test_missing_archive_fails(self):
        with tempfile.TemporaryDirectory() as tmp:
            with self.assertRaises(FileNotFoundError):
                generate("v0.1.0", tmp)
            self.assertFalse((Path(tmp) / "noizen.rb").exists())

    def test_missing_license_or_nonexecutable_binary_fails(self):
        for omit, executable in (("assets/CC-BY-4.0.txt", True), (None, False)):
            with self.subTest(omit=omit, executable=executable), tempfile.TemporaryDirectory() as tmp:
                directory = Path(tmp)
                self.make_archives(directory, omit, executable)
                with self.assertRaises((KeyError, ValueError)):
                    generate("v0.1.0", directory)
                self.assertFalse((directory / "SHA256SUMS").exists())
                self.assertFalse((directory / "noizen.rb").exists())


if __name__ == "__main__":
    unittest.main()
