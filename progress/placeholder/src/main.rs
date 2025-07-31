/// A struct that acts as a peekable zip on peekable iterators.
/// next() returns what peek() would return without advancing the iterator unless zip would return
/// a result
struct PeekingZip<'a, 'b, I: Iterator, J: Iterator>
{
    iter_a: &'a mut std::iter::Peekable<I>,
    iter_b: &'b mut std::iter::Peekable<J>,
}

impl<'a, 'b, I: Iterator, J: Iterator> PeekingZip<'a, 'b, I, J>
{
    /// Creates a new PeekingIterator from mutable references to iterators
    fn new(iter_a: &'a mut std::iter::Peekable<I>, iter_b: &'b mut std::iter::Peekable<J>) -> Self {
        PeekingZip {
            iter_a,
            iter_b,
        }
    }
}

impl<'a, 'b, I: Iterator, J: Iterator> Iterator for PeekingZip<'a, 'b, I, J>
{
    type Item = (I::Item, J::Item);

    /// Returns the same value that peek() would return
    /// only advancing the iterator if Zip should return a value
    /// this allows continuing the iterator from the remainder without losing any values
    fn next(&mut self) -> Option<(I::Item, J::Item)> {
        // Return a copy of the peeked value if available
        let _peek_a = self.iter_a.peek()?;
        let _peek_b = self.iter_b.peek()?;

        Some((self.iter_a.next()?, self.iter_b.next()?))
    }
}

fn main() {
    // Original example with peekable
    println!("Original example with peekable:");
    let mut alist = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let blist = vec![1, 2, 3];

    let mut a_iter = alist.iter_mut().peekable();
    let mut b_iter = blist.iter().peekable();

    let mut peeking_zip = PeekingZip::new(&mut a_iter, &mut b_iter);
    for (a, b) in &mut peeking_zip {
        println!("a: {:?}, b: {:?}", a, b);
    }

    for a in a_iter {
        println!("a: {}", a);
    }

    for b in b_iter {
        println!("b: {}", b);
    }

}
