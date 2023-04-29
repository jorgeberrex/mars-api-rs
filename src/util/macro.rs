pub mod unwrap_helper {
    // inline return of value
    macro_rules! return_default {
        ( $e:expr, $d:expr ) => {
            match $e {
                Some(x) => x,
                None => return ($d),
            }
        }
    }

    // inline continue
    macro_rules! continue_default {
        ( $e:expr ) => {
            match $e {
                Some(x) => x,
                None => continue,
            }
        }
    }

    macro_rules! result_return_default {
        ( $e:expr, $d:expr ) => {
            match $e {
                Ok(x) => x,
                // This is the Sus part
                Err(_) => return ($d),
            }
        }
    }

    pub(crate) use continue_default;
    pub(crate) use return_default;
    pub(crate) use result_return_default;
}
