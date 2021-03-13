pub fn main() {
    nightly_lints();
    march_06();
}

#[rustversion::since(2021-03-06)]
fn march_06() {
    println!("cargo:rustc-cfg=march_06")
}

#[rustversion::before(2021-03-06)]
fn march_06() {}

#[rustversion::not(nightly)]
fn march_06() {}

#[rustversion::since(2021-03-05)]
fn nightly_lints() {
    println!("cargo:rustc-cfg=nightly_lints");
}

#[rustversion::before(2021-03-05)]
fn nightly_lints() {}

#[rustversion::not(nightly)]
fn nightly_lints() {}
