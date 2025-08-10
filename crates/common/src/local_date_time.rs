use crate::prelude::*;



use chrono::{
    Local,
    DateTime,
    Offset,
    FixedOffset,
    format::SecondsFormat,
};



#[derive(Clone, Debug, Serialize, Deserialize, Builder)]
pub struct LocalDateTime {
    pub utc_timestamp: i64,
    pub offset: i32,
}


impl LocalDateTime {
    pub fn now() -> Self {
        let dt = Local::now();

        let offset = dt.fixed_offset().offset().local_minus_utc();
        let utc_timestamp = dt.fixed_offset().timestamp();
        Self {
            utc_timestamp,
            offset,
        }
    }

    pub fn to_datetime(&self) -> AResult<DateTime::<FixedOffset>> {
        let offset = FixedOffset::east_opt(self.offset)
            .ok_or(msg("FixedOffset::east_opt failed"))?;

        let naive_utc = DateTime::from_timestamp(self.utc_timestamp, 0)
            .ok_or(msg("DateTime::from_timestamp failed"))?
            .naive_utc();

        let dt = DateTime::<FixedOffset>::from_naive_utc_and_offset(naive_utc, offset);

        Ok(dt)
    }
}

pub fn test_ldt() {
    let dt = LocalDateTime::now();

    // let dt_str = format!("{}", dt);
    println!("\n'{:#?}'\n", dt);
    // println!("\n'{}'\n", dt);
    println!("\n'{}'\n", dt.to_datetime().unwrap());
    // println!("\n'{}'\n", dt.offset().fix().local_minus_utc());
}


pub fn test_dt() {
    let dt = Local::now();

    // let dt_str = format!("{}", dt);
    // println!("\n'{}'\n", dt_str);
    println!("\n'{}'\n", dt.to_rfc3339_opts(SecondsFormat::Secs, false));
    // println!("\n'{}'\n", dt.offset());
    println!("\n'{}'\n", dt.offset().fix().local_minus_utc());
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_chrono() {
        test_ldt();
        // assert_eq!(counter.count, 1);
    }
}
