use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Region {
    #[serde(rename = "us-east-1")]
    UsEast1,
    #[serde(rename = "us-west-1")]
    UsWest1,
    #[serde(rename = "eu-central-1")]
    EuCentral1,
    #[serde(rename = "ap-northeast-1")]
    ApNortheast1,
}

impl fmt::Display for Region {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Region::UsEast1 => write!(f, "us-east-1"),
            Region::UsWest1 => write!(f, "us-west-1"),
            Region::EuCentral1 => write!(f, "eu-central-1"),
            Region::ApNortheast1 => write!(f, "ap-northeast-1"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InstanceType {
    #[serde(rename = "t2.micro")]
    T2Micro,
    #[serde(rename = "t3.medium")]
    T3Medium,
    #[serde(rename = "m5.large")]
    M5Large,
}

impl fmt::Display for InstanceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InstanceType::T2Micro => write!(f, "t2.micro"),
            InstanceType::T3Medium => write!(f, "t3.medium"),
            InstanceType::M5Large => write!(f, "m5.large"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Instance {
    pub id: String,
    pub name: String,
    pub instance_type: InstanceType,
    pub region: Region,
    pub status: String,
    pub launched_at: DateTime<Utc>,
}

impl Instance {
    #[allow(dead_code)]
    pub fn new(name: String, instance_type: InstanceType, region: Region) -> Self {
        Self {
            id: format!(
                "i-{}",
                Uuid::new_v4().to_string().split('-').next().unwrap()
            ),
            name,
            instance_type,
            region,
            status: "running".to_string(),
            launched_at: Utc::now(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bucket {
    pub name: String,
    pub region: Region,
    pub file_count: usize,
    pub created_at: DateTime<Utc>,
}

impl Bucket {
    #[allow(dead_code)]
    pub fn new(name: String, region: Region) -> Self {
        Self {
            name,
            region,
            file_count: 0,
            created_at: Utc::now(),
        }
    }
}
