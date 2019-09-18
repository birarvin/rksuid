extern crate rksuid;

#[cfg(test)]
mod tests {
    use ::rksuid::rksuid;
    use ::rksuid::rksuid::Ksuid;
    use chrono::prelude::*;
    use rand::distributions::Standard;
    use rand::prelude::*;
    use std::{thread, time};

    // Test get_time
    #[test]
    fn test_get_time() {
        // Friday, July 14, 2017 2:40:00 AM
        let timestamp: DateTime<Utc> = Utc.timestamp(1500000000, 0);
        let epoch_offset = timestamp
            .signed_duration_since(rksuid::gen_epoch())
            .num_seconds();
        // Sanity check the expected timestamp for the ksuid
        assert_eq!(100000000, epoch_offset);
        let ksuid = rksuid::new(Some(epoch_offset as u32), None);
        let ksuid_time = ksuid.get_time();
        assert_eq!(timestamp, ksuid_time);
    }
}
