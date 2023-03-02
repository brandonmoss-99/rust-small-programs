/* Generic struct, different sizes of N will produce different types */
struct NonogramRow<const N: usize> {
    values: [u8; N]
}

impl<const N: usize> NonogramRow<N> {
    fn new(values: [u8; N]) -> NonogramRow<N>{
        NonogramRow { values }
    }

    /// Count the number of consecutive 1s in the array, returning a Vector
    /// of the lengths of every consecutive run found
    fn solve(&self) -> Vec<usize> {
        let mut answer: Vec<usize> = Vec::new();
        let mut consecutive: usize = 0;

        for i in 0..N {
            match self.values[i] {
                1 => consecutive += 1,
                _ => {
                    if consecutive > 0 {
                        answer.push(consecutive);
                        consecutive = 0;
                    }
                }
            }
        }
        /* Write out the final consecutive value to the answer, if the last
        value is a 1 (causing it to not be written in the loop) */
        if consecutive > 0 {
            answer.push(consecutive);
        }

        answer
    }
}

fn main() {
    let empty_row = NonogramRow::new([]);
    let row1 = NonogramRow::new([0,0,0,0,0]);
    let row2 = NonogramRow::new([1,1,1,1,1]);
    let row3 = NonogramRow::new([0,1,1,1,1,1,0,1,1,1,1]);
    let row4 = NonogramRow::new([1,1,0,1,0,0,1,1,1,0,0]);
    let row5 = NonogramRow::new([0,0,0,0,1,1,0,0,1,0,1,1,1]);
    let row6 = NonogramRow::new([1,0,1,0,1,0,1,0,1,0,1,0,1,0,1]);
    println!("Answer to {:?} -> {:?}", empty_row.values, empty_row.solve());
    println!("Answer to {:?} -> {:?}", row1.values, row1.solve());
    println!("Answer to {:?} -> {:?}", row2.values, row2.solve());
    println!("Answer to {:?} -> {:?}", row3.values, row3.solve());
    println!("Answer to {:?} -> {:?}", row4.values, row4.solve());
    println!("Answer to {:?} -> {:?}", row5.values, row5.solve());
    println!("Answer to {:?} -> {:?}", row6.values, row6.solve());
}
