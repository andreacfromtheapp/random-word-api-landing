fn main() {
    let api_url = std::env::var("API_URL").unwrap_or_else(|_| "http://localhost:3000".to_string());
    println!("cargo:rustc-env=API_URL={}", api_url);
}
