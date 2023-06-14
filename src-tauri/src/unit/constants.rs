#![allow(dead_code)]

use const_format::formatcp;
use serde::Serialize;

// TODO: enumerate errors
#[derive(Debug, Serialize)]
pub enum FEError {
    ChangeRoleFailed,
    CourseEditInaccessible,
    CourseEditLoadError,
    CourseEditParseError,
    LoginFailed,
    LoginInaccessible,
    LoginLoadError,
    MainPageInaccessible,
    OtherError(String),
}

pub type FEResult<T> = Result<T, FEError>;
pub type NullFEResult = FEResult<()>;

pub const BASE_URL: &str = "https://academic.ui.ac.id/main";
pub const LOGIN_URL: &str = formatcp!("{}/Authentication/Index", BASE_URL);
pub const CHANGEROLE_URL: &str = formatcp!("{}/Authentication/ChangeRole", BASE_URL);
pub const COURSEPLANEDIT_URL: &str = formatcp!("{}/CoursePlan/CoursePlanEdit", BASE_URL);
const SCHEDULE_URL: &str = formatcp!("{}/Schedule/Index", BASE_URL);
const SUMMARY_URL: &str = formatcp!("{}Authentication/Summary", BASE_URL);