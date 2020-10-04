fn main()
{
  let stdin = std::io::stdin();
  let mut line = String::new();
  stdin.read_line(&mut line).unwrap();
  let ntc: i32 = line.trim().parse().unwrap();
  for _ in 0..ntc {
    line.clear();
    stdin.read_line(&mut line).unwrap();
    let arr: Vec<i32> = line.trim().split_whitespace().map(|x| x.parse().unwrap()).collect();
    let max = match arr.iter().max() {
      Some(x) => *x,
      None => -1
    };
    println!("{}", max);
  }
}
