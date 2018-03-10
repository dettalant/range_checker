use std::fmt;
use std::default::Default;

#[derive(Clone)]
/// 数字二つの範囲を取る
pub struct Range<T> {
    /// 範囲の最小値
    low: T,
    /// 範囲の最大値
    high: T,
}

/// 数字二つの範囲についての機能。
pub trait RangeImpl: Sized {
    type T: PartialOrd;
    
    /// 構造体の生成
    fn new(low: Self::T, high: Self::T) -> Self;
    
    /// 範囲の最小値を出力
    fn low(&self) -> Self::T;
    
    /// 範囲の最大値を出力
    fn high(&self) -> Self::T;
    
    /// 二つのRangeを比較して、重なり合っているかをboolで返す
    fn is_overlap(&self, other: &Range<Self::T>) -> bool;
}

// バリエーションを楽に追加するためにマクロ
macro_rules! range_int_impl {
    ( $($ty:ty),+ ) => {
        $( 
            impl RangeImpl for Range<$ty> {
                type T = $ty;
                
                fn new(low: Self::T, high: Self::T) -> Self {
                    Range {
                        low: low,
                        high: high,
                    }
                }
                
                fn low(&self) -> Self::T {
                    self.low
                }
                
                fn high(&self) -> Self::T {
                    self.high
                }
                
                fn is_overlap(&self, other: &Range<Self::T>) -> bool {
                    self.low <= other.low && self.high >= other.low ||
                    self.low >= other.low && self.low <= other.high
                }
            }
        )+
    }
}

range_int_impl!(usize, u8, u16, u32, u64);
range_int_impl!(isize, i8, i16, i32, i64);
range_int_impl!(f32, f64);


// for Debug
impl <T: fmt::Debug>fmt::Debug for Range<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Range({:?}..{:?})", self.low, self.high)
    }
}

// for Display
impl <T: fmt::Display>fmt::Display for Range<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}..{}", self.low, self.high)
    }
}

macro_rules! range_default_float_impl {
    ($ty:ty) => {
        impl Default for Range<$ty> {
            fn default() -> Range<$ty> {
                Range::new(0.0, 0.0)
            }
        }
    }
}

macro_rules! range_default_int_impl {
    ($ty:ty) => {
        impl Default for Range<$ty> {
            fn default() -> Range<$ty> {
                Range::new(0, 0)
            }
        }
    }
}

macro_rules! range_default_impl {
    ( f32 ) => ( range_default_float_impl!(f32); );
    ( f64 ) => ( range_default_float_impl!(f64); );
    ( $( $ty:ty ),+ ) => {
        $( range_default_int_impl!($ty); )+
    }
}

// intをずらずらと
range_default_impl!(usize, u8, u16, u32, u64);
range_default_impl!(isize, i8, i16, i32, i64);

// 仕様上、floatは二回に分けておく
range_default_impl!(f32);
range_default_impl!(f64);
