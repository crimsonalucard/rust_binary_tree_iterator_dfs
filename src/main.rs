use std::fmt::Display;

struct Node<Item: Default + Display> {
    item: Item,
    children: Vec<Node<Item>>,
}

impl<Item: Display + Default> Node<Item> {
    fn traverse(&self, f: &impl Fn(&Item)) {
        self.children.iter().for_each(move |node| {
            node.traverse(f);
        });
        f(&self.item);
    }

    fn iter(&self) -> NodeIter<Item> {
        NodeIter {
            children: self.children.as_slice(),
            value: Some(&self.item),
        }
    }
}

struct NodeIter<'a, Item: Display + Default> {
    children: &'a [Node<Item>],
    value: Option<&'a Item>,
}

impl<'a, Item: Display + Default> Iterator for NodeIter<'a, Item> {
    type Item = &'a Item;
    fn next(&mut self) -> Option<Self::Item> {
        if self.children.len() == 0 {
            self.value.take()
        } else {
            let next_node = self.children.first().unwrap();
            self.children = &self.children[1..];
            NodeIter{
                value: Some(&next_node.item),
                children: &next_node.children
            }.next()
        }
    }
}

fn main() {
    let x = Node{
        item: 1,
        children: vec![Node{
            item: 2,
            children: vec![]
        }, Node {
            item: 3,
            children: vec![]
        }]
    };
    x.traverse(&|x:&i32 | {
        println!("{}", x);
    });
    println!("test iterator");
    x.iter().for_each(|x|{
        println!("{}", x);
    });
}
