use super::super::{traits::CType, TableUpdateInterval, TableUpdateRate};
use lzham_sys::lzham_decompress_params;
use std::{
    ops::BitOr,
    os::raw::{c_uint, c_void},
};

/// Options to tweak decompression.
#[derive(Clone, Debug)]
pub struct DecompressionOptions {
    /// The base 2 log of the dictionary size.
    pub dict_size_log2: u32,
    /// The table update rate.
    ///
    /// It can be overwritten by specifying [`table_update_interval`](DecompressionOptions::table_update_interval).
    pub table_update_rate: TableUpdateRate,
    /// Flags to pass to the decompression.
    ///
    /// Currently unimplemented.
    pub decompression_flags: Option<DecompressionFlag>,
    /// Number of seed bytes to load.
    pub num_seed_bytes: Option<u32>,
    /// A vector of seed bytes to load.
    pub p_seed_bytes: Option<Vec<u8>>,
    /// The table update interval.
    pub table_update_interval: Option<TableUpdateInterval>,
}

/// Flag to tweak decompression.
///
/// Currently unimplemented.
#[derive(Clone, Copy, Debug)]
pub enum DecompressionFlag {
    OutputUnbuffered = 1,
    ComputeAdler32 = 2,
    ReadZlibStream = 4,
}

/// The status of decompression.
#[derive(Clone, Copy, Debug)]
pub enum DecompressionStatus {
    NotFinished,
    HasMoreOutput,
    NeedsMoreInput,
    Success,
    FailedInitialization,
    OutputBufferTooSmall,
    ExpectedMoreRawBytes,
    BadCode,
    Adler32,
    BadRawBlock,
    BadCompBlockSyncCheck,
    BadZlibHeader,
    NeedSeedBytes,
    BadSeedBytes,
    BadSyncBlock,
    InvalidParameter,
    // Not same as the actual `lzham_codec`. Conversion from the C-type should not require this.
    Failed,
}

impl Default for DecompressionOptions {
    fn default() -> Self {
        Self {
            dict_size_log2: 26,
            table_update_rate: TableUpdateRate::DEFAULT,
            decompression_flags: None,
            num_seed_bytes: None,
            p_seed_bytes: None,
            table_update_interval: None,
        }
    }
}

impl BitOr for DecompressionFlag {
    type Output = Self;

    fn bitor(self, _rhs: Self) -> Self {
        self
    }
}

impl DecompressionStatus {
    pub(crate) fn from_c_type(status: lzham_sys::lzham_decompress_status_t) -> Self {
        match status {
            0 => Self::NotFinished,
            1 => DecompressionStatus::HasMoreOutput,
            2 => DecompressionStatus::NeedsMoreInput,
            3 => DecompressionStatus::Success,
            4 => DecompressionStatus::FailedInitialization,
            5 => DecompressionStatus::OutputBufferTooSmall,
            6 => DecompressionStatus::ExpectedMoreRawBytes,
            7 => DecompressionStatus::BadCode,
            8 => DecompressionStatus::Adler32,
            9 => DecompressionStatus::BadRawBlock,
            10 => DecompressionStatus::BadCompBlockSyncCheck,
            11 => DecompressionStatus::BadZlibHeader,
            12 => DecompressionStatus::NeedSeedBytes,
            13 => DecompressionStatus::BadSeedBytes,
            14 => DecompressionStatus::BadSyncBlock,
            15 => DecompressionStatus::InvalidParameter,
            _ => panic!("decomp `status` out of bounds"),
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

impl CType for DecompressionOptions {
    type CItem = lzham_decompress_params;

    fn to_c_type(self) -> Self::CItem {
        let (max, slow) = self.table_update_interval.unwrap_or_default().to_c_type();

        lzham_decompress_params {
            m_struct_size: std::mem::size_of::<lzham_decompress_params>() as c_uint,
            m_dict_size_log2: self.dict_size_log2 as c_uint,
            m_table_update_rate: self.table_update_rate.to_c_type(),
            m_decompress_flags: self
                .decompression_flags
                .map_or_else(|| 0, |f| f.to_c_type()),
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

impl CType for DecompressionFlag {
    type CItem = lzham_sys::lzham_decompress_flags;

    fn to_c_type(self) -> Self::CItem {
        self as c_uint
    }
}

impl CType for DecompressionStatus {
    type CItem = lzham_sys::lzham_decompress_status_t;

    fn to_c_type(self) -> Self::CItem {
        self as c_uint
    }
}
