use crate::domain::{InstanceType as DomainInstanceType, Region as DomainRegion};
use koral::prelude::*;

// --- Custom Flag Types ---

#[derive(FlagValue, Clone, Debug, PartialEq)]
pub enum OutputFormat {
    Json,
    Text,
    Table,
}

#[derive(FlagValue, Clone, Debug, PartialEq)] // Using FlagValue for domain enums wrapper or implementing manually.
                                              // koral's derive(FlagValue) works on enums!
                                              // But we need to map our domain enums.
                                              // For simplicity, we redefine identical enums here or wrap them?
                                              // Since domain types are simple, we can just derive FlagValue on them if we move FlagValue import to domain,
                                              // OR we just define types here and map them.
                                              // Let's redefine mirrors for CLI usage to decouple domain/cli.
pub enum Region {
    UsEast1,
    UsWest1,
    EuCentral1,
    ApNortheast1,
}

impl From<Region> for DomainRegion {
    fn from(val: Region) -> Self {
        match val {
            Region::UsEast1 => DomainRegion::UsEast1,
            Region::UsWest1 => DomainRegion::UsWest1,
            Region::EuCentral1 => DomainRegion::EuCentral1,
            Region::ApNortheast1 => DomainRegion::ApNortheast1,
        }
    }
}

#[derive(FlagValue, Clone, Debug, PartialEq)]
pub enum InstanceType {
    T2Micro,
    T3Medium,
    M5Large,
}

impl From<InstanceType> for DomainInstanceType {
    fn from(val: InstanceType) -> Self {
        match val {
            InstanceType::T2Micro => DomainInstanceType::T2Micro,
            InstanceType::T3Medium => DomainInstanceType::T3Medium,
            InstanceType::M5Large => DomainInstanceType::M5Large,
        }
    }
}

// --- Flags ---

#[derive(Flag, Debug, Clone)]
#[flag(name = "verbose", short = 'v', help = "詳細なログ出力を有効にします")]
pub struct VerboseFlag(#[allow(dead_code)] pub bool);

#[derive(Flag, Debug, Clone)]
#[flag(name = "profile", help = "使用するプロファイル名")]
pub struct ProfileFlag(#[allow(dead_code)] pub String);

#[derive(Flag, Debug, Clone)]
#[flag(
    name = "region",
    short = 'r',
    help = "リージョンを指定します",
    default = "us-east-1"
)]
pub struct RegionFlag(#[allow(dead_code)] pub Region);

#[derive(Flag, Debug, Clone)]
#[flag(
    name = "format",
    help = "出力形式 (json, text, table)",
    default = "text"
)]
pub struct FormatFlag(#[allow(dead_code)] pub OutputFormat);

#[derive(Flag, Debug, Clone)]
#[flag(name = "type", help = "インスタンスタイプ", default = "t2.micro")]
pub struct InstanceTypeFlag(#[allow(dead_code)] pub InstanceType);

#[derive(Flag, Debug, Clone)]
#[flag(name = "user", required = true, help = "ユーザー名")]
pub struct UserFlag(#[allow(dead_code)] pub String);

#[derive(Flag, Debug, Clone)]
#[flag(name = "token", env = "CLOUD_CLI_TOKEN", help = "認証トークン")]
pub struct TokenFlag(#[allow(dead_code)] pub String);
