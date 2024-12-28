use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(strum_macros::Display, Serialize, Deserialize, Debug)]
pub enum Cpu {
    #[strum(serialize = "x86_64")]
    X86_64,
    #[strum(serialize = "x86_64")]
    Amd64,

    #[strum(serialize = "aarch64")]
    Aarch64,
    #[strum(serialize = "aarch64")]
    Arm64,

    #[strum(serialize = "ppc64")]
    Ppc64,
    // Riscv64,
    #[strum(serialize = "s390x")]
    S390x,
}

impl FromStr for Cpu {
    type Err = strum::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "x86_64" | "amd64" => Ok(Cpu::X86_64),
            "aarch64" | "arm64" => Ok(Cpu::Aarch64),
            "ppc64" => Ok(Cpu::Ppc64),
            "s390x" => Ok(Cpu::S390x),
            _ => Err(strum::ParseError::VariantNotFound),
        }
    }
}

fn main() {
    // Example usage
    let cpu_x86: Cpu = "x86_64".parse().unwrap();
    let cpu_amd: Cpu = "amd64".parse().unwrap();
    let cpu_aarch64: Cpu = "aarch64".parse().unwrap();
    let cpu_arm64: Cpu = "arm64".parse().unwrap();

    println!("{:?}", cpu_x86.to_string()); // Output: x86_64
    println!("{:?}", cpu_amd); // Output: x86_64
    println!("{:?}", cpu_aarch64); // Output: aarch64
    println!("{:?}", cpu_arm64); // Output: aarch64
}
