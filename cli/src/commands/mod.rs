use std::env;

/// Contains the types and functions to support the Enrich Command.
pub mod enrich;
/// Contains the types and functions to support the SBOM Command.
pub mod sbom;
mod snyk;


// /// Extracts the value of an environment variable
// ///
// fn get_env_var(variable_name: &str) -> Option<String> {
//     return match env::var(variable_name) {
//         Ok(v) => Some(v),
//         Err(e) => None,
//     };
// }

// #[tokio::test]
// async fn test_get_env_var() {
//     let var_name: &str = "TEST_ENV_VAR";
//     let var_value: &str = "testvalue";
//     env::set_var(var_name, var_value);

//     match get_env_var(var_name) {
//         Some(value) => assert_eq!(var_value, value),
//         None => panic!("No test value...?")
//     }
// }