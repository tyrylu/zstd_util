# Zstd_util

This library provides a simple API on top of the zstd_safe crate for compression and decompression given a set of parameters and does not suffer the performance hits from repeated compress calls with the same dictionary.