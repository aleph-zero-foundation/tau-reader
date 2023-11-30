use aws_sdk_s3 as s3;
use bellman::pairing::bn256::Bn256;
use bellman::pairing::ff::{Field, ScalarEngine};
use bellman::pairing::{CurveAffine, CurveProjective, Engine};
use rand::Rng;

use tau_reader::parameters::NUMBER_OF_POWERS;
use tau_reader::{aws_access, get_g1, get_g2};

#[tokio::main]
async fn main() -> Result<(), s3::Error> {
    let tau = <Bn256 as ScalarEngine>::Fr::from_hex(
        "0x1f8cd6a3d6ef1026a9b58c087935c9b5516c438fe5aaee2d8668b6baba96c605",
    )
    .unwrap();

    let client = aws_access::access_aws().await;

    for range in [0..10_000usize, 0..NUMBER_OF_POWERS] {
        for _ in 0..100 {
            let power = rand::thread_rng().gen_range(range.clone());
            print!("Getting points for the power: `{power}`...");

            let g1 = get_g1(power, &client).await;
            let expected_g1 = <Bn256 as Engine>::G1Affine::one().mul(tau.pow(&[power as u64]));
            assert_eq!(
                g1,
                expected_g1.into_affine(),
                "G1 {power} power is incorrect"
            );

            let g2 = get_g2(power, &client).await;
            let expected_g2 = <Bn256 as Engine>::G2Affine::one().mul(tau.pow(&[power as u64]));
            assert_eq!(
                g2,
                expected_g2.into_affine(),
                "G2 {power} power is incorrect"
            );

            println!(" ✅")
        }
    }

    Ok(())
}
