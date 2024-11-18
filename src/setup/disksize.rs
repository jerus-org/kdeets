use std::{fmt::Display, ops::AddAssign};

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct DiskSize(u64);

impl DiskSize {
    pub fn zero() -> Self {
        Self(0)
    }

    #[allow(dead_code)]
    pub fn new(val: u64) -> Self {
        Self(val)
    }
}

impl Display for DiskSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut size = self.0 as f64;
        let mut unit = "B";
        if size >= 1024.0 {
            size /= 1024.0;
            unit = "KiB";
        }; // kibibytes
        if size >= 1024.0 {
            size /= 1024.0;
            unit = "MiB";
        } // mebibytes
        if size >= 1024.0 {
            size /= 1024.0;
            unit = "GiB";
        } // gibibytes
        if size >= 1024.0 {
            size /= 1024.0;
            unit = "TiB";
        } // tebibytes
        if size >= 1024.0 {
            size /= 1024.0;
            unit = "PiB";
        } // pebibytes
        if size >= 1024.0 {
            size /= 1024.0;
            unit = "EiB";
        } // exbibytes

        write!(f, "{:.2} {}", size, unit)
    }
}

impl AddAssign<u64> for DiskSize {
    fn add_assign(&mut self, rhs: u64) {
        self.0 += rhs;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_disksize_creation() {
        let size = DiskSize(1024);
        assert!(matches!(size, DiskSize(_)));
    }

    #[test]
    fn test_disksize_zero() {
        let size = DiskSize(0);
        assert!(matches!(size, DiskSize(_)));
    }

    #[test]
    fn test_disksize_large_value() {
        let size = DiskSize(u64::MAX);
        assert!(matches!(size, DiskSize(_)));
    }

    #[test]
    fn test_disksize_add_assign() {
        let mut size = DiskSize(100);
        size += 50;
        assert_eq!(size.0, 150);
    }

    #[test]
    fn test_disksize_add_assign_zero() {
        let mut size = DiskSize(100);
        size += 0;
        assert_eq!(size.0, 100);
    }

    #[test]
    fn test_disksize_add_assign_large_number() {
        let mut size = DiskSize(100);
        size += u64::MAX - 100;
        assert_eq!(size.0, u64::MAX);
    }

    #[test]
    fn test_disksize_add_assign_multiple_times() {
        let mut size = DiskSize(100);
        size += 50;
        size += 25;
        size += 25;
        assert_eq!(size.0, 200);
    }

    #[test]
    fn test_display_bytes() {
        let size = DiskSize(500);
        assert_eq!(format!("{}", size), "500.00 B");
    }

    #[test]
    fn test_display_kibibytes() {
        let size = DiskSize(2048);
        assert_eq!(format!("{}", size), "2.00 KiB");
    }

    #[test]
    fn test_display_mebibytes() {
        let size = DiskSize(2097152);
        assert_eq!(format!("{}", size), "2.00 MiB");
    }

    #[test]
    fn test_display_gibibytes() {
        let size = DiskSize(2147483648);
        assert_eq!(format!("{}", size), "2.00 GiB");
    }

    #[test]
    fn test_display_tebibytes() {
        let size = DiskSize(2199023255552);
        assert_eq!(format!("{}", size), "2.00 TiB");
    }

    #[test]
    fn test_display_zero() {
        let size = DiskSize(0);
        assert_eq!(format!("{}", size), "0.00 B");
    }

    #[test]
    fn test_display_maximum_size() {
        let size = DiskSize(u64::MAX);
        assert_eq!(format!("{}", size), "16.00 EiB");
    }

    #[test]
    fn test_display_exact_boundaries() {
        let size = DiskSize(1024);
        assert_eq!(format!("{}", size), "1.00 KiB");
        let size = DiskSize(1048576);
        assert_eq!(format!("{}", size), "1.00 MiB");
        let size = DiskSize(1073741824);
        assert_eq!(format!("{}", size), "1.00 GiB");
        let size = DiskSize(1099511627776);
        assert_eq!(format!("{}", size), "1.00 TiB");
        let size = DiskSize(1125899906842624);
        assert_eq!(format!("{}", size), "1.00 PiB");
        let size = DiskSize(1152921504606846976);
        assert_eq!(format!("{}", size), "1.00 EiB");
    }

    #[test]
    fn test_zero_returns_zero_disksize() {
        let size = DiskSize::zero();
        assert_eq!(size.0, 0);
    }

    #[test]
    fn test_zero_disksize_equality() {
        let size1 = DiskSize::zero();
        let size2 = DiskSize::zero();
        assert_eq!(size1, size2);
    }

    #[test]
    fn test_zero_disksize_is_empty() {
        let size = DiskSize::zero();
        assert!(size.0 == 0);
    }

    #[test]
    fn test_disksize_new_zero() {
        let size = DiskSize::new(0);
        assert_eq!(size.0, 0);
    }

    #[test]
    fn test_disksize_new_positive() {
        let size = DiskSize::new(1024);
        assert_eq!(size.0, 1024);
    }

    #[test]
    fn test_disksize_new_large_value() {
        let size = DiskSize::new(u64::MAX);
        assert_eq!(size.0, u64::MAX);
    }
}
