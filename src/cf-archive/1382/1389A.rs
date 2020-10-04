fn main() {
  let stdin = std::io::stdin();
  let mut line = String::new();
  stdin.read_line(&mut line).unwrap();
  let ntc: i32 = line.trim().parse().unwrap();
  for _ in 0..ntc {
    line.clear();
    stdin.read_line(&mut line).unwrap();
    let a: Vec<i32> = line.split_whitespace().map(|x| x.parse().expect("parse error")).collect();
    if a[0] * 2 <= a[1] {
      println!("{} {}", a[0], a[0]*2);
    } else {
      println!("-1 -1");
    }
  }
}
