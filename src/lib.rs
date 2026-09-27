use std::env::var;
use std::path::PathBuf;
use std::process;

pub struct NearestEnv;

impl NearestEnv
{
    /// Searches upward from the current working directory for an `.env` file
    /// and loads it into the environment.
    pub fn load()
    {
        // Use the current runtime directory of the executing binary,
        // rather than the compile-time library directory.
        let current_dir = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));

        let Some(env_path) = current_dir
            .ancestors()
            .map(|dir| dir.join(".env"))
            .find(|path| path.is_file())
        else {
            return;
        };

        dotenvy::from_path(env_path).ok();
    }

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
