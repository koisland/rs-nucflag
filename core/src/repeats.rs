use core::str;
use std::{fmt::Display, str::FromStr};

use eyre::bail;
use itertools::Itertools;
use suffix::SuffixTable;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Repeat {
    /// Scaffold sequence with `N` characters.
    Scaffold,
    /// Homopolymers with continuous run of some characters. ex. `AAAAA`
    Homopolymer,
    /// Simple repeat of some size. ex. `AGCAGCAGC`
    Simple,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RepeatSummary<'a> {
    /// Repeat type.
    pub repeat: Repeat,
    /// Repeat sequence.
    pub sequence: &'a str,
    /// Proportion of original string.
    pub prop: f32,
    /// Original sequence.
    pub original_sequence: &'a str,
}

/// Detect largest repeated sequence within `seq`.
/// * Uses the [`suffix`] crate to create a longest common prefix array to determine the largest repeat.
///
/// # Args
/// * `seq`
///     * String to search.
///     * Must be less than 2^32 - 1 bytes. See [`suffix::SuffixTable`].
///
/// # Returns
/// * Repeat type and the proportion within the `seq` as a [`RepeatSummary`].
///
/// # Example
/// ```
/// let seq = "TTAGCAGCAGCCCG"
/// let summary = detect_largest_repeat(seq)
///
/// ```
pub fn detect_largest_repeat(seq: &str) -> Option<RepeatSummary> {
    if seq.is_empty() {
        return None;
    }
    // Construct the suffix table and longest common prefix array.
    let sfx_tbl = SuffixTable::new(seq);
    let lcp_arr = sfx_tbl.lcp_lens();

    // Get largest repeated sequence.
    let (idx_largest_sfx, largest_sfx_length) = lcp_arr
        .into_iter()
        .enumerate()
        .max_by(|a, b| a.1.cmp(&b.1))?;

    // Get total proportion of string based on suffix table positions.
    let largest_repeat = &sfx_tbl.suffix(idx_largest_sfx)[0..largest_sfx_length as usize];
    let positions = sfx_tbl.positions(largest_repeat);
    let mut positions_iter = positions.iter().sorted().peekable();
    let mut total_length = 0;
    loop {
        let Some(pos) = positions_iter.next() else {
            break;
        };
        let Some(next_pos) = positions_iter.peek() else {
            total_length += largest_sfx_length;
            break;
        };
        // Some overlap if diff between two adjacent positions less than the largest sfx length.
        let diff = *next_pos - pos;
        if diff < largest_sfx_length {
            total_length += diff
        } else {
            total_length += largest_sfx_length
        }
    }

    let prop = total_length as f32 / seq.len() as f32;
    let repeat = positions.first().and_then(|p| {
        let idx = *p as usize;
        seq.get(idx..idx + largest_sfx_length as usize)
    })?;
    // Check total unique characters.
    let repeat_type = match repeat.chars().sorted().dedup().count() {
        0 => unreachable!(),
        1 => Repeat::Homopolymer,
        2.. => Repeat::Simple,
    };

    Some(RepeatSummary {
        repeat: if repeat.contains('N') {
            Repeat::Scaffold
        } else {
            repeat_type
        },
        sequence: repeat,
        original_sequence: seq,
        prop,
    })
}

impl FromStr for Repeat {
    type Err = eyre::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "scaffold" => Repeat::Scaffold,
            "homopolymer" => Repeat::Homopolymer,
            "simple" => Repeat::Simple,
            _ => bail!("Invalid repeat type, {s}."),
        })
    }
}

impl Display for Repeat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Repeat::Scaffold => write!(f, "scaffold"),
            Repeat::Homopolymer => write!(f, "homopolymer"),
            Repeat::Simple => write!(f, "simple"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::{detect_largest_repeat, Repeat, RepeatSummary};

    #[test]
    fn test_detect_simple_triple() {
        let seq = "TTAGCAGCAGCCCG";
        let res = detect_largest_repeat(seq);
        assert_eq!(
            Some(RepeatSummary {
                repeat: Repeat::Simple,
                sequence: "AGCAGC",
                prop: 0.64285713,
                original_sequence: "TTAGCAGCAGCCCG",
            },),
            res
        )
    }

    #[test]
    fn test_detect_simple_double() {
        let seq = "ATATATATATC";
        let res = detect_largest_repeat(seq);
        dbg!(
            Some(RepeatSummary {
                repeat: Repeat::Simple,
                sequence: "ATATATAT",
                prop: 0.90909094,
                original_sequence: "ATATATATATC",
            },),
            res
        );
    }

    #[test]
    fn test_detect_homopolymer() {
        let seq = "AAAAAAAAAACCCC";
        let res = detect_largest_repeat(seq);
        assert_eq!(
            Some(RepeatSummary {
                repeat: Repeat::Homopolymer,
                sequence: "AAAAAAAAA",
                prop: 0.71428573,
                original_sequence: "AAAAAAAAAACCCC",
            },),
            res
        )
    }

    #[test]
    fn test_detect_scaffold() {
        let seq = "GNNNNNNA";
        let res = detect_largest_repeat(seq);
        assert_eq!(
            Some(RepeatSummary {
                repeat: Repeat::Scaffold,
                sequence: "NNNNN",
                prop: 0.75,
                original_sequence: "GNNNNNNA",
            },),
            res
        );
    }
}
