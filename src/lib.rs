
use rand::prelude::*;

#[derive(Copy, Clone, Debug)]
struct ProbCell {
    prob: f32,
    count: usize
}

struct MarkovLanguageGenerator {
    grams: Vec<String>,
    mat: Vec<Vec<ProbCell>>,
    rng: rand::rngs::ThreadRng,
    ngram: usize
}

impl MarkovLanguageGenerator {
    pub fn new(ngram: usize) -> MarkovLanguageGenerator {
        return MarkovLanguageGenerator {
            grams: vec![],
            mat: vec![vec![ProbCell{prob: 0.0, count: 0}; 0]; 0],
            rng: rand::thread_rng(),
            ngram: ngram
        }
    }
    pub fn fit(&mut self, s: &String) -> Result<(), String> {
        let chars = s.chars().collect::<Vec<char>>();
        let n_chars = chars.iter().count();
        if n_chars <= 2*self.ngram {
            return Err("Text size must be longer than 2*ngram size".to_owned());
        }
        for i in 0..n_chars - self.ngram*2 {
            let gram1 = &chars[i..i + self.ngram];
            let gram2 = &chars[i + self.ngram..i + self.ngram * 2];
            let c1_i = self.get_or_insert_ngram_index(gram1.iter().collect());
            let c2_i = self.get_or_insert_ngram_index(gram2.iter().collect());
            self.mat[c1_i][c2_i].count += 1;
        }
        self.adjust_probs();
        Ok(())
    }
    pub fn adjust_probs(&mut self) {
        for row in self.mat.iter_mut() {
            let row_total = row.iter().fold(0usize, |sum, &v| sum + v.count);
            for col in row.iter_mut() {
                if row_total == 0 {
                    col.prob = 0f32;
                } else {
                    col.prob = col.count as f32 / row_total as f32;
                }
            }
        }
    }
    pub fn get_or_insert_ngram_index(&mut self, c: String) -> usize {
        match self.grams.iter().position(|entry| *entry == c) {
            Some(i) => i,
            None => {
                for row in self.mat.iter_mut() {
                    row.push(ProbCell{prob: 0f32, count: 0});
                }
                self.mat.push(vec![ProbCell{prob: 0f32, count: 0}; self.mat.len() + 1]);
                self.grams.push(c);
                self.mat.len() - 1
            }
        }
    }
    pub fn gen(&mut self, len: usize) -> Result<String, String> {
        let mut buf = String::with_capacity(len);
        let mut current = self.grams[self.rng.next_u64() as usize % self.grams.len()].clone();
        for _ in 0..len {
            let dest_prob = self.rng.gen_range(0f32, 1f32);
            buf.push_str(&current);
            let mut cummulative_prob = 0f32;
            let mut index = 0;
            let row_i = self.get_or_insert_ngram_index(current);
            for cell in self.mat[row_i].iter() {
                cummulative_prob += cell.prob;
                if cummulative_prob >= dest_prob {
                    break;
                }
                index += 1;
            }
            if index < self.grams.len() {
                current = self.grams[index].clone();
            } else {
                break;
            }
        }
        Ok(buf)
    }
    pub fn print_count(&self) {
        let ngram_space = " ".repeat(self.ngram);
        print!("{} ", ngram_space);
        for c in self.grams.iter() {
            print!("{} ", c);
        }
        println!();
        for (i, row) in self.mat.iter().enumerate() {
            print!("{} ", self.grams[i]);
            for col in row {
                print!("{}{}", col.count, ngram_space);
            }
            println!();
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_instantiation() {
        MarkovLanguageGenerator::new(1);
    }

    #[test]
    fn test_fit_ngram_len_validation() {
        let mut generator = MarkovLanguageGenerator::new(5);

        assert_eq!(generator.fit(&"abc".to_owned()), Err("Text size must be longer than 2*ngram size".to_owned()));
    }
}