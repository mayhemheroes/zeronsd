#![no_main]
use std::{
    io::{Seek, SeekFrom, Write},
    os::unix::prelude::AsRawFd,
};

use libfuzzer_sys::fuzz_target;

use trust_dns_proto::rr::domain::Name;

fuzz_target!(|data: &[u8]| {
    // write fuzz bytes to memfd hostsfile
    let mfd = match memfd::MemfdOptions::default().create("fuzz-file") {
        Ok(m) => m,
        Err(_) => return,
    };

    let fd = mfd.as_raw_fd();
    let filepath = format!("/proc/self/fd/{fd}");
    let filepath = filepath.into();

    let mut file = mfd.into_file();
    if file.write_all(data).is_err() {
        println!("could not write to memfd file!");
        return;
    }

    if file.seek(SeekFrom::Start(0)).is_err() {
        println!("failed to seek!");
        return;
    }

    let _ = zeronsd::hosts::parse_hosts(Some(filepath), Name::root());

    drop(file);
});
