mod sparse;

pub mod prelude {
    pub use super::MarkovLanguageGenerator;
}

use rand::prelude::*;

#[derive(Copy, Clone, Debug)]
struct ProbCell {
    prob: f32,
    count: usize
}

pub struct MarkovLanguageGenerator {
    grams: Vec<String>,
    // matrix: sparse::SparseMatrix<String>,
    mat: sparse::SparseMatrix<ProbCell>,
    rng: rand::rngs::ThreadRng,
    ngram: usize
}

impl MarkovLanguageGenerator {
    pub fn new(ngram: usize) -> MarkovLanguageGenerator {
        return MarkovLanguageGenerator {
            // matrix: sparse::SparseMatrix::new(),
            grams: vec![],
            mat: sparse::SparseMatrix::new(),
            rng: rand::thread_rng(),
            ngram: ngram
        }
    }

    pub fn fit_str(&mut self, s: &str) -> Result<(), String> {
        self.fit(&s.to_owned())
    }

    pub fn fit(&mut self, s: &String) -> Result<(), String> {
        let chars = s.chars().collect::<Vec<char>>();
        let n_chars = chars.iter().count();
        if n_chars < 2*self.ngram {
            return Err("Text size must be at least 2*ngram size".to_owned());
        }

        for i in 0..n_chars - self.ngram * 2 {
            let gram1 = &chars[i..i + self.ngram];
            let gram2 = &chars[i + self.ngram..i + self.ngram * 2];
            let c1_i = self.get_or_insert_ngram_index(gram1.iter().collect());
            let c2_i = self.get_or_insert_ngram_index(gram2.iter().collect());
            match self.mat.get(c1_i, c2_i) {
                Some(mut cell) => {
                    cell.count += 1;
                    self.mat.put(c1_i, c2_i, cell);
                },
                None => {
                    self.mat.put(c1_i, c2_i, ProbCell { count: 1, prob: 0f32 });
                }
            }
        }
        self.adjust_probs();
        Ok(())
    }

    pub fn adjust_probs(&mut self) {
        for r in 0..self.grams.len() {
            let row_total = self.mat.iter_row(r).fold(0usize, |sum, v| sum + v.count);
            self.mat.row_for_each(r, |cell| {
                if row_total == 0 {
                    cell.prob = 0f32;
                } else {
                    cell.prob = cell.count as f32 / row_total as f32;
                }
            });
        }
    }

    pub fn get_or_insert_ngram_index(&mut self, c: String) -> usize {
        match self.grams.iter().position(|entry| *entry == c) {
            Some(i) => i,
            None => {
                self.grams.push(c);
                self.grams.len() - 1
            }
        }
    }

    pub fn gen(&mut self, len: usize) -> Result<String, String> {
        let mut buf = String::with_capacity(len);
        let mut current = self.grams[self.rng.next_u64() as usize % self.grams.len()].clone();
        while buf.len() < len {
            let dest_prob = self.rng.gen_range(0f32, 1f32);
            buf.push_str(&current);
            let mut cummulative_prob = 0f32;
            let mut index = 0;
            let row_i = self.get_or_insert_ngram_index(current);
            for cell in self.mat.iter_row(row_i) {
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
        buf.truncate(len);
        Ok(buf)
    }
}


#[cfg(test)]
mod tests {
    extern crate regex;

    use super::*;
    use regex::Regex;
    use std::str::FromStr;

    #[test]
    fn test_instantiation() {
        MarkovLanguageGenerator::new(1);
    }

    #[test]
    fn test_fit_ngram_len_validation() {
        let mut generator = MarkovLanguageGenerator::new(5);

        assert_eq!(generator.fit(&"abc".to_owned()), Err("Text size must be at least 2*ngram size".to_owned()));
    }

    #[test]
    fn test_larger_ngrams() {
        let mut generator = MarkovLanguageGenerator::new(3);
        generator.fit(&"helloo".to_owned()).unwrap();
        generator.fit(&"hellooooo".to_owned()).unwrap();
        generator.fit(&"hello".to_owned()).unwrap_err();
    }

    #[test]
    fn test_fit() {
        let mut generator = MarkovLanguageGenerator::new(1);
        generator.fit(&"abbbbbc".to_owned()).unwrap();

        assert_eq!(generator.mat.get(0, 1).unwrap().count, 1);
        assert_eq!(generator.mat.get(1, 1).unwrap().count, 4);
    }

    #[test]
    fn test_gen() {
        let mut generator = MarkovLanguageGenerator::new(1);
        let mut text: String = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Accumsan sit amet nulla facilisi morbi tempus. Risus in hendrerit gravida rutrum quisque non tellus orci. Commodo ullamcorper a lacus vestibulum sed arcu non. Egestas quis ipsum suspendisse ultrices gravida dictum fusce. Eu sem integer vitae justo. Nunc sed augue lacus viverra vitae congue eu. Mi proin sed libero enim sed faucibus turpis in eu. Nec feugiat in fermentum posuere urna nec tincidunt praesent semper. Morbi tristique senectus et netus et malesuada fames ac turpis. Feugiat scelerisque varius morbi enim nunc faucibus a pellentesque sit. Tellus orci ac auctor augue mauris. Tellus id interdum velit laoreet id donec. Convallis a cras semper auctor. Eget gravida cum sociis natoque. Senectus et netus et malesuada fames ac turpis egestas integer.".to_owned();
        text = text.to_lowercase();
        let reg = Regex::from_str("[^A-Za-z]").unwrap();
        text = reg.replace_all(&text, "").into_owned();
        assert!(generator.fit(&text).is_ok());
    }
}