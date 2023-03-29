### Python vs Rust Ecosystem

|                 |   Python                 | Rust                       |  
|-----------------|--------------------------|----------------------------|
|Linter           |   pyliny, pep8           | cargo clippy               |
|LSP              |   python-language-server | RLS, rust_analyzer         |
|Code formatting  |   yapf,autopep8          | cargo fmt                  |
|build binary     |   setuptools,py2exe      | cargo build                |
|test             |   unittest, pytest       | cargo test                 |
|build            |   virtualenv, pip        | cargo new, cargo update    |
|environment      |   requirement.txt        | Cargo.toml                 |
|documentation    |   Sphinx based, doxygen  | cargo doc                  |
|benchmark        |   cProfile               | cargo bench, criterion.rs  |

#### Rust commands

##### Create a library project
```cargo new --lib mylib```

##### Create a binary project 
```cargo new mycmd```

##### Rust documentation

```

//! # most amazing geometric object library ever
//! This is starting off like clockwork
//! - bullet
//! - points

/// for surface area calculation of a geometric object
pub trait Area {
    fn area(&self) -> f64;
}

/// for Perimeter/circumferen calculation of a geometric object
pub trait Perimeter {
    fn circumference(&self) -> f64;
}

```
