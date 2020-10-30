use super::super::{TableUpdateRate, TableUpdateInterval, traits::CType};
use lzham_sys::lzham_compress_params;
use std::{
    ops::BitOr,
    os::raw::{c_int, c_uint, c_void},
};

/// Options to tweak compression.
#[derive(Clone, Debug)]
pub struct CompressionOptions {
    /// The base 2 log of the dictionary size.
    pub dict_size_log2: u32,
    /// The level of compression.
    pub compression_level: CompressionLevel,
    /// The table update rate.
    ///
    /// It can be overwritten by specifying [`table_update_interval`](CompressionOptions::table_update_interval).
    pub table_update_rate: TableUpdateRate,
    /// The maximum number of helper threads to use.
    pub max_helper_threads: i32,
    /// Flags to pass to the compression.
    ///
    /// Currently unimplemented.
    pub compression_flags: Option<CompressionFlag>,
    /// Number of seed bytes to load.
    pub num_seed_bytes: Option<u32>,
    /// A vector of seed bytes to load.
    pub p_seed_bytes: Option<Vec<u8>>,
    /// The table update interval.
    pub table_update_interval: Option<TableUpdateInterval>,
}

/// The level of compression.
#[derive(Clone, Copy, Debug)]
pub enum CompressionLevel {
    /// Fastest compression.
    Fastest,
    /// Fast compression.
    Faster,
    /// Default compression.
    Default,
    /// Moderate compression.
    Better,
    /// Slowest compression.
    Uber,
}

/// Flag to tweak compression.
///
/// Currently unimplemented.
#[derive(Clone, Copy, Debug)]
pub enum CompressionFlag {
    ExtremeParsing = 2,
    DeterminisiticParsing = 4,
    HighCompressionRatio = 16,
    WriteZlibStream = 32,
}

/// The status of compression.
#[derive(Clone, Copy, Debug)]
pub enum CompressionStatus {
    NotFinished,
    NeedsMoreInput,
    HasMoreOutput,
    Success,
    Failed,
    FailedInitialization,
    InvalidParameter,
    OutputBufferTooSmall,
}

impl Default for CompressionOptions {
    fn default() -> Self {
        Self {
            dict_size_log2: 26,
            compression_level: CompressionLevel::Default,
            table_update_rate: TableUpdateRate::DEFAULT,
            max_helper_threads: 0,
            compression_flags: None,
            num_seed_bytes: None,
            p_seed_bytes: None,
            table_update_interval: None,
        }
    }
}

impl BitOr for CompressionFlag {
    type Output = Self;

    fn bitor(self, _rhs: Self) -> Self {
        self
    }
}

impl CompressionStatus {
    pub(crate) fn from_c_type(status: lzham_sys::lzham_compress_status_t) -> Self {
        match status {
            0 => Self::NotFinished,
            1 => Self::NeedsMoreInput,
            2 => Self::HasMoreOutput,
            3 => Self::Success,
            4 => Self::Failed,
            5 => Self::FailedInitialization,
            6 => Self::InvalidParameter,
            7 => Self::OutputBufferTooSmall,
            _ => panic!("comp `status` out of bounds"),
        }
    }

    /// Whether the status is of success or not.
    pub fn is_success(&self) -> bool {
        if let Self::Success = self {
            true
        } else {
            false
        }
    }

    /// Whether the status is success or first failure code.
    ///
    /// If failure, it cannot be recovered.
    pub fn is_success_or_first_failure(&self) -> bool {
        *self as u32 >= 3
    }
}

/*
 * CType Implementations
 * ========================================================
*/

impl CType for CompressionOptions {
    type CItem = lzham_compress_params;

    fn to_c_type(self) -> Self::CItem {
        let (max, slow) = self.table_update_interval.unwrap_or_default().to_c_type();

        lzham_compress_params {
            m_struct_size: std::mem::size_of::<lzham_compress_params>() as c_uint,
            m_dict_size_log2: self.dict_size_log2 as c_uint,
            m_level: self.compression_level.to_c_type(),
            m_table_update_rate: self.table_update_rate.to_c_type(),
            m_max_helper_threads: self.max_helper_threads as c_int,
            m_compress_flags: self.compression_flags.map_or_else(|| 0, |f| f.to_c_type()),
            m_num_seed_bytes: self.num_seed_bytes.unwrap_or(0) as c_uint,
            m_pSeed_bytes: {
                if let Some(p) = self.p_seed_bytes {
                    p.as_ptr() as *const c_void
                } else {
                    std::ptr::null()
                }
            },
            m_table_max_update_interval: max,
            m_table_update_interval_slow_rate: slow,
        }
    }
}

impl CType for CompressionLevel {
    type CItem = lzham_sys::lzham_compress_level;

    fn to_c_type(self) -> Self::CItem {
        self as c_uint
    }
}

impl CType for CompressionFlag {
    type CItem = lzham_sys::lzham_compress_flags;

    fn to_c_type(self) -> Self::CItem {
        self as c_uint
    }
}

impl CType for CompressionStatus {
    type CItem = lzham_sys::lzham_compress_status_t;

    fn to_c_type(self) -> Self::CItem {
        self as c_uint
    }
}
