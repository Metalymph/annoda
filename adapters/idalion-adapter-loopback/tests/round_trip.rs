use idalion::{Adapter, AdapterEvent, EndpointId, OperationAcceptance, TextContent};
use idalion_adapter_loopback::LoopbackAdapter;
use idalion_testkit::{assert_bidirectional_text_support, assert_static_contract};

#[test]
fn round_trip_text_semantics() {
    let alice = EndpointId::new(b"alice".to_vec()).expect("alice endpoint");
    let bob = EndpointId::new(b"bob".to_vec()).expect("bob endpoint");

    let (mut alice_adapter, mut bob_adapter) = LoopbackAdapter::pair(alice.clone(), bob.clone());

    assert_static_contract(&alice_adapter);
    assert_static_contract(&bob_adapter);
    assert_bidirectional_text_support(&alice_adapter);
    assert_bidirectional_text_support(&bob_adapter);

    let content = TextContent::new("ciao Bob 👋").expect("valid text");

    let acceptance = alice_adapter
        .send_text(&bob, &content)
        .expect("adapter accepts text");

    assert_eq!(acceptance, OperationAcceptance::Accepted);

    let event = bob_adapter
        .poll_event()
        .expect("bob poll")
        .expect("text event");

    assert_eq!(
        event,
        AdapterEvent::TextReceived {
            source: alice,
            content,
        }
    );
}

#[test]
fn unknown_destination_is_rejected() {
    let alice = EndpointId::new(b"alice".to_vec()).expect("alice endpoint");
    let bob = EndpointId::new(b"bob".to_vec()).expect("bob endpoint");
    let mallory = EndpointId::new(b"mallory".to_vec()).expect("mallory endpoint");

    let (mut alice_adapter, _bob_adapter) = LoopbackAdapter::pair(alice, bob);
    let content = TextContent::new("wrong destination").expect("valid text");

    assert!(alice_adapter.send_text(&mallory, &content).is_err());
}
