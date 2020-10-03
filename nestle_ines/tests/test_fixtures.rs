use anyhow::Result;
use binread::BinReaderExt;
use nestle_ines::model::Ines;
use std::fs::File;
use std::path::{Path, PathBuf};

fn execute_bin(path: &Path) -> Result<()> {
    dbg!(path);
    let mut reader = File::open(path)?;
    let _ines: Ines = reader.read_le().unwrap();
    Ok(())
}

#[test]
fn test_main() -> Result<()> {
    let dir = env!("CARGO_MANIFEST_DIR");
    let fixtures = PathBuf::from(dir).join("fixtures");

    for bin in fixtures.read_dir().unwrap() {
        if let Ok(bin) = bin {
            execute_bin(&bin.path())?;
        }
    }

    Ok(())
}
