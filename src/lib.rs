use std::env::var;
use std::path::PathBuf;
use std::process;

pub struct NearestEnv;

impl NearestEnv
{
    /// Searches upward from the current crate's manifest directory for an `.env` file
    /// and loads it into the environment.
    pub fn load()
    {
        let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

        let Some(env_path) = manifest_dir
            .ancestors()
            .map(|dir| dir.join(".env"))
            .find(|path| path.is_file())
        else {
            return;
        };

        dotenvy::from_path(env_path).ok();
    }

    /// Retrieves a required environment variable.
    /// If the variable is missing, it logs an error and terminates startup immediately.
    pub fn get_env(key: &str) -> String
    {
        match var(key) {
            Ok(val) => val,
            Err(_) => {
                eprintln!(
                    "\n[FATAL CONFIG ERROR] Environment variable `{}` is missing, but required to start.\n",
                    key
                );
                process::exit(1);
            }
        }
    }
}
