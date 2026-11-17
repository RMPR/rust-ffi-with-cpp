use cxx::let_cxx_string;

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("factory.h");

        type WordCounter;

        fn new_word_counter() -> UniquePtr<WordCounter>;
        fn add(self: Pin<&mut WordCounter>, w: &CxxString) -> Result<()>;
        fn count(self: &WordCounter, w: &CxxString) -> i32;
        fn distinct(self: &WordCounter) -> usize;
    }
}

fn main() -> Result<(), cxx::Exception> {
    let mut counter = ffi::new_word_counter();
    let text = "the quick brown fox jumps over the lazy dog the end";
    for word in text.split(' ') {
        let_cxx_string!(word = word);
        counter.pin_mut().add(&word)?;
    }

    let_cxx_string!(the = "the");
    println!("'the' appears {} times", counter.count(&the));
    println!("{} distinct words", counter.distinct());

    let_cxx_string!(empty = "");
    println!("adding \"\": {:?}", counter.pin_mut().add(&empty));
    Ok(())
} // UniquePtr dropped here: the C++ destructor runs
