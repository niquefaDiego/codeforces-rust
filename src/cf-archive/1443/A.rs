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

fn gcd(a: i32, b: i32) -> i32 {
  if b == 0 {
    return a;
  }
  gcd(b, a%b)
}

fn main()
{
  let stdin = std::io::stdin();
  let mut input = Reader::new(std::io::BufReader::new(stdin.lock()));
  let ntc: usize = input.value();
  for _ in 0..ntc {
    let n: i32 = input.value();
    let mut arr: Vec<i32> = Vec::new();
    let mut cur = 4 * n - 2;
    loop
    {
      if (arr.len() as i32) == n {
        break;
      }
      let mut ok = true;
      for x in &arr {
        if x % cur == 0 || gcd(*x, cur) == 1 {
          ok = false;
          break;
        }
      }
      if ok {
        arr.push(cur);
      }
      cur -= 2;
    }
    for i in 0..n {
      print!("{} ", arr[i as usize]);
    }
    println!("");
  }
}
