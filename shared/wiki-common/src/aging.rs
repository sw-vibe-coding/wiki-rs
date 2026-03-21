/// Age tier for visual staleness effects.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AgeTier {
    Fresh,   // < 7 days
    Recent,  // 7-30 days
    Stale,   // 30-90 days
    Old,     // 90-180 days
    Ancient, // 180+ days
}

impl AgeTier {
    pub fn css_class(self) -> &'static str {
        match self {
            Self::Fresh => "age-fresh",
            Self::Recent => "age-recent",
            Self::Stale => "age-stale",
            Self::Old => "age-old",
            Self::Ancient => "age-ancient",
        }
    }
}

const SECS_PER_DAY: u64 = 86400;

/// Determine the age tier of a page based on its last update time.
pub fn age_tier(updated_at: u64, now: u64) -> AgeTier {
    if updated_at == 0 || now < updated_at {
        return AgeTier::Fresh;
    }
    let age_days = (now - updated_at) / SECS_PER_DAY;
    match age_days {
        0..7 => AgeTier::Fresh,
        7..30 => AgeTier::Recent,
        30..90 => AgeTier::Stale,
        90..180 => AgeTier::Old,
        _ => AgeTier::Ancient,
    }
}
