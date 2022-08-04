use std::fmt::Write;
use std::io::{stdin, Read};
use std::collections::VecDeque;

struct UFT {
    par: Vec<usize>,
}

impl UFT {
    fn new(n: usize) -> Self {
        Self {
            par: (0..n as usize).collect()
        }
    }
    fn find(&mut self, k: usize) -> usize {
        if self.par[k]==k {
            return k;
        }
        else {
            self.par[k] = self.find(self.par[k]);
            return self.par[k];
        }
    }
    
    fn union(&mut self, x: usize, y: usize) -> bool {
        let nx = self.find(x);
        let ny = self.find(y);
        if nx==ny {
            return false;
        }
        self.par[ny] = nx;
        return true;
    }
}

fn main() {
    let mut input = String::new();
    let mut output = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut numbers = input.split_ascii_whitespace().map(str::parse::<i32>).flatten();

    let N = numbers.next().unwrap() as usize;
    let K = numbers.next().unwrap() as usize;

    let mut uft = UFT::new((N+5)*(N+5));
    let has = N+1;
    let mut dp = vec![vec![0; N+5]; N+5];
    let mut que = VecDeque::new(); 
    let dx:[i32; 4] = [0, 0, -1, 1];
    let dy:[i32; 4] = [-1, 1, 0, 0];

    let mut ans = 0;
    let mut chk = 0;
    for _i in 0..K {
        let x = numbers.next().unwrap() as usize;
        let y = numbers.next().unwrap() as usize;
        dp[x][y] = 1;
        que.push_back((x, y));
    }

    while que.len() >= 1 {
        let elem = que.front().unwrap();
        let x = elem.0;
        let y = elem.1;
        let ux = x as usize;
        let uy = y as usize;
        que.pop_front();

        for i in 0..4 {
            let nx = (x as i32) + dx[i];
            let ny = (y as i32) + dy[i];
            if 1<=nx && nx<=(N as i32) && 1<=ny && ny<=(N as i32) {
                let unx = nx as usize;
                let uny = ny as usize;
                if dp[unx][uny]<=dp[ux][uy] && dp[unx][uny] != 0 {
                    if uft.union(unx*has+uny,ux*has+uy) {
                        ans = ans.max(dp[unx][uny]);
                        chk += 1;
                    }
                    if chk==K-1 {
                        ans = dp[ux][uy];
                        break;
                    }
                }
            }
        }

        if chk==K-1 {
            break;
        }

        for i in 0..4 {
            let nx = (x as i32) + dx[i];
            let ny = (y as i32) + dy[i];
            if 1<=nx && nx<=(N as i32) && 1<=ny && ny<= (N as i32) {
                let unx = nx as usize;
                let uny = ny as usize;
                if dp[unx][uny]==0 {
                    uft.union(unx*has+uny,ux*has+uy);
                    dp[unx][uny] = dp[ux][uy]+1;
                    que.push_back((unx,uny));
                }
            }
        }
    }

    // for i in 1..N {
    //     for j in 1..N {
    //         write!(output, "{}", dp[i][j]).unwrap();  
    //         write!(output, "{}", " ").unwrap();   
    //     }
    //     writeln!(output).unwrap();  
    // }

    write!(output, "{}", ans-1).unwrap();   
    print!("{}", output);
}
