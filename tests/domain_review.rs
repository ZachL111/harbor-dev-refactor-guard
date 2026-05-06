use harbor_dev_refactor_guard::domain_review::{review_lane, review_score, DomainCase};

#[test]
fn domain_review_case_is_stable() {
    let case = DomainCase { signal: 55, slack: 48, drag: 17, confidence: 59 };
    assert_eq!(review_score(case), 166);
    assert_eq!(review_lane(case), "ship");
}
