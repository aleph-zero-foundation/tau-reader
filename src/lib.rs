pub mod aws_access;
mod offsets;
mod parameters;

use crate::parameters::{BUCKET_NAME, G1_SIZE, NUMBER_OF_POWERS};
use aws_sdk_s3::Client;
use bellman::pairing::bn256::Bn256;
use bellman::pairing::{bn256::G1Affine, CurveAffine, EncodedPoint, Engine};
use std::io::Read;

type Chunk = &'static str;

pub async fn get_g1(index: usize, client: &Client) -> G1Affine {
    assert!(index < NUMBER_OF_POWERS, "index out of bounds");
    let (chunk, inner_offset) = offsets::compute_g1_position(index);

    let mut bytes = Vec::new();

    let mut stream = client
        .get_object()
        .bucket(BUCKET_NAME)
        .key(chunk)
        .range(format!(
            "bytes={inner_offset}-{}",
            inner_offset + G1_SIZE - 1
        ))
        .send()
        .await
        .expect("must get object");
    while let Some(data) = stream.body.next().await {
        bytes.extend(data.expect("must read data"));
    }

    assert_eq!(bytes.len(), G1_SIZE, "must read exactly G1_SIZE bytes");

    let mut encoded =
        <<<Bn256 as Engine>::G1Affine as CurveAffine>::Compressed as EncodedPoint>::empty();
    bytes.as_slice().read_exact(encoded.as_mut()).unwrap();
    encoded.into_affine().unwrap()
}
