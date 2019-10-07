use sled::Db;
//use chrono::prelude::*;
use serde::{Deserialize, Serialize};
//use chrono::{DateTime, Utc};
use std::fmt;

#[derive(Serialize, Deserialize, Debug)]
pub struct Package {
    pub kernel: Kernel,
    pub name: String,
    pub version: String,
    pub arch: CpuArchitecture,
    pub repo: PackageRepository,
    pub release: Release,
    pub split_pkgs: Vec<String>,
    pub description: String,
    pub upstream_url: String,
    pub licenses: Vec<License>,
    pub groups: String,
    pub provides: Vec<String>,
    pub replaces: Vec<String>,
    pub conflicts: Vec<String>,
    pub maintainers: Vec<String>,
    pub pkg_size: String,
    pub install_size: String,
    pub last_packager: String,
    pub build_date: String,
    pub signer: String,
    pub date_signed: String,
    pub last_update: String,
    pub contents: Vec<String>,
    pub dependencies: Vec<String>,
}

impl Package {
    pub fn _show_all_items(&self) {
        println!("Kernel: {}", self.kernel);
        println!("Name: {}", self.name);
        println!("Version: {}", self.version);
        println!("CPU Architecture: {}", self.arch);
        println!("Repository: {}", self.repo);
        println!("Release: {}", self.release);
        println!("Split Packages: {}", self.split_pkgs.join(", "));
        println!("Description: {}", self.description);
        println!("Upstream URL: {}", self.upstream_url);
        println!("License(s): {:?}", self.licenses);
        println!("Group(s): {}", self.groups);
        println!("Provide(s): {}", self.provides.join(", "));
        println!("Replace(s): {}", self.replaces.join(", "));
        println!("Conflict(s): {}", self.conflicts.join(", "));
        println!("Maintainers: {}", self.maintainers.join(", "));
        println!("Package Size: {}", self.pkg_size);
        println!("Install Size: {}", self.install_size);
        println!("Last Packager: {}", self.last_packager);
        println!("Build Date: {}", self.build_date);
        println!("Signer: {}", self.signer);
        println!("Date Signed: {}", self.date_signed);
        println!("Last Update: {}", self.last_update);
        println!("Contents: {}", self.contents.join(", "));
        println!("Dependencies: {}", self.dependencies.join(", "));
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum CpuArchitecture {
    X86_64,
    Arm,
    Power9,
    Riscv,
}

impl fmt::Display for CpuArchitecture {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CpuArchitecture::X86_64 => write!(f, "X86_64"),
            CpuArchitecture::Arm => write!(f, "Arm"),
            CpuArchitecture::Power9 => write!(f, "Power9"),
            CpuArchitecture::Riscv => write!(f, "RISC-V"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Release {
    Rolling,
    Stable,
    LTS,
}

impl fmt::Display for Release {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Release::Rolling => write!(f, "rolling"),
            Release::Stable => write!(f, "stable"),
            Release::LTS => write!(f, "lts"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Kernel {
    Linux,
    BSD,
    Redox,
}

impl fmt::Display for Kernel {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Kernel::Linux => write!(f, "Linux"),
            Kernel::BSD => write!(f, "BSD"),
            Kernel::Redox => write!(f, "Redox"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum PackageRepository {
    Main,
    MainTesting,
    Contrib,
    ContribTesting,
    Noss,
    NossTesting,
}

impl fmt::Display for PackageRepository {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PackageRepository::MainTesting => write!(f, "main-testing"),
            PackageRepository::Main => write!(f, "main"),
            PackageRepository::ContribTesting => write!(f, "contrib-testing"),
            PackageRepository::Contrib => write!(f, "contrib"),
            PackageRepository::NossTesting => write!(f, "noss-testing"),
            PackageRepository::Noss => write!(f, "noss"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum License {
    GPL2,
    LGPL2_1,
}

impl fmt::Display for License {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            License::GPL2 => write!(f, "GPL2"),
            License::LGPL2_1 => write!(f, "LGPL2.1"),
        }
    }
}

fn format_num(num: u64) -> String {
    let string = num.to_string();
    match num / 1000 as u64 {
        0..=9 => string[..1].to_string() + " KB",
        10..=99 => string[..2].to_string() + " KB",
        100..=999 => string[..3].to_string() + " KB",
        1_000..=9_999 => string[..1].to_string() + "." + &string[1..2] + " KB",
        10_000..=99_999 => string[..2].to_string() + "." + &string[1..1] + " MB",
        100_000..=999_999 => string[..3].to_string() + " MB",
        1_000_000..=9_999_999 => string[..1].to_string() + "." + &string[1..2] + " GB",
        10_000_000..=99_999_999 => string[..2].to_string() + "." + &string[1..1] + " GB",
        100_000_000..=999_999_999 => string[..3].to_string() + " GB",
        _ => string + " KB",
    }
}

pub fn read_db(path: String) -> sled::Db {
    let tree = Db::open(path).expect("Error opening database");

    let systemd = Package {
        kernel: Kernel::Linux,
        name: String::from("systemd"),
        version: String::from("243.51-1"),
        arch: CpuArchitecture::X86_64,
        repo: PackageRepository::Main,
        release: Release::Rolling,
        split_pkgs: vec![
            String::from("systemd-libs"),
            String::from("systemd-resolvconf"),
            String::from("systemd-sysvcompat"),
        ],
        description: String::from("system and service manager"),
        upstream_url: String::from("ttps://www.github.com/systemd/systemd"),
        licenses: vec![License::GPL2, License::LGPL2_1],
        groups: String::from("base-devel"),
        provides: vec![
            String::from("nss-myhostname"),
            String::from("systemd-tools=243.51"),
            String::from("udev=243.51"),
        ],
        replaces: vec![
            String::from("nss-myhostname"),
            String::from("systemd-tools"),
            String::from("udev"),
        ],
        conflicts: vec![
            String::from("nss-myhostname"),
            String::from("systemd-tools"),
            String::from("udev"),
        ],
        maintainers: vec![
            String::from("Dave Reisner"),
            String::from("Christian Hesse"),
        ],
        pkg_size: format_num(4_800_000),
        install_size: format_num(21_400_000),
        last_packager: String::from("Christian Hesse"),
        build_date: String::from("2019-09-22 18:35 UTC"),
        signer: String::from("Christian Hesse"),
        date_signed: String::from("2019-09-22 18:35 UTC"),
        last_update: String::from("2019-09-22 18:35 UTC"),
        contents: vec![
            String::from("file_1"),
            String::from("file_2"),
            String::from("file_1000"),
        ],
        dependencies: vec![
            String::from("nss-myhostname"),
            String::from("systemd-tools"),
            String::from("udev"),
        ],
    };

    let bytes = bincode::serialize(&systemd).expect("Error: converting package to bincode");
    for i in 0..50 {
        let string = String::from("systemd") + &i.to_string();
        tree.insert(string, bytes.clone())
            .expect("Error: Handling sled db");
    }

    tree
}
