use chrono::Timelike;

fn main() {
    icon();
    generate_build_number();
}

fn icon() {
    #[cfg(windows)]
    {
        let path = "../../res/icons/icon.ico";
        println!("cargo:rerun-if-changed={}", path);
        let mut res = winresource::WindowsResource::new();
        res.set_icon(path);
        res.set("CompanyName", "LFInteractive LLC.");
        res.set("FileDescription", "A modern SFTP, FTP, and cloud storage explorer with a built-in shell and code editor — designed for keyboards, made for the dark.");
        res.set("ProductName", "QuaySFTP");

        res.compile().unwrap();
    }
}

fn generate_build_number() {
    let seconds_past_in_day = chrono::Local::now().num_seconds_from_midnight();
    let build = chrono::Local::now().format("%Y%m%d").to_string();
    let build = format!("{}.{:06}", build, seconds_past_in_day);
    println!("cargo:rustc-env=BUILD={build}");
}
