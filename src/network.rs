// TODO: flat mod into file
pub mod controller {
    pub struct NetworkController {}

    impl NetworkController {
        pub async fn new() -> Self {
            NetworkController {}
        }
    }

    // see https://github.com/dtolnay/async-trait for async trait
    impl Default for NetworkController {
        fn default() -> Self {
            NetworkController::new()
        }
    }
}
