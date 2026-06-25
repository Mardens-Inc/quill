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
        res.set("CompanyName", "Mardens Inc");
        res.set(
            "FileDescription",
            "A configuration interface for the quill auto print system",
        );
        res.set("ProductName", "Quill Configurator");

        res.compile().unwrap();
    }
}

fn generate_build_number() {
    let seconds_past_in_day = chrono::Local::now().num_seconds_from_midnight();
    let build = chrono::Local::now().format("%Y%m%d").to_string();
    let build = format!("{}.{:06}", build, seconds_past_in_day);
    println!("cargo:rustc-env=CARGO_PKG_BUILD={build}");
}
