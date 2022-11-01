use dotenv::dotenv;
use dotenv_config_ext::EnvConfig;

async fn ext_dummy(s: String) -> Result<String, ()>{
    Ok(s)
}

#[derive(Debug, EnvConfig)]
struct Config {
    #[env_config(default = "192.168.2.1", ext="true", ext_post_with="ext_dummy")]
    server_addr: String,
    server_mode: bool,
    #[env_config(name = "ZINC_FOO", default = true, ext="false")]
    foo: bool,
    #[env_config(name = "ZINC_BAR", default = 123456, ext="false")]
    bar: Option<i64>,
    rr: Redis,
}

#[derive(Debug, EnvConfig)]
struct Redis {
    #[env_config(name = "ZINC_BAR", default = 127.0.0.1, ext="true", ext_post_with="ext_dummy")]
    addr: String,
    port: String,
    auth: String,
    #[env_config(name = "ZINC_REDIS_TIMEOUT", default = 30)]
    timeout: i32,
}

macro_rules! aw {
    ($e:expr) => {
        tokio_test::block_on($e)
    };
  }

#[test]
fn test_config() {
    dotenv().ok();
    let cfg = aw!(Config::init()).unwrap();
    assert!(cfg.server_addr == "192.168.2.1");
    assert!(cfg.rr.timeout == 30i32);
}
