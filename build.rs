/**
 * Calliope || build.rs
 * --------------------------------
 * Rust build script(?)
 *
 * Authors: @MaxineToTheStars <https://github.com/MaxineToTheStars>
 * ----------------------------------------------------------------
 */

fn main() {
    println!("cargo:rustc-link-arg-bins=-Tlinkall.x");
}
