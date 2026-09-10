use signal_message::{
    ByteViewable, MessageBody, MessageKind, MessageRecipient, MessageSubmission, Query, Response,
    Restorable, Signal, Signalizable, ThreadSelection,
};
fn submission() -> MessageSubmission {
    MessageSubmission {
        message_recipient: MessageRecipient::from("router"),
        message_kind: MessageKind::Send,
        message_body: MessageBody::from("current signal"),
        thread_selection: ThreadSelection::None,
    }
}
#[test]
fn message_query_and_response_restore_from_fresh_peer_bytes() {
    let query = Query::Submit(submission());
    let outgoing = query.signalize().expect("archive query");
    assert!(!outgoing.bytes().is_empty());
    let incoming = Signal::<Query>::from(outgoing.bytes().to_vec());
    assert_eq!(incoming.restore().expect("restore query"), query);
    let response = Response::SubmissionAccepted(41);
    let outgoing = response.signalize().expect("archive response");
    let incoming = Signal::<Response>::from(outgoing.bytes().to_vec());
    assert_eq!(incoming.restore().expect("restore response"), response);
}
#[test]
fn malformed_peer_bytes_are_rejected() {
    assert!(Signal::<Query>::from(vec![0xff, 0, 1]).restore().is_err());
}
#[cfg(feature = "datom")]
#[test]
fn datom_round_trip_preserves_message_payload() {
    use datom_codec::{Actualizing, Budget, Datomizable, Potential};
    use protos::{Protosizable, ReaderBudget, Textualizable};
    let query = Query::Submit(submission());
    let rendered = query.clone().datomize(vec![]).protosize().textualize();
    let mut pending = Potential::<Query>::from(rendered);
    let restored = pending
        .actualize(&mut Budget {
            remaining: 4096,
            reader: ReaderBudget { remaining: 4096 },
            depth: 0,
            maximum_depth: 256,
        })
        .expect("actualize");
    assert_eq!(restored, query);
}
