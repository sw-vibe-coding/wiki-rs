use wiki_common::aging::{AgeTier, age_tier};

const DAY: u64 = 86400;
const NOW: u64 = 1_700_000_000;

#[test]
fn test_fresh() {
    assert_eq!(age_tier(NOW - DAY, NOW), AgeTier::Fresh);
    assert_eq!(age_tier(NOW - 6 * DAY, NOW), AgeTier::Fresh);
}

#[test]
fn test_recent() {
    assert_eq!(age_tier(NOW - 7 * DAY, NOW), AgeTier::Recent);
    assert_eq!(age_tier(NOW - 29 * DAY, NOW), AgeTier::Recent);
}

#[test]
fn test_stale() {
    assert_eq!(age_tier(NOW - 30 * DAY, NOW), AgeTier::Stale);
    assert_eq!(age_tier(NOW - 89 * DAY, NOW), AgeTier::Stale);
}

#[test]
fn test_old() {
    assert_eq!(age_tier(NOW - 90 * DAY, NOW), AgeTier::Old);
    assert_eq!(age_tier(NOW - 179 * DAY, NOW), AgeTier::Old);
}

#[test]
fn test_ancient() {
    assert_eq!(age_tier(NOW - 180 * DAY, NOW), AgeTier::Ancient);
    assert_eq!(age_tier(NOW - 365 * DAY, NOW), AgeTier::Ancient);
}

#[test]
fn test_zero_timestamp_is_fresh() {
    assert_eq!(age_tier(0, NOW), AgeTier::Fresh);
}

#[test]
fn test_css_classes() {
    assert_eq!(AgeTier::Fresh.css_class(), "age-fresh");
    assert_eq!(AgeTier::Ancient.css_class(), "age-ancient");
}
