//! Location to population lookup.
use byteorder::{LittleEndian as LE, ReadBytesExt};
use h3o::CellIndex;
use hextree::{Cell, HexTreeMap};
use indicatif::{ProgressBar, ProgressState, ProgressStyle};
use std::{
    fs::File,
    io::{BufReader, Error as IoError, ErrorKind as IoErrorKind},
    path::PathBuf,
};
use thiserror::Error;

#[derive(Debug)]
pub struct PopMap {
    popmap: HexTreeMap<f32>,
}

impl PopMap {
    pub fn load(src_file: File) -> Result<Self, PopError> {
        let file_size = src_file.metadata()?.len();
        let mut popmap = HexTreeMap::new();
        let idx_val_pairs_total =
            file_size / (std::mem::size_of::<u64>() + std::mem::size_of::<f32>()) as u64;

        let pb = ProgressBar::new(idx_val_pairs_total);
        pb.set_prefix("Loading population data");
        pb.set_style(
            ProgressStyle::with_template("{prefix}:\n[{wide_bar:.cyan/blue}]")
                .expect("incorrect progress bar format string")
                .with_key(
                    "eta",
                    |state: &ProgressState, w: &mut dyn std::fmt::Write| {
                        write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap();
                    },
                )
                .progress_chars("#>-"),
        );

        let mut rdr = BufReader::new(src_file);
        loop {
            match (rdr.read_u64::<LE>(), rdr.read_f32::<LE>()) {
                (Ok(h3_index), Ok(val)) => {
                    let cell = Cell::try_from(h3_index)
                        .expect("serialized hexmap should only contain valid indices");
                    popmap.insert(cell, val);
                    pb.inc(1)
                }
                (Err(e), _) if e.kind() == IoErrorKind::UnexpectedEof => {
                    break;
                }
                (err @ Err(_), _) => {
                    err?;
                }
                (_, err @ Err(_)) => {
                    err?;
                }
            };
        }

        Ok(Self { popmap })
    }

    pub fn get(&self, target_cell: CellIndex) -> Option<f32> {
        let target_cell =
            Cell::try_from(u64::from(target_cell)).expect("A CellIndex is always a valid Cell");
        match self.popmap.get(target_cell) {
            Some((cell, _)) if target_cell.res() > cell.res() => None,
            _ => self
                .popmap
                .subtree_iter(target_cell)
                .map(|kv| *kv.1)
                .filter(|&pop| pop > 0.0)
                .reduce(|acc, pop| acc + pop),
        }
    }
}

#[derive(Debug, Error)]
pub enum PopError {
    #[error("{0}")]
    Io(#[from] IoError),
    #[error("unknown region {0:?}")]
    Region(String),
    #[error("not a file {0:?}")]
    NotFile(PathBuf),
    #[error("error parsing {0:?}")]
    Parse(PathBuf),
}
