// The golden rule of dynamically sized types is that we must always put
// values of dynamically sized types behind a pointer of some kind.

pub fn run() {
    // Error, "str" must be "&str".
    // let s1: str = "Hello there!";
    // let s2: str = "How's it going?";
}
