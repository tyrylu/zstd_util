//! A simple library providing higher level zstd compression with a better performance characteristics for dictionary usage during repeated calls.
mod context;
mod error;
pub use context::ZstdContext;
pub use error::Error;

/// The type of results which this library provides.
pub type Result<T> = std::result::Result<T, Error>;

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn it_works() {
        env_logger::init();
        let mut ctx = ZstdContext::new(10, None);
        let original = "Hello, nice world.";
        let compressed = ctx
            .compress(&original.to_string().into_bytes())
            .expect("Failed to compress");
        let decompressed = ctx.decompress(&compressed).expect("Failed to decompress");
        assert_eq!(String::from_utf8_lossy(&decompressed), original);
    }

    #[test]
    fn it_works_with_a_dict() {
        let mut ctx = ZstdContext::new(5, Some(include_bytes!("../test.dict")));
        let original = "Hello, nice world, i would like to meet with you. AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAnd...";
        let compressed = ctx
            .compress(&original.to_string().into_bytes())
            .expect("Failed to compress");
        let decompressed = ctx.decompress(&compressed).expect("Failed to decompress");
        assert_eq!(String::from_utf8_lossy(&decompressed), original);
    }
}
