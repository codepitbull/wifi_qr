extern crate qrcodegen;
use qrcodegen::QrCode;
use qrcodegen::QrCodeEcc;

extern crate chrono;
use chrono::{Datelike, Timelike, Utc};

use std::fs::File;
use std::io::Write;
use std::process::Command;
use std::env;

extern crate rand;
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

static HTML_TEMPLATE:&str = " \
<html> \
<head> \
    <style> \
        * { \
            margin: 0; \
            padding: 0; \
        } \
        .imgbox { \
            display: grid; \
            height: 100%; \
        } \
        .center-fit { \
            max-width: 100%; \
            max-height: 100vh; \
            margin: auto; \
        } \
    </style> \
</head> \
<body> \
<div class=\"imgbox\"> \
    <img class= \"center-fit \" src= \"img_location\"> \
</div> \
</body> \
</html> \
";

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Please provide the id of the iface and the path to write the files to");
        std::process::exit(1);
    }

    let iface_id = &args[1].parse::<u32>().unwrap();
    let target_path = &args[2];

    set_key(iface_id);

    let key = get_key(iface_id);
    let ssid = get_ssid(iface_id);

    let string_to_encode = format!("WIFI:S:{};T:WPA;P:{};;", ssid, key);
    println!("Encoded: ({})", string_to_encode.to_string());
    let qr = QrCode::encode_text(format!("WIFI:S:{};T:WPA;P:{};;",ssid,key).as_str(),
                                 QrCodeEcc::High).unwrap();
    let svg = qr.to_svg_string(4);

    let now = Utc::now();
    let h = now.hour();
    let d = now.day();
    let absolute_img_path = format!("{}/qr_{}_{}.svg", target_path, h, d);
    let img_name = format!("qr_{}_{}.svg", h, d);
    let absolute_html_path = format!("{}/index.html", target_path);

    let mut qr_file = File::create(absolute_img_path.clone()).unwrap();
    qr_file.write_all(svg.as_bytes()).unwrap();

    let html_content = HTML_TEMPLATE.replace("img_location", img_name.as_str());
    let mut html_file = File::create(absolute_html_path).unwrap();
    html_file.write_all(html_content.as_bytes()).unwrap();
}

fn get_key(iface_id: &u32) -> String {
    let output = Command::new("/sbin/uci")
        .arg("get")
        .arg(format!("wireless.@wifi-iface[{}].key", iface_id))
        .output()
        .expect("failed to get key");

    let mut key = String::from_utf8_lossy(&output.stdout).to_string();
    let len = key.len();
    key.truncate(len - 1);
    return key;
}

fn set_key(iface_id: &u32) {
    let password: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(16)
        .collect();

    Command::new("/sbin/uci")
        .arg("set")
        .arg(format!("wireless.@wifi-iface[{}].key={}", iface_id, password))
        .output()
        .expect("failed to set key");
}

fn get_ssid(iface_id: &u32) -> String {
    let output = Command::new("/sbin/uci")
        .arg("get")
        .arg(format!("wireless.@wifi-iface[{}].ssid", iface_id))
        .output()
        .expect("failed to get ssid");
    let mut ssid = String::from_utf8_lossy(&output.stdout).to_string();
    let len = ssid.len();
    ssid.truncate(len - 1);
    return ssid;
}