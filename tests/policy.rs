use harbor_dev_refactor_guard::{classify, score, Signal};
#[test]
fn fixture_decisions() {
    let signal = Signal { demand: 79, capacity: 106, latency: 22, risk: 10, weight: 6 };
    assert_eq!(score(signal), 134);
    assert_eq!(classify(signal), "review");
    let signal = Signal { demand: 93, capacity: 72, latency: 25, risk: 19, weight: 4 };
    assert_eq!(score(signal), 56);
    assert_eq!(classify(signal), "review");
    let signal = Signal { demand: 101, capacity: 98, latency: 27, risk: 8, weight: 13 };
    assert_eq!(score(signal), 183);
    assert_eq!(classify(signal), "accept");
}
