use crate::sound::{Sound, validate_blend};
use serde::{Deserialize, Serialize};
use std::{
    fs,
    io::{Read, Write},
    path::{Path, PathBuf},
};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Preset {
    sounds: Vec<Sound>,
}

pub fn directory() -> Result<PathBuf> {
    let dirs = directories::ProjectDirs::from("", "", "noizen")
        .ok_or("cannot determine platform configuration directory")?;
    Ok(dirs.config_dir().join("presets"))
}
fn path(dir: &Path, label: &str) -> Result<PathBuf> {
    if label.is_empty()
        || label.len() > 64
        || !label
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b == b'-' || b == b'_')
    {
        return Err(
            "preset label must be 1–64 ASCII letters, digits, hyphens or underscores".into(),
        );
    }
    Ok(dir.join(format!("{label}.toml")))
}

pub fn save(dir: &Path, label: &str, sounds: Vec<Sound>) -> Result<()> {
    let destination = path(dir, label)?;
    validate_blend(&sounds)?;
    let content = toml::to_string_pretty(&Preset { sounds })?;
    fs::create_dir_all(dir)?;
    let mut temp = tempfile::NamedTempFile::new_in(dir)?;
    temp.write_all(content.as_bytes())?;
    temp.as_file().sync_all()?;
    temp.persist_noclobber(&destination).map_err(|e| {
        format!(
            "cannot save preset '{label}' (existing presets are never overwritten): {}",
            e.error
        )
    })?;
    Ok(())
}

pub fn load(dir: &Path, label: &str) -> Result<Vec<Sound>> {
    let file = path(dir, label)?;
    if !fs::symlink_metadata(&file)?.file_type().is_file() {
        return Err(format!("preset '{label}' is not a regular file").into());
    }
    let mut text = String::new();
    fs::File::open(file)?
        .take(65_537)
        .read_to_string(&mut text)?;
    if text.len() > 65_536 {
        return Err("preset exceeds 64 KiB".into());
    }
    let preset: Preset =
        toml::from_str(&text).map_err(|e| format!("invalid preset '{label}': {e}"))?;
    validate_blend(&preset.sounds).map_err(|e| format!("invalid preset '{label}': {e}"))?;
    Ok(preset.sounds)
}

pub fn list(dir: &Path) -> Result<Vec<String>> {
    let entries = match fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(Vec::new()),
        Err(e) => return Err(e.into()),
    };
    let mut labels = Vec::new();
    for entry in entries {
        let file = entry?.path();
        if file.extension().is_some_and(|ext| ext == "toml") {
            let label = file
                .file_stem()
                .and_then(|s| s.to_str())
                .ok_or("preset filename is not valid UTF-8")?;
            load(dir, label)?;
            labels.push(label.to_owned());
        }
    }
    labels.sort();
    Ok(labels)
}

pub fn remove(dir: &Path, label: &str) -> Result<()> {
    fs::remove_file(path(dir, label)?)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn persistence_and_no_overwrite() {
        let tmp = tempfile::tempdir().unwrap();
        let dir = tmp.path().join("presets");
        assert!(list(&dir).unwrap().is_empty());
        let sounds = vec!["rain:70".parse().unwrap(), "pink:20".parse().unwrap()];
        save(&dir, "cozy", sounds.clone()).unwrap();
        let before = fs::read(dir.join("cozy.toml")).unwrap();
        assert!(
            !String::from_utf8(before.clone())
                .unwrap()
                .contains("volume")
        );
        assert!(save(&dir, "cozy", vec!["white".parse().unwrap()]).is_err());
        assert_eq!(fs::read(dir.join("cozy.toml")).unwrap(), before);
        assert_eq!(load(&dir, "cozy").unwrap(), sounds);
        assert_eq!(list(&dir).unwrap(), ["cozy"]);
        remove(&dir, "cozy").unwrap();
        assert!(load(&dir, "cozy").is_err());
        assert!(remove(&dir, "cozy").is_err());
    }
    #[test]
    fn invalid_content_and_io_failures() {
        let tmp = tempfile::tempdir().unwrap();
        for label in ["../escape", "/tmp/noizen", "a/b", "..", "", "a.b"] {
            assert!(save(tmp.path(), label, vec!["white".parse().unwrap()]).is_err());
            assert!(load(tmp.path(), label).is_err());
            assert!(remove(tmp.path(), label).is_err());
        }
        for content in [
            "garbage",
            "sounds = []",
            "sounds = [{id='grey',level=10}]",
            "sounds = [{id='pink',level=101}]",
            "volume=50\nsounds=[{id='pink',level=20}]",
            "sounds=[{id='pink',level=20,extra=1}]",
            "sounds=[{id='pink',level=20},{id='pink',level=30}]",
        ] {
            fs::write(tmp.path().join("bad.toml"), content).unwrap();
            assert!(load(tmp.path(), "bad").is_err(), "{content}");
            assert!(list(tmp.path()).is_err());
        }
        let blocked = tmp.path().join("file");
        fs::write(&blocked, "preserve").unwrap();
        assert!(save(&blocked, "cozy", vec!["white".parse().unwrap()]).is_err());
        assert_eq!(fs::read_to_string(blocked).unwrap(), "preserve");
        assert!(list(&tmp.path().join("file")).is_err());
    }
}
