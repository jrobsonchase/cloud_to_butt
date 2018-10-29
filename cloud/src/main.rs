extern crate cloud_to_butt;

use cloud_to_butt::cloud_to_butt;

#[cloud_to_butt]
fn main() {
    let cloud = 8;

    // Shouldn't work
    let a = syn::parse::<proc_macro2::TokenTree>;

    println!("cloud: {}", cloud);
    println!("Hello, cloud!");
}
