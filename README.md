# Criando um demo em rust usando Sqlite


## Cargo
```zsh
# cargo
apt upgrade cargo

cargo new --lib diesel_demo
cd diesel_demo

cargo install cargo-watch

cargo test
cargo check
cargo watch -c -x run

# Available binaries:
    publish_post
    show_posts
    write_post

cargo run --bin write_post
  teste_1
  Dig conteudo
cargo run --bin publish_post 1
cargo run --bin show_posts
cargo run --bin delete_post teste_1
```

## Install Diesel
```zsh
# instalar o diesel (https://lib.rs/install/diesel)
# usando asdf a PATH de instalação sera em:
# ~/.asdf/installs/rust/1.46.0/bin/diesel

cargo install cargo-edit
cargo add diesel

cargo rm diesel
cargo rm diesel --dev
cargo rm diesel --build

cargo upgrade
cargo upgrade regex --workspace

#ou em Cargo.toml no seu projeto, adicione à seção [dependencies]:
[dependencies]
diesel = "1.4.5"
```


## up.sql
```sql
CREATE TABLE "posts" (
  "id"    INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  "title" TEXT NOT NULL,
  "body"  TEXT NOT NULL,
  "published" INTEGER NOT NULL
);
```


## Execute modulos
```zsh

cargo run --bin publish_post 1
cargo run --bin show_posts
```


## starship - prompt conf 
```yaml
vim ~/.config/starship.toml
  # https://starship.rs

  add_newline = false
  #prompt_order=["rust","line_break","package","line_break","character"]
  #scan_timeout = 10

  [package]
  disabled = false

  [username]
  disabled = true

  [character]
  symbol = "~^~\"
  error_symbol = "~\~W"
  use_symbol_for_status = true

  [docker_context]
  symbol = "~_~P~K "
  only_with_files = true

  [dotnet]
  symbol = "~_~E "
  style = "green"
  heuristic = false

  [python]
  symbol = "~_~Q "
  pyenv_version_name = true
  pyenv_prefix = "foo "
```


# Erros
```zsh
# error: no override and no default toolchain set
  # fix erro:
  rustup default stable
  # ou para nightly rust:
  rustup default nightly
```