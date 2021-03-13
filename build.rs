pub fn main() {
    nightly_lints();
    march_06();
}

#[rustversion::all(since(2021-03-06), nightly)]
fn march_06() {
    println!("cargo:rustc-cfg=march_06")
}

#[rustversion::all(before(2021-03-06), nightly)]
fn march_06() {}

#[rustversion::not(nightly)]
fn march_06() {}

#[rustversion::all(since(2021-03-05), nightly)]
fn nightly_lints() {
    println!("cargo:rustc-cfg=nightly_lints");
}

#[rustversion::all(before(2021-03-05),nightly)]
fn nightly_lints() {}

#[rustversion::not(nightly)]
fn nightly_lints() {}
