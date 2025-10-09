fn main() {
    let api_url = std::env::var("API_URL").unwrap_or_else(|_| "http://localhost:3000".to_string());
    println!("cargo:rustc-env=API_URL={}", api_url);
    
    let sps_url = std::env::var("SPS_URL").unwrap_or_else(|_| "http://localhost:5173".to_string());
    println!("cargo:rustc-env=SPS_URL={}", sps_url);
}
