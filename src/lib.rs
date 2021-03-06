#![feature(test)]
#![feature(std_misc)]
#![feature(float_extras)]
extern crate test;

mod hash;

pub use hash::hash_one;
pub use hash::hash_two;
pub use hash::hash_three;

#[cfg(test)]
mod hash_tests {
    use hash::hash_one;
    use hash::hash_two;
    use hash::hash_three;
    use test::Bencher;

    #[test]
    fn it_works() {
        // yes I cheated. well, this tests if fn is deterministic
        // see it_actually_works for more tests...

        assert_eq!(hash_one("Greystark", 255_u32), 45);
        assert_eq!(hash_one("Greystark", 255_u32), 45);
        assert_eq!(hash_two("Karstark", 255_u32), 232);
        assert_eq!(hash_two("Karstark", 255_u32), 232);
        assert_eq!(hash_three("Stark", 255_u32), 47);
        assert_eq!(hash_three("Stark", 255_u32), 47);
    }
    #[test]
    fn it_actually_works() {
        let north_houses = [
            "Amber" , "Bolton", "Condon", "Dustin", "Forrester",
            "Greystark", "Harclay", "Ironsmith", "Karstark", "Lake", "Marsh",
            "Norrey", "Overton", "Peat", "Pool", "Quagg", "Redbeard", "Reed",
            "Ryswell", "Slate", "Stane", "Stark", "Stout", "Thenn", "Umber",
            "Waterman", "Wells", "Whitehill", "Woodfoot", "Woods", "Woolfield",
            "Wull", "Wull"
            ];
        let mut northern_hash_one = Vec::new();
        let mut northern_hash_two = Vec::new();
        let mut northern_hash_three = Vec::new();

        for house in north_houses.iter() {
            northern_hash_one.push(hash_one(house, 255_u32));
            northern_hash_two.push(hash_two(house, 255_u32));
            northern_hash_three.push(hash_three(house, 255_u32));
        }

        // 33 elements and a range of 255 - a "well distributed" hash function
        // would hash one element every interval of 7.73
        // To test distribution we'll sort the  hashed values
        northern_hash_one.sort_by(|a, b| a.cmp(b));
        northern_hash_two.sort_by(|a, b| a.cmp(b));
        northern_hash_three.sort_by(|a, b| a.cmp(b));

        // & check there's a *relatively* consistent difference b/w elements
        // The better the hashing fn the smaller the interval
        // Let's start with 5 < x < 10 as the range - %30 away from "ideal"
        // ( 7.73 ± 2.27 )
        {
            let mut iter = northern_hash_one.iter().peekable();
            while let Some(num) = iter.next() {
                if iter.peek() != None {
                    let difference: u32 = *iter.peek().unwrap() - *num;
                    assert!(difference >= 5 || difference <= 10);
                }
            }
        }
        {
            let mut iter = northern_hash_two.iter().peekable();
            while let Some(num) = iter.next() {
                if iter.peek() != None {
                    let difference: u32 = *iter.peek().unwrap() - *num;
                    assert!(difference >= 5 || difference <= 10);
                }
            }
        }
        {
            let mut iter = northern_hash_three.iter().peekable();
            while let Some(num) = iter.next() {
                if iter.peek() != None {
                    let difference: u32 = *iter.peek().unwrap() - *num;
                    assert!(difference >= 5 || difference <= 10);
                }
            }
        }

        // to test for collisions we'll dedup and expect a difference of <= 2,
        // or 3% (33 elements with 1 intentional duplicate - Wull)
        let northern_one_len: usize = northern_hash_one.len();
        let northern_two_len: usize = northern_hash_two.len();
        let northern_three_len: usize = northern_hash_three.len();
        northern_hash_one.dedup();
        northern_hash_two.dedup();
        northern_hash_three.dedup();

        assert!(northern_one_len - northern_hash_one.len() <= 2);
        assert!(northern_two_len - northern_hash_two.len() <= 2);
        assert!(northern_three_len - northern_hash_three.len() <= 2);
    }
    #[bench]
    fn hash_one_bench(b: &mut Bencher) {
        b.iter(|| hash_one("Great Danton", 255_u32));
    }
    #[bench]
    fn hash_two_bench(b: &mut Bencher) {
        b.iter(|| hash_two("The Professor", 255_u32));
    }
    #[bench]
    fn hash_three_bench(b: &mut Bencher) {
        b.iter(|| hash_three("Cutter", 255_u32));
    }
}

