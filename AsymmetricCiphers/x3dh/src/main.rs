mod x3dh;
use ed25519_dalek::{Signature, VerifyingKey};

use x25519_dalek::PublicKey;

use crate::x3dh::*;

fn main() {
    // Bob initialization
    let ikb: IdentityKey = IdentityKey::new();
    let spkb: SignedPrekey = SignedPrekey::new();
    let opkb_bundle: Vec<OneTimePrekey> = OneTimePrekey::generate_opk_bundle(10);
    
    // Bob publishing key to the server
    let (server_ikb, server_spk, mut server_opkb_bundle, server_bob_signature_prekey, server_verifying_key): (PublicKey, PublicKey, Vec<PublicKey>, Signature, VerifyingKey);
    (server_ikb, server_spk, server_opkb_bundle, server_bob_signature_prekey, server_verifying_key) = create_prekey_bundle(&ikb, &spkb, &opkb_bundle);

    // Alice send the initial message (Ask information to the server)
    let ika: IdentityKey = IdentityKey::new();
    let (sk_alice, eka, opkb_used): ([u8; 32], PublicKey, Option<PublicKey>);
    let temp: Result<([u8; 32], PublicKey, Option<PublicKey>), X3DHError> = x3dh_sender(ika.clone(), server_ikb, server_spk, server_bob_signature_prekey, server_verifying_key, server_opkb_bundle.pop());
    match temp {
        Ok((current_sk, current_eka, current_opkb)) => {
            sk_alice = current_sk;
            eka = current_eka;
            opkb_used = current_opkb;
        },
        Err(error) => panic!("{}", error),
    };
    let ad = get_ad(ika.clone().get_public_key(), server_ikb, None);

    println!("AD: {:?}\n", ad);
    println!("SK: {:?}", sk_alice);

    // Bob receive the initial message
    let opkb_used_by_alice: Option<OneTimePrekey> = get_opk_used(opkb_used.unwrap(), opkb_bundle);
    let sk_bob: [u8; 32] = x3dh_receiver(ika.get_public_key(), eka, ikb, spkb, opkb_used_by_alice);
    
    assert_eq!(sk_alice, sk_bob);

    // Test without opk
    /* 
    let sk_without_opk = x3dh_sender(ika.clone(), server_ikb, server_spk, server_bob_signature_prekey, server_verifying_key, None);
    match sk_without_opk {
        Ok(res) => println!("{:?}", res),
        Err(error) => panic!("{}", error),
    };
    */
}
