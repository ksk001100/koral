use crate::domain::{InstanceType as DomainInstanceType, Region as DomainRegion};
use koral::prelude::*;

// --- Custom Flag Types ---

#[derive(FlagValue, Clone, Debug, PartialEq)]
pub enum OutputFormat {
    Json,
    Text,
    Table,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Region {
    UsEast1,
    UsWest1,
    EuCentral1,
    ApNortheast1,
}

impl std::str::FromStr for Region {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "us-east-1" => Ok(Region::UsEast1),
            "us-west-1" => Ok(Region::UsWest1),
            "eu-central-1" => Ok(Region::EuCentral1),
            "ap-northeast-1" => Ok(Region::ApNortheast1),
            _ => Err(format!("Invalid region: {}", s)),
        }
    }
}

impl std::fmt::Display for Region {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Region::UsEast1 => write!(f, "us-east-1"),
            Region::UsWest1 => write!(f, "us-west-1"),
            Region::EuCentral1 => write!(f, "eu-central-1"),
            Region::ApNortheast1 => write!(f, "ap-northeast-1"),
        }
    }
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

#[derive(Clone, Debug, PartialEq)]
pub enum InstanceType {
    T2Micro,
    T3Medium,
    M5Large,
}

impl std::str::FromStr for InstanceType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "t2.micro" => Ok(InstanceType::T2Micro),
            "t3.medium" => Ok(InstanceType::T3Medium),
            "m5.large" => Ok(InstanceType::M5Large),
            _ => Err(format!("Invalid instance type: {}", s)),
        }
    }
}

impl std::fmt::Display for InstanceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InstanceType::T2Micro => write!(f, "t2.micro"),
            InstanceType::T3Medium => write!(f, "t3.medium"),
            InstanceType::M5Large => write!(f, "m5.large"),
        }
    }
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
#[flag(name = "verbose", short = 'v', help = "Enable verbose logging")]
pub struct VerboseFlag(#[allow(dead_code)] pub bool);

#[derive(Flag, Debug, Clone)]
#[flag(
    name = "profile",
    help = "Profile name to use",
    help_heading = "Config"
)]
pub struct ProfileFlag(#[allow(dead_code)] pub String);

#[derive(Flag, Debug, Clone)]
#[flag(
    name = "region",
    short = 'r',
    help = "Specify the region",
    default = "us-east-1"
)]
pub struct RegionFlag(#[allow(dead_code)] pub Region);

#[derive(Flag, Debug, Clone)]
#[flag(
    name = "format",
    help = "Output format (json, text, table)",
    default = "text"
)]
pub struct FormatFlag(#[allow(dead_code)] pub OutputFormat);

#[derive(Flag, Debug, Clone)]
#[flag(name = "type", help = "Instance type", default = "t2.micro")]
pub struct InstanceTypeFlag(#[allow(dead_code)] pub InstanceType);

#[derive(Flag, Debug, Clone)]
#[flag(name = "user", required = true, help = "Username")]
pub struct UserFlag(#[allow(dead_code)] pub String);

#[derive(Flag, Debug, Clone)]
#[flag(
    name = "token",
    env = "CLOUD_CLI_TOKEN",
    help = "Authentication token",
    help_heading = "Config"
)]
pub struct TokenFlag(#[allow(dead_code)] pub String);
