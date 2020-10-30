pub mod compression;
pub mod decompression;
mod traits;

use traits::CType;
use std::os::raw::c_uint;

/// The table update rate for compression/decompression.
///
/// You can create this by using the [`new`] method or use one of the constants.
///
/// [`new`]: TableUpdateRate::new()
#[derive(Clone, Copy, Debug)]
pub struct TableUpdateRate(pub(crate) u32);

impl TableUpdateRate {
    /// The fastest table update rate. It has a rate of `20`.
    pub const FAST: TableUpdateRate = TableUpdateRate(20);
    /// The default table update rate. It has a rate of `8`.
    pub const DEFAULT: TableUpdateRate = TableUpdateRate(8);
    /// A moderately slow table update rate. It has a rate of `2`.
    pub const SLOW: TableUpdateRate = TableUpdateRate(2);
    /// The slowest table update rate. It has a rate of `1`.
    pub const VERY_SLOW: TableUpdateRate = TableUpdateRate(1);

    /// Create new [`TableUpdateRate`] from given `rate`.
    ///
    /// If `rate` is greater than the fastest rate, the fastest rate is used. Similarly,
    /// if `rate` is lower than the slowest rate, the slowest rate is used.
    pub fn new(rate: u32) -> Self {
        if rate > Self::FAST.0 {
            Self::FAST
        } else if rate > Self::VERY_SLOW.0 {
            Self::VERY_SLOW
        } else {
            Self(rate)
        }
    }

    /// Converts the update rate into [`TableUpdateInterval`].
    pub fn table_update_settings(&self) -> TableUpdateInterval {
        let (max, slow) = match self.0 {
            1 => (4, 32),
            2 => (5, 33),
            3 => (6, 34),
            4 => (7, 35),
            5 => (8, 36),

            6 => (16, 48),
            7 => (32, 72),
            8 => (64, 64),
            9 => (98, 80),
            10 => (128, 96),

            11 => (192, 112),
            12 => (256, 128),
            13 => (512, 160),
            14 => (1024, 192),
            15 => (2048, 224),

            16 => (2048, 128 + (16 * 8)),
            17 => (2048, 128 + (16 * 10)),
            18 => (2048, 128 + (16 * 12)),
            19 => (2048, 128 + (16 * 14)),
            20 => (2048, 128 + (16 * 16)),

            _ => unreachable!()
        };

        TableUpdateInterval(max, slow)
    }
}

/// The table update interval for compression/decompression.
///
/// It stores the max and the slowest interval.
#[derive(Clone, Copy, Debug)]
pub struct TableUpdateInterval(pub u32, pub u32);

impl TableUpdateInterval {
    /// Creates a new interval from [`TableUpdateRate`].
    pub fn from_rate(rate: &TableUpdateRate) -> Self {
        rate.table_update_settings()
    }
}

impl Default for TableUpdateInterval {
    fn default() -> Self {
        TableUpdateRate::DEFAULT.table_update_settings()
    }
}

impl CType for TableUpdateRate {
    type CItem = lzham_sys::lzham_table_update_rate;

    fn to_c_type(self) -> Self::CItem {
        self.0
    }
}

impl CType for TableUpdateInterval {
    type CItem = (c_uint, c_uint);

    fn to_c_type(self) -> Self::CItem {
        (self.0 as c_uint, self.1 as c_uint)
    }
}
