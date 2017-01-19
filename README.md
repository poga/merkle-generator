# Merkle Generator

Generator a merkle tree based on incoming data. Based on [mafintosh/merkle-tree-stream](https://github.com/mafintosh/merkle-tree-stream).

## Usage

```rust
extern crate merkle_generator;

// define how to hash incoming data
fn parent(a: &Node, b: &Node) -> Vec<u8> {
    let mut data = a.data.clone().unwrap();
    data.append(&mut b.data.clone().unwrap());

    digest::digest(&digest::SHA256, data.as_slice())
        .as_ref()
        .to_vec()
}

// define how to hash two merkle tree node hashes into a new parent hash
fn leaf(leaf: &Node, roots: &Vec<Node>) -> Vec<u8> {
    let data = leaf.data.clone().unwrap();
    digest::digest(&digest::SHA256, data.as_slice()).as_ref().to_vec()
}

let mut gen = Generator::new(leaf, parent);

let nodes = gen.next(b"Hello World".to_vec());
println!("{:?}", nodes);
```

## Tree Structure

See [mafintosh/flat-tree](https://github.com/mafintosh/flat-tree-rs) for more information about how node/parent indexes are calculated.

## License

The MIT License
