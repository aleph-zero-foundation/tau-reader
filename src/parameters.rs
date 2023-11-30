/// AWS S3 bucket name where the powers of tau are stored.
pub const BUCKET_NAME: &str = "piomiko-powersoftau";

/// Number of powers of tau generated and available.
pub const NUMBER_OF_POWERS: usize = (1 << 28) * 9 * 29 + 1;

/// First bytes of the response are the challenge hash (actually zeroed out).
pub const CHALLENGE_HASH_SIZE: usize = 64;

/// Compressed G1 point size.
pub const G1_SIZE: usize = 32;
/// Compressed G2 point size.
pub const G2_SIZE: usize = 64;

/// First 6 chunks are of 1TB each, the last one contains remaining data.
pub const CHUNK_SIZE: usize = 1 << 40;
