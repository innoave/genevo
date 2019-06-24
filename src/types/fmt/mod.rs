//! This module defines project specific formatting of struct values for
//! displaying values to the console or in log files.
//!
//! To do so it defines it defines its own `Display` trait, that is implemented
//! for some types used in this project.

use chrono::Duration;

pub trait Display {
    fn fmt(&self) -> String;
}

pub trait DisplayDuration: Display {
    fn fmt_seconds(&self, always_print_till_seconds: bool) -> String;
    fn fmt_sub_seconds(&self, always_print_millis: bool) -> String;
}

impl Display for Duration {
    fn fmt(&self) -> String {
        let (sign, abs) = duration_sign_abs(self);
        let duration_secs = abs.num_seconds();
        let duration_nanos = duration_sub_seconds(self);
        if duration_secs == 0 {
            if duration_nanos == 0 {
                format!("{}s", duration_secs)
            } else {
                sign + &fmt_duration_sub_seconds(duration_nanos, false)
            }
        } else if duration_nanos == 0 {
            sign + &fmt_duration_seconds(duration_secs, false)
        } else {
            sign + &fmt_duration_seconds(duration_secs, true)
                + " "
                + &fmt_duration_sub_seconds(duration_nanos, true)
        }
    }
}

impl DisplayDuration for Duration {
    fn fmt_seconds(&self, always_print_till_seconds: bool) -> String {
        let (sign, abs) = duration_sign_abs(self);
        sign + &fmt_duration_seconds(abs.num_seconds(), always_print_till_seconds)
    }

    fn fmt_sub_seconds(&self, always_print_millis: bool) -> String {
        let (sign, abs) = duration_sign_abs(self);
        sign + &fmt_duration_sub_seconds(duration_sub_seconds(&abs), always_print_millis)
    }
}

fn duration_sign_abs(duration: &Duration) -> (String, Duration) {
    if duration.num_seconds() < 0 {
        (format!("-"), -*duration)
    } else {
        (format!(""), *duration)
    }
}

fn duration_sub_seconds(duration: &Duration) -> i64 {
    match duration.num_nanoseconds() {
        Some(nanos) => nanos % 1_000_000_000,
        None => 0,
    }
}

fn fmt_duration_seconds(duration_secs: i64, always_print_till_seconds: bool) -> String {
    let weeks = duration_secs / (7 * 24 * 60 * 60);
    let sub_weeks = duration_secs % (7 * 24 * 60 * 60);
    let days = sub_weeks / (24 * 60 * 60);
    let sub_days = sub_weeks % (24 * 60 * 60);
    let hours = sub_days / (60 * 60);
    let sub_hours = sub_days % (60 * 60);
    let mins = sub_hours / 60;
    let secs = sub_hours % 60;
    if weeks == 0 {
        if days == 0 {
            if hours == 0 {
                if mins == 0 {
                    format!("{}s", secs)
                } else if secs == 0 && !always_print_till_seconds {
                    format!("{}m", mins)
                } else {
                    format!("{}m {}s", mins, secs)
                }
            } else if secs == 0 && !always_print_till_seconds {
                if mins == 0 {
                    format!("{}h", hours)
                } else {
                    format!("{}h {}m", hours, mins)
                }
            } else {
                format!("{}h {}m {}s", hours, mins, secs)
            }
        } else if secs == 0 && !always_print_till_seconds {
            if mins == 0 {
                if hours == 0 {
                    format!("{}d", days)
                } else {
                    format!("{}d {}h", days, hours)
                }
            } else {
                format!("{}d {}h {}m", days, hours, mins)
            }
        } else {
            format!("{}d {}h {}m {}s", days, hours, mins, secs)
        }
    } else if secs == 0 && !always_print_till_seconds {
        if mins == 0 {
            if hours == 0 {
                if days == 0 {
                    format!("{}w", weeks)
                } else {
                    format!("{}w {}d", weeks, days)
                }
            } else {
                format!("{}w {}d {}h", weeks, days, hours)
            }
        } else {
            format!("{}w {}d {}h {}m", weeks, days, hours, mins)
        }
    } else {
        format!("{}w {}d {}h {}m {}s", weeks, days, hours, mins, secs)
    }
}

fn fmt_duration_sub_seconds(duration_nanos: i64, always_print_millis: bool) -> String {
    let millis = duration_nanos / 1_000_000;
    let nanos = duration_nanos % 1_000;
    let micros = duration_nanos / 1_000 % 1_000;
    if millis == 0 && !always_print_millis {
        if nanos == 0 {
            if micros == 0 {
                String::new()
            } else {
                format!("{},000ns", micros)
            }
        } else if micros == 0 {
            format!("{}ns", nanos)
        } else {
            format!("{},{:03}ns", micros, nanos)
        }
    } else if nanos == 0 {
        if micros == 0 {
            format!("{}ms", millis)
        } else {
            format!("{}ms {},000ns", millis, micros)
        }
    } else if micros == 0 {
        format!("{}ms {}ns", millis, nanos)
    } else {
        format!("{}ms {},{:03}ns", millis, micros, nanos)
    }
}

#[cfg(test)]
mod tests;
