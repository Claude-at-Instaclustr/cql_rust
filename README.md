# Initial work using `antlr` generated CQL parser

To execute the code in this repository you must use the `nightly` version of Rust.

To set this up execute: 

```
rustup install nightly
rustup default nightly
```
When you want to revert to using rust `stable`, you can run:
```
rustup default stable
```
## Usage examples

Usage examples can be found at: https://github.com/rrevenantt/antlr4rust

## cql parser source generation.

To generate the source antlr jar that has the Rust extension and CQL3 grammar from antlr grammars-v4 project are required.

```
wget https://github.com/rrevenantt/antlr4rust/releases/download/antlr4-4.8-2-Rust-0.2/antlr4-4.8-2-SNAPSHOT-complete.jar 
git clone https://github.com/Claude-at-Instaclustr/cql_rust.git
git clone https://github.com/antlr/grammars-v4.git
java -jar antlr4-4.8-2-SNAPSHOT-complete.jar \
    -listener -visitor -Dlanguage=Rust \
    -Xexact-output-dir -o cql_rust/src \
    grammars-v4/cql3/*.g4

```

## External components

### https://github.com/antlr/grammars-v4.git

Contains the grammar source files for antlr.  This is the source of the CQL3 definition.

### https://github.com/rrevenantt/antlr4rust

A custom implementation of the [Antlr4](https://github.com/antlr/antlr4) tool that builds the `Rust` source code.  This component imposes the `nightly` requirement on the `Rust` system. 

The documentation for Antlr4Rust states:

```
Currently, requires nightly version of rust. This likely will be 
the case until coerce_unsized or some kind of coercion trait is 
stabilized. There are other unstable features in use but only 
CoerceUnsized is essential.
```

