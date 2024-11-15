use std::{fmt::Display, ops::AddAssign};

pub(crate) struct DiskSize(u64);

impl DiskSize {
    pub fn zero() -> Self {
        Self(0)
    }
}

impl Display for DiskSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut size = self.0 as f64;
        let mut unit = "B";
        if size > 1024.0 {
            size /= 1024.0;
            unit = "KiB";
        }; // kibibytes
        if size > 1024.0 {
            size /= 1024.0;
            unit = "MiB";
        } // mebibytes
        if size > 1024.0 {
            size /= 1024.0;
            unit = "GiB";
        } // gibibytes
        if size > 1024.0 {
            size /= 1024.0;
            unit = "TiB";
        } // tebibytes
        write!(f, "{:.2} {}", size, unit)
    }
}

impl AddAssign<u64> for DiskSize {
    fn add_assign(&mut self, rhs: u64) {
        self.0 += rhs;
    }
}
