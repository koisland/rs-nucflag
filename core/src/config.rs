use serde::Deserialize;

#[derive(Deserialize, Debug, Default, Clone)]
pub struct Config {
    /// General config.
    pub general: GeneralConfig,
    /// Coverage config.
    pub cov: CoverageConfig,
    /// Mismatch base config.
    pub mismatch: Option<MismatchConfig>,
    /// Indel base config.
    pub indel: IndelConfig,
    /// Softclip base config.
    pub softclip: SoftClipConfig,
    /// Bin pileup based on self-identity. Requires fasta in input.
    pub group_by_ani: Option<GroupByANIConfig>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct GroupByANIConfig {
    /// Size of window to calculate self-identity.
    pub window_size: usize,
    /// Size of band along alignment matrix diagonal. Larger values will get longer distance relationships.
    pub band_size: usize,
    /// Threshold change in percent identity to group intervals.
    pub thr_dt_ident: f32,
    /// Minimum group size. Smaller groups are merged into larger groups of higher identity.
    pub min_grp_size: usize,
}

impl Default for GroupByANIConfig {
    fn default() -> Self {
        Self {
            window_size: 2000,
            band_size: 50,
            thr_dt_ident: 0.1,
            min_grp_size: 50_000,
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
/// Config for generated plots.
pub struct GeneralConfig {
    /// Display verbose logging.
    pub verbose: bool,
    /// Number of bases to merge misassembly intervals.
    pub bp_merge: usize,
    /// Minimum misassembly size.
    pub bp_min: usize,
    /// Whole genome window size in base pairs. Only used if no BED file is provided.
    pub bp_wg_window: usize,
    /// Merge across misassembly type.
    pub merge_across_type: bool,
}

impl Default for GeneralConfig {
    fn default() -> Self {
        Self {
            verbose: true,
            bp_merge: 5000,
            bp_min: 1,
            bp_wg_window: 10_000_000,
            merge_across_type: false,
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
/// Configuration for coverage signal.
pub struct CoverageConfig {
    /// Number of z-scores above the median to be considered a misassembly.
    pub n_zscores_high: f32,
    /// Number of z-scores below the median to be considered a misassembly.
    pub n_zscores_low: f32,
    /// Number of z-scores below the median to be considered a false-dupe.
    pub n_zscores_false_dupe: f32,
    /// Baseline coverage used for false-duplication classification. Defaults to average coverage of region.
    pub baseline: Option<u64>,
    /// Store coverage data. Toggle off to reduce memory usage.
    pub store_coverage: bool,
}

impl Default for CoverageConfig {
    fn default() -> Self {
        Self {
            n_zscores_high: 3.0,
            n_zscores_low: 3.4,
            n_zscores_false_dupe: 2.5,
            baseline: None,
            store_coverage: true,
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
/// Configuration for the mismatch signal.
pub struct MismatchConfig {
    /// Number of z-scores above the median to flag.
    pub n_zscores_high: f32,
    /// Ratio used to split hets from small collapses.
    pub ratio_het: f32,
}

impl Default for MismatchConfig {
    fn default() -> Self {
        Self {
            n_zscores_high: 3.4,
            ratio_het: 0.2,
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
/// Configuration for the base indel coverage signal.
pub struct IndelConfig {
    /// Number of z-scores above the median to flag.
    pub n_zscores_high: f32,
    /// Ratio used to call indel peaks.
    pub ratio_indel: f32,
    /// Window to apply rolling mean filter. Reduces noise.
    pub rolling_mean_window: Option<usize>,
}

impl Default for IndelConfig {
    fn default() -> Self {
        Self {
            n_zscores_high: 4.0,
            ratio_indel: 0.8,
            rolling_mean_window: Some(3),
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
/// Configuration for the base softclip coverage signal.
pub struct SoftClipConfig {
    /// Number of z-scores above the median to flag.
    pub n_zscores_high: f32,
    /// Ratio used to call softclipped peaks.
    pub ratio_softclip: f32,
}

impl Default for SoftClipConfig {
    fn default() -> Self {
        Self {
            n_zscores_high: 3.4,
            ratio_softclip: 0.5,
        }
    }
}
