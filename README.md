# Forked from
[zinclabs/dotenv-config](https://github.com/zinclabs/dotenv-config/)

# Dot Env Config

use `.env` as config file and parse environments to config struct.

## Usage

### derive EnvConfig

```rust
use dotenv::dotenv;
use dotenv_config::EnvConfig;

#[derive(Debug, EnvConfig)]
struct Config {
    #[env_config(default = "192.168.2.1")]
    server_addr: String,
    server_mode: bool,
    #[env_config(name = "ZINC_FOO", default = true)]
    foo: bool,
    #[env_config(name = "ZINC_BAR", default = 123456)]
    bar: Option<i64>,
}

fn main() {
    dotenv().ok();
    let cfg = Config::init().unwrap();
    println!("{:#?}", cfg);
}
```

### attribute env_config

you can use macro attribute set field attribute 

- name: change default environment key
- default: if not set, used as default value

## you can though system environments or `.env` file config it.

```
ZINC_FOO=false
ZINC_BAR=8787878
```

default load environment key is: `structName_fieldName` do UpperSnake, like above struct, default config key is:

```
CONFIG_SERVER_ADDR
CONFIG_SERVER_MODE
ZINC_FOO
ZINC_BAR
```

## Added feature
Allows the usage of external function to post-process the field value once it get parsed.

For example we can pass an environment variable with an aws ARN for a secret manager, so once the value of the arn 
has been parsed, the added function specified in `ext_post_with` will retrieve the value from aws.

### Usage

```rust
use dotenv::dotenv;
use dotenv_config::EnvConfig;

fn ssm_client(s: String) -> Result<String, ()>{
    Ok(s)
}

#[derive(Debug, EnvConfig)]
struct Config {
    #[env_config(default = "192.168.2.1", ext=true, ext_post_with="ssm_client")]
    server_addr: String,
    server_mode: bool,
    #[env_config(name = "ZINC_FOO", default = true)]
    foo: bool,
    #[env_config(name = "ZINC_BAR", default = 123456)]
    bar: Option<i64>,
}
#[tokio::main]
async fn main() {
    dotenv().ok();
    let cfg = Config::init().await.unwrap();
    println!("{:#?}", cfg);
}
```

### Attributes

`ext: bool` enable or disable feature.

`ext_post_with: String` required if `ext` is true.

The attribute `ext_post_with` receive a function name as a string, the function passed must have the following signature:

```rust
async fn func(_: String) -> Result<String, E>;
```