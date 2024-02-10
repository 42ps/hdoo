use std::str::Chars;

struct Solution;

impl Solution {
    pub fn merge_alternately(word1: String, word2: String) -> String {
        let mut wordit1 = word1.chars();
        let mut wordit2 = word2.chars();

        let mut result = String::new();

        loop {
            match (wordit1.next(), wordit2.next()) {
                (Some(c1), Some(c2)) => {
                    result.push(c1);
                    result.push(c2);
                }
                (Some(c1), None) => {
                    result.push(c1);
                    result.push_str(&wordit1.collect::<String>());
                    break;
                }
                (None, Some(c2)) => {
                    result.push(c2);
                    result.push_str(&wordit2.collect::<String>());
                    break;
                }
                (None, None) => {
                    // Both iterators are exhausted, exit the loop.
                    break;
                }
            }
        }

        result
    }

    pub fn merge_alternately2(word1: String, word2: String) -> String {
        let mut result = String::from("");
        let mut its = word1.chars().zip(word2.chars());

        // Iterating over both Strings simultaneously and pushing chars
        while let Some(chars) = its.next() {
            result.push(chars.0);
            result.push(chars.1);
        }

        result.push_str(if word1.len() < word2.len() {
            &word2[word1.len()..]
        } else if word1.len() > word2.len() {
            &word1[word2.len()..]
        } else {
            ""
        });

        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let word1 = String::from("as");
        let word2 = String::from("df");
        assert_eq!(
            Solution::merge_alternately(word1.clone(), word2.clone()),
            "adsf"
        );
        assert_eq!(
            Solution::merge_alternately2(word1.clone(), word2.clone()),
            "adsf"
        );
    }

    #[test]
    fn suffix() {
        let word1 = String::from("as");
        let word2 = String::from("dfttasdfasf");
        assert_eq!(
            Solution::merge_alternately(word1.clone(), word2.clone()),
            "adsfttasdfasf"
        );
        assert_eq!(
            Solution::merge_alternately2(word1.clone(), word2.clone()),
            "adsfttasdfasf"
        );
    }
}
