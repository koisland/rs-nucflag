use std::{convert::Infallible, str::FromStr};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MisassemblyType {
    LowQuality,
    Indel,
    SoftClip,
    Collapse,
    Misjoin,
    FalseDupe,
    Null,
}

impl MisassemblyType {
    pub fn item_rgb(&self) -> &'static str {
        match self {
            // Purple
            MisassemblyType::Indel => "128,0,128",
            // Teal
            MisassemblyType::SoftClip => "0,255,255",
            // Pink
            MisassemblyType::LowQuality => "255,0,128",
            // Green
            MisassemblyType::Collapse => "0,255,0",
            // Orange
            MisassemblyType::Misjoin => "255,165,0",
            // Blue
            MisassemblyType::FalseDupe => "0,0,255",
            MisassemblyType::Null => "0,0,0",
        }
    }
}

impl FromStr for MisassemblyType {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "low_quality" => MisassemblyType::LowQuality,
            "indel" => MisassemblyType::Indel,
            "softclip" => MisassemblyType::SoftClip,
            "misjoin" => MisassemblyType::Misjoin,
            "collapse" => MisassemblyType::Collapse,
            "false_dupe" => MisassemblyType::FalseDupe,
            _ => MisassemblyType::Null,
        })
    }
}
