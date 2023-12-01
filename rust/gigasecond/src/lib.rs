use time::Duration;
use time::{PrimitiveDateTime};

// Returns a DateTime one billion seconds after start.
pub fn after(start: PrimitiveDateTime) -> PrimitiveDateTime {
    const GIGASECOND: time::Duration = Duration::seconds(1_000_000_000);
    start + GIGASECOND
}
