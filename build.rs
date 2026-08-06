fn main() {
    println!("cargo:warning=build.rs is running");

    glib_build_tools::compile_resources(
        &["resources"],
        "resources/resources.gresource.xml",
        "emu-board.gresource",
    );

    println!("cargo:warning=Finished.");
}
