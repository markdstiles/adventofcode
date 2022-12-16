pub struct FindAllIter<'a> {
    source: &'a str,
    curr_pos: usize,
    pat_parts: Vec<String>,
    done: bool,
}

impl<'a> FindAllIter<'a> {
    pub fn new(source: &'a str, pat: &str) -> FindAllIter<'a> {
        //We split the pattern by the placeholders, we're trying to find the text
        //where {} is positioned. E.g The pattern "Hello {}!" would find the word
        //"world" in "Hello world!"
        let pat_parts: Vec<String> = pat.split("{}").map(|s| s.to_string()).collect();

        FindAllIter { source, curr_pos: 0, pat_parts, done: false }
    }
}

impl Iterator for FindAllIter<'_> {
    type Item = Vec<String>;

    fn next(&mut self) -> Option<Self::Item> {
        //Check if the iterator is already complete
        if self.done {
            return None;
        }

        //If no pattern to match then we can't match anything
        if self.pat_parts.is_empty() || (
            self.pat_parts.len() == 1 && self.pat_parts[0].is_empty()
        ) {
            self.done = true;
            return None;
        }

        //When the pattern "{}" is used, we're expected to return
        //everything. The pat_parts will be empty.
        if self.pat_parts.len() == 2 
        && self.pat_parts[0].is_empty() 
        && self.pat_parts[1].is_empty() {
            self.done = true;
            return Some(vec!{ self.source.to_string() });
        }

        //Walk through self, only return matches when all parts of the pattern match
        //if there are no matches for a part of the pattern then we can return immediately        
        loop {
            let mut matched = false;
            let mut working = vec!();

            for i in 1..self.pat_parts.len() {                
                let left = &self.pat_parts[i-1];
                let right = &self.pat_parts[i];

                if let Some(m) = self.source.text_between(left, right) {
                    //We've matched part of the pattern so store the text between
                    //the left and right parts
                    matched = true;
                    working.push(m.to_string());
                    self.curr_pos = self.source.find(left).unwrap() + left.len();
                    self.source = &self.source[self.curr_pos..];
                } else {
                    //This is an incomplete match therefore 
                    //break out of this loop and start again
                    matched = false;
                    break;
                }
            }

            //If the first part of the pattern is not found in the remaining 
            //string then we're finished.
            if !self.source.contains(&self.pat_parts[0]) {
                self.done = true;
            }

            if matched {
                //We got a match for the whole pattern so return it
                return Some(working);
            }
            
            //We might not have returned a match but we might be done
            //in which case we can return None and get out of the loop
            if self.done {
                return None;
            }
        }
    }
}

pub trait FindAll {
    fn find_all(&self, pat: &str) -> FindAllIter;
    fn text_between(&self, left: &str, right: &str) -> Option<String>;
}

impl FindAll for str {
    fn find_all(&self, pat: &str) -> FindAllIter {
        FindAllIter::new(self, pat)
    }

    fn text_between(&self, left: &str, right: &str, ) -> Option<String> {
        if left.is_empty() && right.is_empty() {
            return Some(self.to_string());
        }

        if left.is_empty() {
            if let Some(end) = self.find(right) {
                return Some(self[..end].to_string())
            }
        }

        if let Some(i) = self.find(left) {
            let start = i + left.len();

            if right.is_empty() {
                return Some(self[start..].to_string())
            } 
            
            if let Some(end) = self[start..].find(right) {
                return Some(self[start..start + end].to_string())
            }
        } 

        None
    }
}

impl FindAll for String {
    fn find_all(&self, pat: &str) -> FindAllIter {
        FindAllIter::new(self, pat)
    }

    fn text_between(&self, left: &str, right: &str, ) -> Option<String> {
        self.as_str().text_between(left, right)
    }
}