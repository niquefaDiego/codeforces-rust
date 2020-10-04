fn read_line<R: std::io::BufRead>(reader: &mut R) -> String {
  let mut line = String::new();
  reader.read_line(&mut line).unwrap();
  line
}

fn main()
{
  let stdin = std::io::stdin();
  let mut reader = std::io::BufReader::new(stdin.lock());
  let ntc: i32 = read_line(&mut reader).trim().parse().unwrap();
  for _ in 0..ntc {
    let arr: Vec<i32> = read_line(&mut reader).split_whitespace().map(|x| x.parse().unwrap()).collect();
    let max = arr.iter().max().unwrap();
    println!("{}", max);
  }
}
