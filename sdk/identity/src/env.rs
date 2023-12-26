use azure_core::error::{ErrorKind, ResultExt};

pub(crate) trait Env: Send + Sync + std::fmt::Debug {
    fn var(&self, key: &str) -> azure_core::Result<String>;
}

#[derive(Debug, Clone)]
pub(crate) struct ProcessEnv;

impl ProcessEnv {
    pub(crate) fn new() -> Self {
        Self
    }
}

impl Env for ProcessEnv {
    fn var(&self, key: &str) -> azure_core::Result<String> {
        std::env::var(key).map_kind(ErrorKind::Io)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[derive(Debug, Clone)]
    struct MockEnv {
        vars: HashMap<String, String>,
    }

    impl MockEnv {
        fn new() -> Self {
            Self {
                vars: HashMap::new(),
            }
        }

        fn insert(&mut self, k: String, v: String) {
            self.vars.insert(k, v);
        }
    }

    impl From<&[(&str, &str)]> for MockEnv {
        fn from(pairs: &[(&str, &str)]) -> Self {
            let mut env = MockEnv::new();
            for (k, v) in pairs {
                env.insert(k.to_string(), v.to_string());
            }
            env
        }
    }

    impl Env for MockEnv {
        fn var(&self, key: &str) -> azure_core::Result<String> {
            self.vars
                .get(key)
                .cloned()
                .ok_or_else(|| ErrorKind::Io.into())
        }
    }

    // test CHRISTMAS_GRINCH environment variable return message
    #[test]
    fn test_env_var() {
        let env = MockEnv::from(&[("CHRISTMAS_GRINCH", "You're a mean one, Mr. Grinch")][..]);
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
}
