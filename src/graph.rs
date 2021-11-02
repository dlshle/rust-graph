use std::collections::hash_map::Iter;
use std::collections::HashMap;
use std::collections::HashSet;

pub trait Comparable {
  fn compare(&self, other: &Self) -> i8;
}

pub trait IGraphVisitor<T: Comparable, R> {
  fn visit(&mut self, graph: &mut IGraph<T>) -> R;
}

pub struct TopoSortVisitor {
  permVisited: HashSet<i32>,
  tempVisited: HashSet<i32>,
  result: Vec<i32>,
}

impl TopoSortVisitor {
  pub fn new() -> TopoSortVisitor {
    return TopoSortVisitor {
      permVisited: HashSet::<i32>::new(),
      tempVisited: HashSet::<i32>::new(),
      result: Vec::<i32>::new(),
    };
  }

  fn topo<T: Comparable>(&mut self, graph: &mut IGraph<T>, v: i32) {
      if self.permVisited.contains(&v) {
        // ok
        return;
      }
      if !self.tempVisited.insert(v) {
        // no good, cycle detected!
        return;
      }
      for neighbor in graph.get_out_neighbours(v).iter() {
        self.topo(graph, *neighbor);
      }
      self.tempVisited.remove(&v);
      self.permVisited.insert(v);
      self.result.insert(0, v);
      return;
  }
}

impl <T: Comparable> IGraphVisitor<T, Vec<i32>> for TopoSortVisitor {
  fn visit(&mut self, graph: &mut IGraph<T>) -> Vec<i32> {
    self.permVisited.clear();
    self.tempVisited.clear();
    self.result.clear();
    for vertex in graph.vertices() {
      self.topo(graph, vertex);
    }
    let mut result = Vec::<i32>::new();
    for v in &self.result {
      result.push(*v);
    }
    return result;
  }
}

// -----------------------------

pub struct Graph<T: Comparable> {
  vals: HashMap<i32, T>,
  inwards: HashMap<i32, HashSet<i32>>,
  outwards: HashMap<i32, HashSet<i32>>,
  counter: i32,
}

pub trait IGraph<T: Comparable> {
  fn add(&mut self, val: T) -> i32;
  fn get(&mut self, vertex: i32) -> Option<&T>;
  fn get_vector(&mut self, v: &T) -> i32;
  fn bulk_get(&mut self, vertices: Vec<i32>) -> Vec<Option<&T>>;
  fn link(&mut self, u: i32, v: i32) -> bool;
  fn unlink(&mut self, u: i32, v: i32) -> bool;
  fn get_out_neighbours(&mut self, vec: i32) -> Vec<i32>;
  fn clear(&mut self);
  fn iter(&mut self) -> Iter<'_, i32, T>;
  fn vertices(&mut self) -> Vec<i32>;
}

impl <T: Comparable> Graph<T> {
  pub fn new() -> Graph<T> {
    return Graph{
      vals: HashMap::new(),
      inwards: HashMap::new(),
      outwards: HashMap::new(),
      counter: 0,
    };
  }

  pub fn accept<R>(&mut self, v: &mut IGraphVisitor<T, R>) -> R {
    return v.visit(self);
  }

  fn _is_key_exist(&self, k: i32) -> bool {
    if k >= self.counter {
      return false;
    }
    return self.vals.contains_key(&k);
  }

  fn _add_outward_link(&mut self, u: i32, v: i32) -> bool {
    return self.outwards.entry(u).or_insert(HashSet::new()).insert(v);
  }

  fn _add_inward_link(&mut self, v: i32, u: i32) -> bool {
    return self.inwards.entry(v).or_insert(HashSet::new()).insert(u);
  }

  fn _remove_outward_link(&mut self, u: i32, v: i32) -> bool {
    return self.outwards.entry(u).or_insert(HashSet::new()).remove(&v);
  }

  fn _remove_inward_link(&mut self, v: i32, u: i32) -> bool {
    return self.outwards.entry(v).or_insert(HashSet::new()).remove(&u);
  }

  fn _get_outwards_vertices(&mut self, u: i32) -> Vec<i32> {
    let mut vertices = Vec::new();
    for v in self.outwards.entry(u).or_insert(HashSet::new()).iter() {
      vertices.push(*v);
    }
    return vertices;
  }
}

impl <T: Comparable> IGraph<T> for Graph<T> {
  fn add(&mut self, val: T) -> i32 {
    let v = self.counter;
    self.vals.insert(self.counter, val);
    self.counter += 1;
    return v;
  }

  fn get(&mut self, vertex: i32) -> Option<&T> {
    return self.vals.get(&vertex);
  }

  fn get_vector(&mut self, val: &T) -> i32 {
    for (k, v) in self.vals.iter() {
      if v.compare(val) == 0 {
        return *k;
      }
    }
    return -1;
  }


  fn bulk_get(&mut self, vertices: Vec<i32>) -> Vec<Option<&T>> {
    let mut result = Vec::<Option<&T>>::new();
    for v in self.vertices() {
      result.push(self.vals.get(&v));
    }
    return result;
  }

  fn get_out_neighbours(&mut self, u: i32) -> Vec<i32> {
    return self._get_outwards_vertices(u);
  }

  fn iter(&mut self) -> Iter<'_, i32, T> {
    return self.vals.iter();
  }

  fn vertices(&mut self) -> Vec<i32> {
    let mut vector = Vec::<i32>::new();
    for (vertex, _) in self.iter() {
      vector.push(*vertex);
    }
    return vector
  }

  fn link(&mut self, u: i32, v: i32) -> bool {
    if !(self._is_key_exist(u) && self._is_key_exist(v)) {
      return false;
    }
    return self._add_outward_link(u, v) && self._add_inward_link(v, u);
  }
  
  fn unlink(&mut self, u: i32, v: i32) -> bool {
    if !(self._is_key_exist(u) && self._is_key_exist(v)) {
      return false;
    }
    return self._remove_outward_link(u, v) && self._remove_inward_link(v, u);
  }

  fn clear(&mut self) {
    self.vals.clear();
    self.inwards.clear();
    self.outwards.clear();
  }
}

// -----------------------------

impl Comparable for i32 {
  fn compare(&self, other: &i32) -> i8 {
    if *self > *other {
      return 1;
    } else if *self == *other {
      return 0;
    } else {
      return -1;
    }
  }
}

pub fn print_line(v: i32) {
  println!("{}", v);
}

pub fn print_vec(v: &Vec<i32>) {
  println!("{:?} at {:p}", v, &v);
}

pub fn add_one(v: &mut i32) {
  *v+=1;
}


