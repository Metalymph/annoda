use annoda::{Adapter, AdapterEvent, PeerId};
use annoda_adapter_loopback::LoopbackAdapter;
use annoda_testkit::assert_static_contract;

#[test]
fn round_trip_frame() {
    let alice_id = PeerId::new(b"alice".to_vec()).expect("alice id");
    let bob_id = PeerId::new(b"bob".to_vec()).expect("bob id");

    let (mut alice, mut bob) = LoopbackAdapter::pair(alice_id.clone(), bob_id.clone());

    assert_static_contract(&alice);
    assert_static_contract(&bob);

    alice.connect(&bob_id).expect("alice connects");
    bob.connect(&alice_id).expect("bob connects");

    alice.send(&bob_id, b"ping").expect("alice sends");

    let mut payload = None;

    for _ in 0..4 {
        if let Some(AdapterEvent::Frame {
            peer,
            payload: frame,
        }) = bob.poll_event().expect("bob poll")
        {
            assert_eq!(peer, alice_id);
            payload = Some(frame);
            break;
        }
    }

    assert_eq!(payload.as_deref(), Some(b"ping".as_slice()));
}
