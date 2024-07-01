pub trait YayNay {
    fn yay(&self) -> bool;
    fn nay(&self) -> bool;
}
impl YayNay for bool {
    fn yay(&self) -> bool {*self}
    fn nay(&self) -> bool {!*self}
}
impl <A> YayNay for Option<A>{
    fn yay(&self) -> bool {self.is_some()}
    fn nay(&self) -> bool {self.is_none()}
}
impl <A,B> YayNay for Result<A,B>{
    fn yay(&self) -> bool {self.is_ok()}
    fn nay(&self) -> bool {self.is_err()}
}

#[macro_export]
macro_rules! flow_base {
    ($block:block $($any:tt)*) => {if $($any)* {$block}};
}

#[macro_export]
macro_rules! flow_base_let {
    ($block:block ?$opt:expr) => {
        if $opt.nay() {$block}
    };
    ($block:block *$($opt:tt)*) => {
        let $($opt)* else {$block};
    };
    ($block:block $($var:ident)* =$opt:expr $(,$($plus:tt)*)*) => {
        let $($var)* = {
            let _tmp = $opt;
            if _tmp.nay() {$block}
            _tmp.unwrap()$(.$($plus)*)*
        };
    };
    ($block:block ($($($var:ident)*),*)=$opt:expr $(,$($plus:tt)*)*) => {
        let ($($($var)*),*) = {
            let _tmp = $opt;
            if _tmp.nay() {$block}
            _tmp.unwrap()$(.$($plus)*)*
        };
    };
    ($block:block [$($($var:ident)*),*]=$opt:expr $(,$($plus:tt)*)*) => {
        let [$($($var)*),*] = {
            let _tmp = $opt;
            if _tmp.nay() {$block}
            _tmp.unwrap()$(.$($plus)*)*
        };
    };    
}

#[macro_export]
macro_rules! exit {
    () => {return Default::default()};
    (>if ($($any:tt)*) $($ret:tt)*) => {flow_base!{{return $($ret)*} $($any)*}};
    (if $($any:tt)*) => {flow_base!{{return Default::default()} $($any)*}};
    (>($($any:tt)*) $($ret:tt)*) => {flow_base_let!{{return $($ret)*} $($any)*}};
    ($($any:tt)*) => {flow_base_let!{{return Default::default()} $($any)*}};
}

#[macro_export]
macro_rules! next {
    (if $($any:tt)*) => {flow_base!{{continue} $($any)*}};
    ($($any:tt)*) => {flow_base_let!{{continue} $($any)*}};
}

#[macro_export]
macro_rules! kill {
    (if $($any:tt)*) => {flow_base!{{panic!()} $($any)*}};
    ($($any:tt)*) => {flow_base_let!{{panic!()} $($any)*}};
}

#[macro_export]
macro_rules! hold {
    (if $($any:tt)*) => {flow_base!{{break} $($any)*}};
    ($($any:tt)*) => {flow_base_let!{{break} $($any)*}};
}