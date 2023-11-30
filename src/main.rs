use aws_sdk_s3 as s3;
use bellman::pairing::bn256::Bn256;
use bellman::pairing::ff::{Field, ScalarEngine};
use bellman::pairing::{CurveAffine, CurveProjective, Engine};

use tau_reader::{aws_access, get_g1, get_g2};

#[tokio::main]
async fn main() -> Result<(), s3::Error> {
    let tau = <Bn256 as ScalarEngine>::Fr::from_hex(
        "0x1f8cd6a3d6ef1026a9b58c087935c9b5516c438fe5aaee2d8668b6baba96c605",
    )
    .unwrap();

    let client = aws_access::access_aws().await;

    for i in [0, 1, 2, 3, 4, 5, 123, 10000, 1239123usize] {
        print!("Getting points for the power: `{i}`...");

        let g1 = get_g1(i, &client).await;
        let expected_g1 = <Bn256 as Engine>::G1Affine::one().mul(tau.pow(&[i as u64]));
        assert_eq!(g1, expected_g1.into_affine(), "G1 {i} power is incorrect");

        let g2 = get_g2(i, &client).await;
        let expected_g2 = <Bn256 as Engine>::G2Affine::one().mul(tau.pow(&[i as u64]));
        assert_eq!(g2, expected_g2.into_affine(), "G2 {i} power is incorrect");

        println!(" ✅")
    }

    Ok(())
}
