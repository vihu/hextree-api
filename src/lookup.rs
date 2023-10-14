//! Location to LoraWAN region lookip
use byteorder::{LittleEndian as LE, ReadBytesExt};
use flate2::read::GzDecoder;
use helium_proto::Region;
use hextree::{Cell, HexTreeMap};
use std::{
    fs::{self, File},
    io::Error as IoError,
    path::{Path, PathBuf},
};
use thiserror::Error;

#[derive(Debug)]
pub struct RegionMap {
    regions: HexTreeMap<Region>,
}

impl RegionMap {
    /// Load regional maps `region_dir`.
    pub fn load<P: AsRef<Path>>(region_dir: P) -> Result<Self, RmError> {
        let mut map_tree = HexTreeMap::new();

        for dir_entry in fs::read_dir(region_dir.as_ref())? {
            let dir_entry = dir_entry?;
            let dir_entry_path = dir_entry.path();
            if dir_entry_path.is_file() {
                if let Some("h3idz") = dir_entry_path.extension().and_then(|ext| ext.to_str()) {
                    // Gzipped serialized H3 indicies
                    let region = parse_region_name(&dir_entry_path)?;
                    let cells = parse_region_cells(&dir_entry_path)?;
                    map_tree.extend(cells.into_iter().zip(std::iter::repeat(region)));
                }
            }
        }

        Ok(Self { regions: map_tree })
    }

    pub fn get(&self, cell: Cell) -> Option<&Region> {
        self.regions.get(cell).map(|(_cell, region)| region)
    }
}

fn parse_region_cells(path: &Path) -> Result<Vec<Cell>, RmError> {
    let f = File::open(path)?;
    let mut decoder = GzDecoder::new(f);
    let mut cells = Vec::new();
    while let Ok(idx) = decoder.read_u64::<LE>() {
        let cell = Cell::try_from(idx).map_err(|_| RmError::Parse(path.to_owned()))?;
        cells.push(cell)
    }
    Ok(cells)
}

fn parse_region_name(path: &Path) -> Result<Region, RmError> {
    let prefix = file_prefix(path)?;
    parse_region_str(&prefix)
}

// Extract filename until the first '.'
fn file_prefix(path: &Path) -> Result<String, RmError> {
    Ok(path
        .file_name()
        .ok_or_else(|| RmError::NotFile(path.to_owned()))?
        .to_str()
        .ok_or_else(|| RmError::NotFile(path.to_owned()))?
        .chars()
        .take_while(|&c| c != '.')
        .collect::<String>())
}

fn parse_region_str(prefix: &str) -> Result<Region, RmError> {
    let region = match prefix {
        "AS923-1" => Region::As9231,
        "AS923-1A" => Region::As9231a,
        "AS923-1B" => Region::As9231b,
        "AS923-1C" => Region::As9231c,
        "AS923-1D" => Region::As9231d,
        "AS923-1E" => Region::As9231e,
        "AS923-1F" => Region::As9231f,
        "AS923-2" => Region::As9232,
        "AS923-3" => Region::As9233,
        "AS923-4" => Region::As9234,
        "AU915" => Region::Au915,
        "AU915-SB1" => Region::Au915Sb1,
        "AU915-SB2" => Region::Au915Sb2,
        "CD900-1A" => Region::Cd9001a,
        "CN470" => Region::Cn470,
        "EU433" => Region::Eu433,
        "EU868" => Region::Eu868,
        "EU868-A" => Region::Eu868A,
        "EU868-B" => Region::Eu868B,
        "EU868-C" => Region::Eu868C,
        "EU868-D" => Region::Eu868D,
        "EU868-E" => Region::Eu868E,
        "EU868-F" => Region::Eu868F,
        "IN865" => Region::In865,
        "KR920" => Region::Kr920,
        "RU864" => Region::Ru864,
        "US915" => Region::Us915,
        other => return Err(RmError::Region(other.to_owned())),
    };
    Ok(region)
}

#[derive(Debug, Error)]
pub enum RmError {
    #[error("{0}")]
    Io(#[from] IoError),
    #[error("unknown region {0:?}")]
    Region(String),
    #[error("not a file {0:?}")]
    NotFile(PathBuf),
    #[error("error parsing {0:?}")]
    Parse(PathBuf),
}
