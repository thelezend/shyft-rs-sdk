/// Sets up the test environment by loading environment variables from the .env.test file.
pub fn setup() {
    dotenvy::from_filename(".env.test").ok();
}
