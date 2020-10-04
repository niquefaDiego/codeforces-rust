

fn main() {
  let stdin = std::io::stdin();
  let mut line: String = String::new();
  stdin.read_line(&mut line).unwrap();
  let ntc: i32 = line.trim().parse().unwrap();

  for _ in 0..ntc {
    stdin.read_line(&mut line).unwrap();

    line.clear();
    stdin.read_line(&mut line).unwrap();
    let a: Vec<i32> = line.split_whitespace().map(|s| s.parse().expect("parse error")).collect();
    
    line.clear();
    stdin.read_line(&mut line).unwrap();
    let mut b: Vec<i32> = line.split_whitespace().map(|s| s.parse().expect("parse error")).collect();
    b.sort();
    
    let mut ans: i32 = 0;
    for x in a {
      let mut fr = 0;
      let mut to = b.len()-1;
      while fr < to {
        let mid = (fr + to) / 2;
        if x > b[mid] {
          fr = mid + 1;
        } else {
          to = mid;
        }
      }
      if b[fr] == x {
        ans = x;
        break;
      }
    }

    if ans == 0 {
      println!("NO")
    } else {
      println!("YES\n1 {}", ans);
    }
  }
}
