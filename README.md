# Lua cache

Este repositório contém um projeto escrito em Rust com o objetivo de implementar sistemas de cache. Foi criado como parte de um esforço para aprender mais sobre a linguagem de programação Rust e suas aplicações.

## Sobre o Projeto

O projeto é uma coletânea de sistemas de cache implementados de uma forma simples.

Aqui está um exemplo de como o código pode ser usado para construir um storage baseado em File (sistema de arquivo):

```rust
mod cache;

use cache::memory::MemoryStorage;
use std::time::Duration;

fn main() {
    let mut cache = MemoryStorage::new(Duration::from_secs(1));
    cache.add("car", "white");

    loop {
        println!("{}", cache.get("car")); // car invalidates after 1 sec
    }
}
```
