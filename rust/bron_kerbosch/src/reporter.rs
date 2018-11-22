use graph::Vertex;
pub type Clique = Vec<Vertex>;

pub trait Reporter {
    fn inc_count(&mut self);
    fn record(&mut self, clique: &Clique);
}

pub struct SimpleReporter {
    pub cnt: i32,
    pub cliques: Vec<Clique>,
}

impl SimpleReporter {
    pub fn new() -> SimpleReporter {
        SimpleReporter {
            cnt: 0,
            cliques: vec![],
        }
    }
}

impl Reporter for SimpleReporter {
    fn inc_count(&mut self) {
        self.cnt += 1
    }

    fn record(&mut self, clique: &Clique) {
        if clique.len() > 1 {
            self.cliques.push(clique.clone());
        }
    }
}
