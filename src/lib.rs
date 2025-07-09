//! [SoapySDR](https://github.com/pothosware/SoapySDR/wiki) provides a hardware abstraction layer
//! for transmitting and receiving with many software defined radio devices.
//!
//!

mod args;
pub use args::{Args, ArgsIterator};

mod arginfo;
pub use arginfo::ArgInfo;

mod device;
pub use device::{enumerate, Device, RxStream, TxStream, Error, ErrorCode, Direction, Range, StreamFlags, StreamResult};

mod format;
pub use format::{Format, StreamSample};

/// Configures SoapySDR to log to the Rust `log` facility.
///
/// With `env_logger`, use e.g `RUST_LOG=soapysdr=info` to control the log level.
#[cfg(feature="log")]
pub fn configure_logging() {
    use log::log;
    use log::Level;
    use soapysdr_sys::*;
    use std::os::raw::c_char;
    use std::ffi::CStr;

    extern "C" fn soapy_log(level: SoapySDRLogLevel, message: *const c_char) {
        #![allow(non_upper_case_globals)]
        let level = match level {
                SoapySDRLogLevel_SOAPY_SDR_FATAL    => Level::Error,
                SoapySDRLogLevel_SOAPY_SDR_CRITICAL => Level::Error,
                SoapySDRLogLevel_SOAPY_SDR_ERROR    => Level::Error,
                SoapySDRLogLevel_SOAPY_SDR_WARNING  => Level::Warn,
                SoapySDRLogLevel_SOAPY_SDR_NOTICE   => Level::Info,
                SoapySDRLogLevel_SOAPY_SDR_INFO     => Level::Info,
                SoapySDRLogLevel_SOAPY_SDR_DEBUG    => Level::Debug,
                SoapySDRLogLevel_SOAPY_SDR_TRACE    => Level::Trace,
                SoapySDRLogLevel_SOAPY_SDR_SSI      => Level::Info, // Streaming status indicators such as "U" (underflow) and "O" (overflow).
                _ => Level::Error,
        };

        let msg = unsafe { CStr::from_ptr(message) };
        log!(level, "{}", msg.to_string_lossy().trim_start_matches(&['\r', '\n'][..]));
    }

    unsafe {
        SoapySDR_registerLogHandler(Some(soapy_log));
    }
}

/// Convert a tick count into a time in nanoseconds using the tick rate.
pub fn ticks_to_time_ns(ticks: i64, rate: f64) -> i64 {
    use soapysdr_sys::SoapySDR_ticksToTimeNs;
    unsafe {
        SoapySDR_ticksToTimeNs(ticks, rate)
    }
}

/// Convert a time in nanoseconds into a tick count using the tick rate.
pub fn time_ns_to_ticks(time_ns: i64, rate: f64) -> i64 {
    use soapysdr_sys::SoapySDR_timeNsToTicks;
    unsafe {
        SoapySDR_timeNsToTicks(time_ns, rate)
    }
}
