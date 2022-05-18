#[test]
fn it_works() {
    let hex_hash = "054f802339a094d21c8861ea15be44bbac9d1ee722f554dfce2affcb17c3bf69".to_string();
    let hex_vec = hex::decode(format!("cd1400{}", hex_hash)).expect("could not parse hex string");

    let b58c_encoded = crate::encode(hex_vec).expect("Could not encode hex");
    assert_eq!("3JuESn7NTskLD6NG43EEwKmyHydHYhRR8twyJfMYE4snCfiuu853Ww", b58c_encoded);
}