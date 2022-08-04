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
