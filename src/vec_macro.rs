// I want something that works like this, but the commas do not parse
// macro_rules! triple {
//   ($thing:expr) => {
//     $thing, $thing, $thing
//   };
// }
//
// #[test]
// fn test_triple() {
//   let v = vec![triple!(6)];
//   assert_eq!(v, vec![6, 6, 6]);
// }
//
// According to https://veykril.github.io/tlborm/macros/syntax/expansion.html#expansion,
// a macro may only expand to:
// * an expression
// * a pattern,
// * a type,
// * zero or more items, or
// * zero or more statements.
// I would need it to expand to a series of token trees, which is not acceptable.

// The best we can do is this:

macro_rules! triple_vec {
  ($x:expr) => {
    vec![$x, $x, $x]
  };
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_triple_vec() {
    let v = triple_vec!(6);
    assert_eq!(v, vec![6, 6, 6]);
  }
}
