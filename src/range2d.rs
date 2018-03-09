use std::fmt;
use range::{ Range, RangeImpl };

#[derive(Clone)]
/// Range二つの値で成り立つ構造体
pub struct Range2D<T> {
    /// x軸の範囲
    x: Range<T>,
    /// y軸の範囲
    y: Range<T>,
}

/// x軸, y軸二つのRangeからなる、いわゆるRectangle向けの機能
pub trait Range2DImpl: Sized {
    type T: PartialOrd;
    
    /// 構造体の生成
    fn new(x: Self::T, y: Self::T, w: Self::T, h: Self::T) -> Self;
    
    /// すでにある範囲から、構造体を生成
    fn from_range(x_range: Range<Self::T>, y_range: Range<Self::T>) -> Self;
    
    /// x軸のRangeを出力
    fn x(&self) -> Range<Self::T>;
    
    /// y軸のRangeを出力
    fn y(&self) -> Range<Self::T>;
    
    /// 二つのRectangle範囲を比較する
    fn is_overlap(&self, other: &Range2D<Self::T>) -> bool;
}

// バリエーションを楽に追加するためにマクロ
macro_rules! range2d_int_impl {
    ($ty:ty) => {
        impl Range2DImpl for Range2D<$ty> {
            type T = $ty;
            
            fn new(x: Self::T, y: Self::T, w: Self::T, h: Self::T) -> Self {
                Range2D {
                    x: Range::new(x, x + w),
                    y: Range::new(y, y + h),
                }
            }
            
            fn from_range(x_range: Range<Self::T>, y_range: Range<Self::T>) -> Self {
                Range2D {
                    x: x_range,
                    y: y_range,
                }
            }
            
            fn x(&self) -> Range<Self::T> {
                self.x.clone()
            }
            
            fn y(&self) -> Range<Self::T> {
                self.y.clone()
            }
            
            fn is_overlap(&self, other: &Range2D<Self::T>) -> bool {
                self.x.is_overlap(&other.x) && self.y.is_overlap(&other.y)
            }
        }
    }
}

// 各種マクロ起動
range2d_int_impl!(u32);
range2d_int_impl!(i32);
range2d_int_impl!(f32);

// for Debug
impl <T: fmt::Debug>fmt::Debug for Range2D<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Range2D(x: {:?}, y: {:?})", self.x, self.y)
    }
}

// for Display
impl <T: fmt::Display>fmt::Display for Range2D<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
