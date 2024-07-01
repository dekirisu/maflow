use maflow::*;

fn main () {
    let mut a = 0;
    for i in 0..10 {
        next!{if i == 2} // continue if ..
        hold!{if i == 5} // break if ..
        a += 1;
    }
    assert_eq!(a,4);
    assert!(func(Some(true)))
}

fn func (bo:Option<bool>) -> bool {
    exit!{is_true = bo} // return false if None
    is_true
}