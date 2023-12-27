use azure_core::error::{ErrorKind, ResultExt};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum EnvEnum {
    Process(ProcessEnv),
    Mem(MemEnv),
}

impl Default for EnvEnum {
    fn default() -> Self {
        Self::Process(ProcessEnv)
    }
}

impl EnvEnum {
    pub fn var(&self, key: &str) -> azure_core::Result<String> {
        match self {
            EnvEnum::Process(env) => env.var(key),
            EnvEnum::Mem(env) => env.var(key),
        }
    }
}

impl From<ProcessEnv> for EnvEnum {
    fn from(env: ProcessEnv) -> Self {
        Self::Process(env)
    }
}

impl From<MemEnv> for EnvEnum {
    fn from(env: MemEnv) -> Self {
        Self::Mem(env)
    }
}

/// The standard environment that gets variables from the process.
#[derive(Debug, Clone, Default)]
pub struct ProcessEnv;

impl ProcessEnv {
    fn var(&self, key: &str) -> azure_core::Result<String> {
        std::env::var(key).map_kind(ErrorKind::Io)
    }
}

/// An environment that stores and gets variables in memory.
#[derive(Debug, Clone, Default)]
pub struct MemEnv {
    vars: HashMap<String, String>,
}

impl From<HashMap<String, String>> for MemEnv {
    fn from(vars: HashMap<String, String>) -> Self {
        Self { vars }
    }
}

impl From<&[(&str, &str)]> for MemEnv {
    fn from(pairs: &[(&str, &str)]) -> Self {
        let mut vars = HashMap::new();
        for (k, v) in pairs {
            vars.insert(k.to_string(), v.to_string());
        }
        Self { vars }
    }
}

impl MemEnv {
    fn var(&self, key: &str) -> azure_core::Result<String> {
        self.vars
            .get(key)
            .cloned()
            .ok_or_else(|| ErrorKind::Io.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_env_var() {
        let env = MemEnv::from(&[("CHRISTMAS_GRINCH", "You're a mean one, Mr. Grinch")][..]);
        assert_eq!(
            env.var("CHRISTMAS_GRINCH").unwrap(),
            "You're a mean one, Mr. Grinch"
        );
    }

    // test ProcessEnv::var() returns an error when the environment variable is not set
    #[test]
    fn test_env_var_not_set() {
        let env = ProcessEnv {};
        assert!(env.var("CHRISTMAS_GRINCH").is_err());
    }

    // test MemEnv::var() returns an error when the environment variable is not set
    #[test]
    fn test_mem_env_var_not_set() {
        let env = MemEnv::default();
        assert!(env.var("CHRISTMAS_GRINCH").is_err());
    }

    // test MemEnv::var() returns valid entries when multiple environment variables are set
    #[test]
    fn test_mem_env_var_multiple() {
        let env = MemEnv::from(
            &[
                ("CHRISTMAS_GRINCH", "You're a mean one, Mr. Grinch"),
                ("CHRISTMAS_TREE", "O Christmas Tree, O Christmas Tree"),
                ("CHRISTMAS_SNOW", "Let it snow, let it snow, let it snow"),
            ][..],
        );
        assert_eq!(
            env.var("CHRISTMAS_GRINCH").unwrap(),
            "You're a mean one, Mr. Grinch"
        );
        assert_eq!(
            env.var("CHRISTMAS_TREE").unwrap(),
            "O Christmas Tree, O Christmas Tree"
        );
        assert_eq!(
            env.var("CHRISTMAS_SNOW").unwrap(),
            "Let it snow, let it snow, let it snow"
        );
    }
}
