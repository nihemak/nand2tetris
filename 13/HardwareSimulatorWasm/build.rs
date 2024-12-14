use std::env;

fn main() {
    // FIXME: Remove this
    if let Ok(val) = env::var("TEST") {
    } else {
        // https://doc.rust-jp.rs/rust-by-example-ja/cargo/build_scripts.html
        // https://github.com/rustwasm/wasm-bindgen/issues/3368#issuecomment-1483954797
        // https://github.com/aduros/wasm4/blob/main/cli/assets/templates/rust/.cargo/config.toml
        // https://doc.rust-jp.rs/rust-by-example-ja/std/box.html
        println!("cargo::rustc-link-arg=-zstack-size=6000000");
    }
}
