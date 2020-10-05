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
    let sizes: Vec<usize> = input.vec();
    let n = sizes[0];
    let m = sizes[1];
    let mut g: Vec<Vec<i32>> = Vec::with_capacity(n);
    for i in 0..n {
      g.push(input.vec());
    }
    let mut ans: i64 = 0;
    for i in 0..(n/2 + n%2) {
      for j in 0..(m/2 + m%2) {
        let mut v: Vec<i32> = Vec::new();
        v.push(g[i][j]);
        v.push(g[i][m-j-1]);
        v.push(g[n-i-1][j]);
        v.push(g[n-i-1][m-j-1]);
        v.sort();
        let median = v[1];

        ans += i32::abs(g[i][j] - median) as i64;
        g[i][j] = median;

        ans += i32::abs(g[i][m-j-1] - median) as i64;
        g[i][m-j-1] = median;

        ans += i32::abs(g[n-i-1][j] - median) as i64;
        g[n-i-1][j] = median;

        ans += i32::abs(g[n-i-1][m-j-1] - median) as i64;
        g[n-i-1][m-j-1] = median;
      }
    }
    println!("{}", ans);
  }
}
