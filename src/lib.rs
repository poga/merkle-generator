extern crate flat_tree;

#[derive(Clone, Debug)]
pub struct Node {
    index: u64,
    parent: u64,
    hash: Vec<u8>,
    data: Option<Vec<u8>>,
    size: usize,
}

pub struct Generator {
    roots: Vec<Node>,
    blocks: u64,
    leaf: fn(&Node, &Vec<Node>) -> Vec<u8>,
    parent: fn(&Node, &Node) -> Vec<u8>,
}

impl Generator {
    pub fn new(leaf: fn(&Node, &Vec<Node>) -> Vec<u8>,
               parent: fn(&Node, &Node) -> Vec<u8>)
               -> Generator {
        Generator {
            roots: vec![],
            blocks: 0,
            leaf: leaf,
            parent: parent,
        }
    }

    pub fn next(&mut self, data: Vec<u8>) -> Vec<Node> {
        let mut nodes = vec![];
        let index = 2 * self.blocks;
        self.blocks += 1;
        let len = data.len();
        let mut leaf = Node {
            index: index,
            parent: flat_tree::parent(index),
            hash: vec![],
            data: Some(data),
            size: len,
        };
        let hash = (self.leaf)(&leaf, &self.roots);
        leaf.hash = hash;
        self.roots.push(leaf.clone());
        nodes.push(leaf.clone());

        while self.roots.len() > 1 {
            let ref left = self.roots[self.roots.len() - 2].clone();
            let ref right = self.roots[self.roots.len() - 1].clone();

            if left.parent != right.parent {
                break;
            }

            self.roots.pop();
            let leaf = Node {
                index: left.parent,
                parent: flat_tree::parent(left.parent),
                hash: (self.parent)(left, right),
                size: left.size + right.size,
                data: None,
            };
            let len = self.roots.len();
            self.roots[len - 1] = leaf.clone();
            nodes.push(leaf.clone());
        }

        nodes
    }
}

extern crate ring;

#[cfg(test)]
mod tests {
    use ring::digest;
    use super::{Generator, Node};

    #[test]
    fn it_works() {
        let mut gen = Generator::new(leaf, parent);
        let nodes = gen.next(b"Hello World".to_vec());
        assert!(nodes[0].index == 0);
        assert!(nodes[0].parent == 1);
        let data = nodes[0].data.clone().unwrap();
        assert!(data == b"Hello World");
        let hash = digest::digest(&digest::SHA256, b"Hello World").as_ref().to_vec();
        assert!(nodes[0].hash == hash);
    }

    fn parent(a: &Node, b: &Node) -> Vec<u8> {
        let mut data = a.data.clone().unwrap();
        data.append(&mut b.data.clone().unwrap());

        digest::digest(&digest::SHA256, data.as_slice())
            .as_ref()
            .to_vec()
    }

    fn leaf(leaf: &Node, roots: &Vec<Node>) -> Vec<u8> {
        let data = leaf.data.clone().unwrap();
        digest::digest(&digest::SHA256, data.as_slice()).as_ref().to_vec()
    }
}
