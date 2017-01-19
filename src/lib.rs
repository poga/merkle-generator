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
    fn gen1() {
        let mut gen = Generator::new(leaf, parent);
        let nodes = gen.next(b"Hello World".to_vec());
        assert_eq!(nodes[0].index, 0);
        assert_eq!(nodes[0].parent, 1);
        let data = nodes[0].data.clone().unwrap();
        assert_eq!(data, b"Hello World");
        let hash = digest::digest(&digest::SHA256, b"Hello World").as_ref().to_vec();
        assert_eq!(nodes[0].hash, hash);
    }

    #[test]
    fn gen2() {
        let mut gen = Generator::new(leaf, parent);
        let nodes = gen.next(b"Hello".to_vec());
        let n1 = nodes[0].clone();
        assert_eq!(nodes[0].index, 0);
        assert_eq!(nodes[0].parent, 1);
        let data = nodes[0].data.clone().unwrap();
        assert_eq!(data, b"Hello");
        let hash = digest::digest(&digest::SHA256, b"Hello").as_ref().to_vec();
        assert_eq!(nodes[0].hash, hash);

        let nodes = gen.next(b"World".to_vec());
        let n2 = nodes[0].clone();
        assert_eq!(nodes[0].index, 2);
        assert_eq!(nodes[0].parent, 1);
        let data = nodes[0].data.clone().unwrap();
        assert_eq!(data, b"World");
        let hash = digest::digest(&digest::SHA256, b"World").as_ref().to_vec();
        assert_eq!(nodes[0].hash, hash);

        assert_eq!(nodes[1].index, 1);
        assert_eq!(nodes[1].parent, 3);
        assert!(nodes[1].data.is_none());
        let hash = parent(&n1, &n2);
        assert_eq!(nodes[1].hash, hash);
    }

    fn parent(a: &Node, b: &Node) -> Vec<u8> {
        let ref mut hash = a.hash.clone();
        hash.extend(b.hash.iter().cloned());

        digest::digest(&digest::SHA256, hash.as_slice())
            .as_ref()
            .to_vec()
    }

    fn leaf(leaf: &Node, roots: &Vec<Node>) -> Vec<u8> {
        let data = leaf.data.clone().unwrap();
        digest::digest(&digest::SHA256, data.as_slice()).as_ref().to_vec()
    }
}
