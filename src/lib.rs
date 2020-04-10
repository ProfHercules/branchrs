pub use self::branchrs::Node;

pub mod branchrs {
  pub trait Weight: ops::AddAssign + Eq + PartialEq + Ord + PartialOrd + Copy {}
  impl<W> Weight for W where W: ops::AddAssign + Eq + PartialEq + Ord + PartialOrd + Copy {}

  pub trait DataType: std::hash::Hash + Eq + PartialEq + Copy {}
  impl<T> DataType for T where T: std::hash::Hash + Eq + PartialEq + Copy {}

  type Link<W, T> = Option<Box<Node<W, T>>>;

  use std::cmp::Ordering;
  use std::collections::HashMap;
  use std::ops;

  /// W (Weight) needs to be a numeric value that can be used in a
  /// `BinaryHeap`, T is the type of the data stored by the node (inside its `Option<T>`)
  #[derive(Debug)]
  pub struct Node<W: Weight, T: DataType> {
    weight: W,
    pub data: Option<T>,
    left: Link<W, T>,
    right: Link<W, T>,
  }

  impl<W: Weight, T: DataType> Node<W, T> {
    pub fn new(weight: W, data: Option<T>) -> Node<W, T> {
      Node {
        weight,
        data,
        left: None,
        right: None,
      }
    }

    pub fn get_weight(&self) -> W {
      self.weight
    }

    pub fn link_left(&mut self, link: Node<W, T>) {
      self.weight += link.weight;
      // self.value += node.value;
      self.left = Some(Box::new(link));
    }

    pub fn get_left(&self) -> Option<&Node<W, T>> {
      if let Some(b) = &self.left {
        return Some(b.as_ref());
      }
      None
    }

    pub fn link_right(&mut self, link: Node<W, T>) {
      self.weight += link.weight;
      // self.value += node.value;
      self.right = Some(Box::new(link));
    }

    pub fn get_right(&self) -> Option<&Node<W, T>> {
      if let Some(b) = &self.right {
        return Some(b.as_ref());
      }
      None
    }

    pub fn walk(&self, path: String, map: &mut HashMap<T, String>) {
      //let (left, right) = ("0", "1");
      if let Some(data) = self.data {
        if path == "" {
          map.insert(data, String::from("0"));
        } else {
          map.insert(data, path.clone());
        }
      };

      if let Some(left_node) = self.get_left() {
        left_node.walk(path.clone() + "0", map);
      };
      if let Some(right_node) = self.get_right() {
        right_node.walk(path.clone() + "1", map);
      };
    }

    pub fn find_path(node: &Node<W, T>, find: &T, path_char: (&str, &str)) -> Option<String>
    where
      T: std::cmp::PartialEq + Copy,
    {
      find_path_recurse(node, find, path_char, String::new())
    }
  }

  fn find_path_recurse<W, T>(
    node: &Node<W, T>,
    find: &T,
    path_char: (&str, &str),
    path: String,
  ) -> Option<String>
  where
    W: Weight,
    T: DataType,
  {
    let left = path_char.0;
    let right = path_char.1;

    if let Some(data) = node.data {
      if data == *find {
        if path == "" {
          return Some(left.to_string());
        }
        return Some(path);
      }
      return None;
    }

    if let Some(left_node) = node.get_left() {
      let left_path = find_path_recurse(left_node, find, path_char, path.clone() + left);
      if let Some(p) = left_path {
        return Some(p);
      }
    }

    if let Some(right_node) = node.get_right() {
      let right_path = find_path_recurse(right_node, find, path_char, path.clone() + right);
      if let Some(p) = right_path {
        return Some(p);
      }
    }

    None
  }

  impl<W: Weight, T: DataType> Eq for Node<W, T> {}
  impl<W: Weight, T: DataType> PartialEq for Node<W, T> {
    fn eq(&self, other: &Self) -> bool {
      self.weight == other.weight
    }
  }

  impl<W: Weight, T: DataType> Ord for Node<W, T> {
    fn cmp(&self, other: &Node<W, T>) -> Ordering {
      (&other.weight).cmp(&self.weight)
    }
  }

  impl<W: Weight, T: DataType> PartialOrd for Node<W, T> {
    fn partial_cmp(&self, other: &Node<W, T>) -> Option<Ordering> {
      Some(self.cmp(other))
    }
  }

  #[cfg(test)]
  mod test {
    use super::*;

    #[test]
    fn test_get_weight() {
      let root_node = Node::new(10, Some('a'));
      assert_eq!(root_node.get_weight(), 10);
    }

    #[test]
    fn test_node_creation() {
      let root_node = Node::new(10, Some('c'));

      assert_eq!(root_node.get_weight(), 10);
      assert_eq!(root_node.data, Some('c'));
    }

    #[test]
    fn test_link_left() {
      let mut root_node = Node::new(10, Some('a'));
      let left_node = Node::new(20, Some('b'));

      root_node.link_left(left_node);

      let left_weight = root_node.get_left().unwrap().get_weight();
      let root_weight = root_node.get_weight();

      let left_data = root_node.get_left().unwrap().data;

      assert_eq!(left_weight, 20);
      assert_eq!(root_weight, 30);
      assert_eq!(left_data, Some('b'));
    }

    #[test]
    fn test_link_right() {
      let mut root_node = Node::new(40, Some('c'));
      let right_node = Node::new(50, Some('d'));

      root_node.link_right(right_node);

      let right_weight = root_node.get_right().unwrap().get_weight();
      let root_weight = root_node.get_weight();

      let right_data = root_node.get_right().unwrap().data;

      assert_eq!(right_weight, 50);
      assert_eq!(root_weight, 90);
      assert_eq!(right_data, Some('d'));
    }

    #[test]
    fn test_find_path() {
      let mut root_node = Node::new(0, None);
      let mut left = Node::new(1, None);
      let mut right = Node::new(2, None);

      let left_child1 = Node::new(3, Some('a'));
      let left_child2 = Node::new(4, Some('b'));

      let right_child1 = Node::new(5, Some('c'));
      let right_child2 = Node::new(6, Some('d'));

      left.link_left(left_child1);
      left.link_right(left_child2);

      right.link_left(right_child1);
      right.link_right(right_child2);

      root_node.link_left(left);
      root_node.link_right(right);

      /*
      The node structure should now look like this.
             0+21
          /       \
        1+7        2+11
       /   \      /    \
      3:a  4:b  5:c   6:d
      */

      assert_eq!(root_node.get_weight(), 21);

      let left = root_node.get_left().unwrap();
      let right = root_node.get_right().unwrap();
      assert_eq!(left.get_weight(), 8);
      assert_eq!(right.get_weight(), 13);

      let left_c1 = left.get_left().unwrap();
      let left_c2 = left.get_right().unwrap();
      let right_c1 = right.get_left().unwrap();
      let right_c2 = right.get_right().unwrap();
      assert_eq!(left_c1.get_weight(), 3);
      assert_eq!(left_c2.get_weight(), 4);
      assert_eq!(right_c1.get_weight(), 5);
      assert_eq!(right_c2.get_weight(), 6);

      let path = Node::find_path(&root_node, &'a', ("0", "1")).unwrap();
      assert_eq!(path, "00");

      let path = Node::find_path(&root_node, &'b', ("0", "1")).unwrap();
      assert_eq!(path, "01");

      let path = Node::find_path(&root_node, &'c', ("0", "1")).unwrap();
      assert_eq!(path, "10");

      let path = Node::find_path(&root_node, &'d', ("0", "1")).unwrap();
      assert_eq!(path, "11");

      let no_path = Node::find_path(&root_node, &'e', ("0", "1"));
      assert_eq!(no_path, None);
    }

    #[test]
    fn test_walk() {
      let mut root_node = Node::new(0, None);
      let mut left = Node::new(1, None);
      let mut right = Node::new(2, None);

      let left_child1 = Node::new(3, Some('a'));
      let left_child2 = Node::new(4, Some('b'));

      let right_child1 = Node::new(5, Some('c'));
      let right_child2 = Node::new(6, Some('d'));

      left.link_left(left_child1);
      left.link_right(left_child2);

      right.link_left(right_child1);
      right.link_right(right_child2);

      root_node.link_left(left);
      root_node.link_right(right);

      /*
      The node structure should now look like this.
             0+21
          /       \
        1+7        2+11
       /   \      /    \
      3:a  4:b  5:c   6:d
      */

      let mut key_map: HashMap<char, String> = HashMap::new();

      root_node.walk(String::from(""), &mut key_map);

      assert_eq!(key_map[&'a'], "00");
      assert_eq!(key_map[&'b'], "01");
      assert_eq!(key_map[&'c'], "10");
      assert_eq!(key_map[&'d'], "11");
    }
  }
}
