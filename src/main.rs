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
}
