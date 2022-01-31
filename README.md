# Initial work using `antlr` generated CQL parser

To execute the code in this repository you must use the `nightly` verson of Rust.

To set this up execute: 

```
rustup install nightly
rustup default nightly
```
When you want to revert to using rust `stable`, you can run:
```
rustup default stable
```

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


