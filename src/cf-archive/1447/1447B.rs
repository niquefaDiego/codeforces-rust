struct Reader<R: std::io::BufRead> {
  buf_reader: R
}

impl<R: std::io::BufRead> Reader<R> {
  fn new(buf_reader: R) -> Reader<R> {
    Reader {
      buf_reader
    }
  }

  fn line(&mut self) -> String {
    let mut line = String::new();
    self.buf_reader.read_line(&mut line).unwrap();
    line
  }

  fn value<T: std::str::FromStr>(&mut self) -> T {
    self.line().trim().parse::<T>().ok().unwrap()
  }

  fn vec<T: std::str::FromStr>(&mut self) -> Vec<T> {
    self.line().split_whitespace().map(|x| x.parse().ok().unwrap()).collect()
  }
}

fn main()
{
  let stdin = std::io::stdin();
  let mut input = Reader::new(std::io::BufReader::new(stdin.lock()));
  let ntc: i32 = input.value();
  for _ in 0..ntc {
    let dim: Vec<usize> = input.vec();
    let n = dim[0];
    let m = dim[1];
    let mut min_value = std::i32::MAX;
    let mut sum = 0;
    let mut neg_count = 0;
    for i in 0..n {
      let values: Vec<i32> = input.vec();
      for j in 0..m {
        sum += values[j].abs();
        min_value = std::cmp::min(min_value, values[j].abs());
        neg_count += if values[j] < 0 { 1 } else { 0 };
      }
    }
    println!("{}", sum - (neg_count%2) * min_value * 2);
  }
}
