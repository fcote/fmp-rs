pub enum FMPPeriod {
    YEAR,
    QUARTER,
}

impl FMPPeriod {
    pub fn value(&self) -> &str {
        match *self {
            FMPPeriod::YEAR => "year",
            FMPPeriod::QUARTER => "quarter",
        }
    }
}