use std::fmt;

/// 数字二つの範囲を取る
pub struct Range<T>(T, T);

/// Rangeを取るためのtrait
pub trait RangeImpl: Sized {
    type T: PartialOrd;
    
    fn new(low: Self::T, high: Self::T) -> Self;
    fn is_contain(&self, item: Self::T) -> bool;
    fn is_range_lap(&self, item: Range<Self::T>) -> bool;
}

/// Rangeに関する部分がここ
macro_rules! range_impl {
    ($ty:ty) => {
        impl RangeImpl for Range<$ty> {
            type T = $ty;
            
            /// Rangeの生成
            fn new(low: Self::T, high: Self::T) -> Self {
                Range(low, high) 
            }
            
            /// itemの数字がRange内に入っているかを判別して、boolで返す
            fn is_contain(&self, item: Self::T) -> bool {
                item >= self.0 && item <= self.1
            }
            
            /// 二つのRangeが重なっているかを判別して、boolで返す
            fn is_range_lap(&self, item: Range<Self::T>) -> bool {
                /*
                   条件式の上二行だけだと、両方入ってる際に取りこぼしがでる。
                   なので、逆条件をいれて補完。
                   
                   人情としては四行目の部分も付け足したくなるけど、
                   上二行で不足になるのは
                   「item.0とitem.1が両方ともselfに収まってる場合」
                   なので、論理的には三行で十分。
                */
                (item.0 >= self.0 && item.0 <= self.1) || 
                (item.1 >= self.0 && item.1 <= self.1) ||
                (self.0 >= item.0 && self.0 <= item.1) 
            }
        }
    }
}

// 各種typeを取り揃えております
range_impl! { i32 }
range_impl! { u32 }
range_impl! { f32 }

impl<T: fmt::Display> fmt::Display for Range<T> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}..{}", self.0, self.1)
    }
}

impl<T: fmt::Debug> fmt::Debug for Range<T> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "Range({:?}..{:?})", self.0, self.1)
    }
}
