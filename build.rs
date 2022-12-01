fn main() {
    let install_dir = cmake::Config::new("./spdlog/")
        .cxxflag("-w")
        .build();

    println!(
        "cargo:rustc-link-search=native={}/lib/",
        install_dir.display()
    );
}
