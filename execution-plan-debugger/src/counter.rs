pub struct Counter {
    pub curr: Option<usize>,
    pub max: usize,
}

impl PartialEq<Option<usize>> for Counter {
    fn eq(&self, other: &Option<usize>) -> bool {
        &self.curr == other
    }
}

impl std::fmt::Display for Counter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.curr {
            None => write!(f, "Start"),
            Some(i) => i.fmt(f),
        }
    }
}

impl Counter {
    pub fn new(list_len: usize) -> Self {
        Self {
            curr: None,
            max: list_len - 1,
        }
    }

    pub fn is_start(&self) -> bool {
        self.curr.is_none()
    }

    /// Increment the counter
    pub fn inc(&mut self) {
        match self.curr {
            Some(c) => {
                if c < self.max {
                    self.curr = Some(c + 1);
                }
            }
            None => self.curr = Some(0),
        }
    }
    /// Decrement the counter
    pub fn dec(&mut self) {
        if let Some(c) = self.curr {
            self.curr = if c > 0 { Some(c - 1) } else { None }
        }
    }
}
