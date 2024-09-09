use anchor_lang::prelude::constant;

#[constant]
pub const CITIZENSHIP_FEE: u64 = 10;
#[constant]
pub const NUM_OF_JUDGES: u64 = 5;
#[constant]
pub const MINUTE: i64 = 60;
#[constant]
pub const HOUR: i64 = 60 * MINUTE;
#[constant]
pub const DAY: i64 = 24 * HOUR;
#[constant]
pub const WEEK: i64 = 7 * DAY;
#[constant]
pub const TWO_WEEKS: i64 = 2 * WEEK;
#[constant]
pub const MONTH: i64 = 30 * DAY;