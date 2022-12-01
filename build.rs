fn main() {

    cc::Build::new()
        .cpp(true)
        .file("foo.cc")
        .compile("libfoo");

    let install_dir = cmake::Config::new("./spdlog/")
        .cxxflag("-w")
        .build();

    println!(
        "cargo:rustc-link-search=native={}/lib/",
        install_dir.display()
    );
}
