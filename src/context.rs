//! Contains the main data structure, the ZSTD context.
use crate::Result;
use log::trace;
use std::time::Instant;
use zstd_safe::{CCtx, CDict, CParameter, DCtx};

/// The ZSTD compression/decompression context.
///
/// It allows to perform compression and decompression given a compression level and optionally a dictionary.
pub struct ZstdContext<'a> {
    compression_context: CCtx<'a>,
    _compression_dict: Option<CDict<'a>>,
    decompression_context: DCtx<'a>,
}

impl<'a> ZstdContext<'a> {
    /// Creates a new instance. The level must be in the range 0..=22.
    pub fn new(compression_level: i32, dictionary: Option<&[u8]>) -> Self {
        let mut cctx = CCtx::create();
        let mut dctx = DCtx::create();
        let cdict = match dictionary {
            Some(dict_data) => {
                let dict = CDict::create(dict_data, compression_level);
                cctx.ref_cdict(&dict)
                    .expect("Failed to associate compression dictionary");
                Some(dict)
            }
            None => {
                cctx.set_parameter(CParameter::CompressionLevel(compression_level))
                    .expect("Failed to se the compression level");
                None
            }
        };
        if let Some(dict_data) = dictionary {
            dctx.load_dictionary(dict_data)
                .expect("Failed to associate decompression dictionary");
        }
        Self {
            compression_context: cctx,
            decompression_context: dctx,
            _compression_dict: cdict,
        }
    }

    /// Compresses a slice of bytes using the parameters set during construction.
    pub fn compress(&mut self, data: &[u8]) -> Result<Vec<u8>> {
        let mut compressed = vec![0; zstd_safe::compress_bound(data.len())];
        let start = Instant::now();
        let compressed_size = self
            .compression_context
            .compress2(compressed.as_mut_slice(), data)?;
        compressed.resize(compressed_size, 0);
        trace!(
            "Compressed {} to {} bytes in {:?}.",
            data.len(),
            compressed.len(),
            start.elapsed()
        );
        Ok(compressed)
    }

    /// Decompresses a previously compressed data using the parameters given during construction. Mainly, the used dictionary must match, if any.
    pub fn decompress(&mut self, compressed: &[u8]) -> Result<Vec<u8>> {
        let mut original = vec![
            0;
            zstd_safe::get_frame_content_size(compressed)
                .unwrap_or(Some(1024))
                .unwrap_or(1024) as usize
        ];
        let start = Instant::now();
        self.decompression_context
            .decompress(original.as_mut_slice(), compressed)?;
        trace!(
            "Decompressed {} to {} bytes in {:?}.",
            compressed.len(),
            original.len(),
            start.elapsed()
        );
        Ok(original)
    }
}
