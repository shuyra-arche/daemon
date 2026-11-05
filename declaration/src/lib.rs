#![no_std]

//! Legacy Daemon UnitFile and image build compatibility declaration.
//! Current runtime policy is owned by the repository's `unit.toml`.

use unit::{Ending, ImageBuild, UnitFile};

pub static UNIT: UnitFile = UnitFile {
    name: "daemon",
    loader: unit::LoaderConfig::user("daemon"),
    operator: unit::OperatorConfig::DEFAULT,
    identity: unit::Identity::DEFAULT,
    control: unit::ControlConfig {
        supervision: Some("supervised"),
        after: Some(&[
            unit::Dependency::Bootstrap(unit::Bootstrap::Operator),
            unit::Dependency::Target("supervised"),
        ]),
        ending: Some(Ending::Resident),
        ready: unit::Ready::Signal(unit::READY_MARK),
        construction: &[],
        retirement: &[],
    },
    supply: unit::SupplyConfig::DEFAULT,
};

pub const BUILD: ImageBuild = ImageBuild {
    image: "daemon",
    manifest: "server/Cargo.toml",
    package: "daemon-server",
    bin: "daemon",
    target: "riscv64gc-unknown-none-elf",
    profile: "release",
    features: &[],
    default_features: true,
};
