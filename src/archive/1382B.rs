fn main() {
  let stdin = std::io::stdin();
  let mut line = String::new();

  stdin.read_line(&mut line).unwrap();
  let ntc: i32 = line.trim().parse().unwrap();
  for _ in 0..ntc {
    stdin.read_line(&mut line).unwrap();
    line.clear();
    stdin.read_line(&mut line).unwrap();
    let a: Vec<i32> = line.split_whitespace().map(|x| x.parse().expect("parse error")).collect();
    let mut i = 0;
    while i < a.len() && a[i] == 1 {
      i += 1;
    }
    let ans: bool;
    if i == a.len() {
      ans = i % 2 == 1;
    } else {
      ans = i % 2 == 0;
    }
    println!("{}", if ans { "First" } else { "Second"});
  }
}
