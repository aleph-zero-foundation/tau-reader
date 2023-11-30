pub mod aws_access;
mod offsets;
pub mod parameters;

use crate::parameters::{BUCKET_NAME, G1_SIZE, G2_SIZE, NUMBER_OF_POWERS};
use aws_sdk_s3::Client;
use bellman::pairing::{
    bn256::{Bn256, G1Affine, G2Affine},
    CurveAffine, EncodedPoint, Engine,
};
use std::io::Read;

type Chunk = &'static str;
type G1Encoded = <<Bn256 as Engine>::G1Affine as CurveAffine>::Compressed;
type G2Encoded = <<Bn256 as Engine>::G2Affine as CurveAffine>::Compressed;

pub async fn get_g1(index: usize, client: &Client) -> G1Affine {
    assert!(index < NUMBER_OF_POWERS, "index out of bounds");
    let (chunk, inner_offset) = offsets::compute_g1_position(index);
    read_point::<G1_SIZE, G1Encoded>(chunk, inner_offset, client).await
}

pub async fn get_g2(index: usize, client: &Client) -> G2Affine {
    assert!(index < NUMBER_OF_POWERS, "index out of bounds");
    let (chunk, inner_offset) = offsets::compute_g2_position(index);
    read_point::<G2_SIZE, G2Encoded>(chunk, inner_offset, client).await
}

async fn read_point<const POINT_SIZE: usize, Enc: EncodedPoint>(
    chunk: Chunk,
    inner_offset: usize,
    client: &Client,
) -> Enc::Affine {
    let mut bytes = Vec::new();

    let mut stream = client
        .get_object()
        .bucket(BUCKET_NAME)
        .key(chunk)
        .range(format!(
            "bytes={inner_offset}-{}",
            inner_offset + POINT_SIZE - 1
        ))
        .send()
        .await
        .expect("must get object");

    while let Some(data) = stream.body.next().await {
        bytes.extend(data.expect("must read data"));
    }

    assert_eq!(bytes.len(), POINT_SIZE);

    let mut encoded = Enc::empty();
    bytes.as_slice().read_exact(encoded.as_mut()).unwrap();
    encoded.into_affine().unwrap()
}
