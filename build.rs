use std::env;
use std::fs;
use std::path::PathBuf;
fn main() {
    if env::var_os("CARGO_FEATURE_RT").is_some() {
        let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
        println!("cargo:rustc-link-search={}", out.display());
        let devices = [
            "msp430g2x01",
            "msp430g2x02",
            "msp430g2x03",
            "msp430g2x10",
            "msp430g2x30",
            "msp430g2x11",
            "msp430g2x21",
            "msp430g2x31",
            "msp430g2x12",
            "msp430g2x32",
            "msp430g2x52",
            "msp430g2x13",
            "msp430g2x33",
            "msp430g2x53",
            "msp430g2x44",
            "msp430g2x55",
        ];
        let mut device_file = None;
        for &d in &devices {
            if env::var_os(&format!("CARGO_FEATURE_{}", d.to_uppercase())).is_some() {
                device_file = Some(format!("devices/{d}.x"));
                break;
            }
        }
        if let Some(device_file) = device_file {
            fs::copy(&device_file, out.join("device.x")).unwrap();
            println!("cargo:rerun-if-changed={device_file}");
        } else {
            panic!("No device features selected. Avaliable device features are: {devices:?}");
        }
    }
    println!("cargo:rerun-if-changed=build.rs");
}
