use core::cast::transmute;
use core::cmp::{Eq, Ord};
use core::ptr::to_unsafe_ptr;
use core::sys::size_of;
use core::util::swap;
use core::vec::raw::buf_as_slice;

use std::cmp::{FuzzyEq, FUZZY_EPSILON};
use numeric::*;
use numeric::number::Number;
use numeric::number::Number::{zero,one};

use vec::{
    Vector,
    Vector4,
    MutableVector,
    NumericVector,
    NumericVector4,
    MutableNumericVector,
    ToHomogeneous,
    EuclideanVector,
    MutableEuclideanVector,
    EquableVector,
    OrdinalVector,
    BooleanVector,
};

/**
 * A 4-dimensional vector
 *
 * # Type parameters
 *
 * * `T` - The type of the components. This is intended to support boolean,
 *         integer, unsigned integer, and floating point types.
 *
 * # Fields
 *
 * * `x` - the first component of the vector
 * * `y` - the second component of the vector
 * * `z` - the third component of the vector
 * * `w` - the fourth component of the vector
 */
#[deriving_eq]
pub struct Vec4<T> { x: T, y: T, z: T, w: T }

pub impl<T:Copy Eq> Vec4<T>: Vector<T> {
    #[inline(always)]
    static pure fn from_value(value: T) -> Vec4<T> {
        Vector4::new(value, value, value, value)
    }
    
    #[inline(always)]
    pure fn to_ptr(&self) -> *T {
        unsafe {
            transmute::<*Vec4<T>, *T>(
                to_unsafe_ptr(self)
            )
        }
    }
}

pub impl<T> Vec4<T>: Vector4<T> {
    #[inline(always)]
    static pure fn new(x: T, y: T, z: T, w: T) -> Vec4<T> {
        Vec4 { x: x, y: y, z: z, w: w }
    }
}

pub impl<T:Copy Eq> Vec4<T>: Index<uint, T> {
    #[inline(always)]
    pure fn index(&self, i: uint) -> T {
        unsafe { do buf_as_slice(self.to_ptr(), 4) |slice| { slice[i] } }
    }
}

pub impl<T:Copy> Vec4<T>: MutableVector<T> {
    #[inline(always)]
    fn index_mut(&mut self, i: uint) -> &self/mut T {
        match i {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            3 => &mut self.w,
            _ => fail!(fmt!("index out of bounds: expected an index from 0 to 3, but found %u", i))
        }
    }
    
    #[inline(always)]
    fn swap(&mut self, a: uint, b: uint) {
        swap(self.index_mut(a),
             self.index_mut(b));
    }
}

pub impl<T:Copy Number> Vec4<T>: NumericVector<T> {
    #[inline(always)]
    static pure fn identity() -> Vec4<T> {
        Vector4::new(one::<T>(), one::<T>(), one::<T>(), one::<T>())
    }
    
    #[inline(always)]
    static pure fn zero() -> Vec4<T> {
        Vector4::new(zero::<T>(), zero::<T>(), zero::<T>(), zero::<T>())
    }
    
    #[inline(always)]
    pure fn is_zero(&self) -> bool {
        self[0] == zero() &&
        self[1] == zero() &&
        self[2] == zero() &&
        self[3] == zero()
    }
    
    #[inline(always)]
    pure fn mul_t(&self, value: T) -> Vec4<T> {
        Vector4::new(self[0] * value,
                     self[1] * value,
                     self[2] * value,
                     self[3] * value)
    }
    
    #[inline(always)]
    pure fn div_t(&self, value: T) -> Vec4<T> {
        Vector4::new(self[0] / value,
                     self[1] / value,
                     self[2] / value,
                     self[3] / value)
    }
    
    #[inline(always)]
    pure fn add_v(&self, other: &Vec4<T>) -> Vec4<T> {
        Vector4::new(self[0] + other[0],
                     self[1] + other[1],
                     self[2] + other[2],
                     self[3] + other[3])
    }
    
    #[inline(always)]
    pure fn sub_v(&self, other: &Vec4<T>) -> Vec4<T> {
        Vector4::new(self[0] - other[0],
                     self[1] - other[1],
                     self[2] - other[2],
                     self[3] - other[3])
    }
    
    #[inline(always)]
    pure fn mul_v(&self, other: &Vec4<T>) -> Vec4<T> {
        Vector4::new(self[0] * other[0],
                     self[1] * other[1],
                     self[2] * other[2],
                     self[3] * other[3])
    }
    
    #[inline(always)]
    pure fn div_v(&self, other: &Vec4<T>) -> Vec4<T> {
        Vector4::new(self[0] / other[0],
                     self[1] / other[1],
                     self[2] / other[2],
                     self[3] / other[3])
    }
    
    #[inline(always)]
    pure fn dot(&self, other: &Vec4<T>) -> T {
        self[0] * other[0] +
        self[1] * other[1] +
        self[2] * other[2] +
        self[3] * other[3]
    }
}

pub impl<T:Copy Number> Vec4<T>: Neg<Vec4<T>> {
    #[inline(always)]
    pure fn neg(&self) -> Vec4<T> {
        Vector4::new(-self[0], -self[1], -self[2], -self[3])
    }
}

pub impl<T:Copy Number> Vec4<T>: NumericVector4<T> {
    #[inline(always)]
    static pure fn unit_x() -> Vec4<T> {
        Vector4::new(one::<T>(), zero::<T>(), zero::<T>(), zero::<T>())
    }
    
    #[inline(always)]
    static pure fn unit_y() -> Vec4<T> {
        Vector4::new(zero::<T>(), one::<T>(), zero::<T>(), zero::<T>())
    }
    
    #[inline(always)]
    static pure fn unit_z() -> Vec4<T> {
        Vector4::new(zero::<T>(), zero::<T>(), one::<T>(), zero::<T>())
    }
    
    #[inline(always)]
    static pure fn unit_w() -> Vec4<T> {
        Vector4::new(zero::<T>(), zero::<T>(), zero::<T>(), one::<T>())
    }
}

pub impl<T:Copy Number> Vec4<T>: MutableNumericVector<&self/T> {
    #[inline(always)]
    fn neg_self(&mut self) {
        *self.index_mut(0) = -*self.index_mut(0);
        *self.index_mut(1) = -*self.index_mut(1);
        *self.index_mut(2) = -*self.index_mut(2);
        *self.index_mut(3) = -*self.index_mut(3);
    }
    
    #[inline(always)]
    fn mul_self_t(&mut self, value: &T) {
        *self.index_mut(0) *= (*value);
        *self.index_mut(1) *= (*value);
        *self.index_mut(2) *= (*value);
        *self.index_mut(3) *= (*value);
    }
    
    #[inline(always)]
    fn div_self_t(&mut self, value: &T) {
        *self.index_mut(0) /= (*value);
        *self.index_mut(1) /= (*value);
        *self.index_mut(2) /= (*value);
        *self.index_mut(3) /= (*value);
    }
    
    #[inline(always)]
    fn add_self_v(&mut self, other: &Vec4<T>) {
        *self.index_mut(0) += other[0];
        *self.index_mut(1) += other[1];
        *self.index_mut(2) += other[2];
        *self.index_mut(3) += other[3];
    }
    
    #[inline(always)]
    fn sub_self_v(&mut self, other: &Vec4<T>) {
        *self.index_mut(0) -= other[0];
        *self.index_mut(1) -= other[1];
        *self.index_mut(2) -= other[2];
        *self.index_mut(3) -= other[3];
    }
    
    #[inline(always)]
    fn mul_self_v(&mut self, other: &Vec4<T>) {
        *self.index_mut(0) *= other[0];
        *self.index_mut(1) *= other[1];
        *self.index_mut(2) *= other[2];
        *self.index_mut(3) *= other[3];
    }
    
    #[inline(always)]
    fn div_self_v(&mut self, other: &Vec4<T>) {
        *self.index_mut(0) /= other[0];
        *self.index_mut(1) /= other[1];
        *self.index_mut(2) /= other[2];
        *self.index_mut(3) /= other[3];
    }
}

pub impl<T:Copy Float> Vec4<T>: EuclideanVector<T> {
    #[inline(always)]
    pure fn length2(&self) -> T {
        self.dot(self)
    }
    
    #[inline(always)]
    pure fn length(&self) -> T {
        self.length2().sqrt()
    }
    
    #[inline(always)]
    pure fn distance2(&self, other: &Vec4<T>) -> T {
        other.sub_v(self).length2()
    }
    
    #[inline(always)]
    pure fn distance(&self, other: &Vec4<T>) -> T {
        other.distance2(self).sqrt()
    }
    
    #[inline(always)]
    pure fn angle(&self, other: &Vec4<T>) -> T {
        acos(self.dot(other) / (self.length() * other.length()))
    }
    
    #[inline(always)]
    pure fn normalize(&self) -> Vec4<T> {
        self.mul_t(one::<T>()/self.length())
    }
    
    #[inline(always)]
    pure fn normalize_to(&self, length: T) -> Vec4<T> {
        self.mul_t(length / self.length())
    }
    
    #[inline(always)]
    pure fn lerp(&self, other: &Vec4<T>, amount: T) -> Vec4<T> {
        self.add_v(&other.sub_v(self).mul_t(amount))
    }
}

pub impl<T:Copy Float> Vec4<T>: MutableEuclideanVector<&self/T> {
    #[inline(always)]
    fn normalize_self(&mut self) {
        let n = one::<T>() / self.length();
        self.mul_self_t(&n);
    }
    
    #[inline(always)]
    fn normalize_self_to(&mut self, length: &T) {
        let n = length / self.length();
        self.mul_self_t(&n);
    }
    
    fn lerp_self(&mut self, other: &Vec4<T>, amount: &T) {
        self.add_self_v(&other.sub_v(&*self).mul_t(*amount));
    }
}

pub impl<T:Copy Float FuzzyEq<T>> Vec4<T>: FuzzyEq<T> {
    #[inline(always)]
    pure fn fuzzy_eq(&self, other: &Vec4<T>) -> bool {
        self.fuzzy_eq_eps(other, &Number::from(FUZZY_EPSILON))
    }
    
    #[inline(always)]
    pure fn fuzzy_eq_eps(&self, other: &Vec4<T>, epsilon: &T) -> bool {
        self[0].fuzzy_eq_eps(&other[0], epsilon) &&
        self[1].fuzzy_eq_eps(&other[1], epsilon) &&
        self[2].fuzzy_eq_eps(&other[2], epsilon) &&
        self[3].fuzzy_eq_eps(&other[3], epsilon)
    }
}

pub impl<T:Copy Ord Eq> Vec4<T>: OrdinalVector<T, Vec4<bool>> {
    #[inline(always)]
    pure fn less_than(&self, other: &Vec4<T>) -> Vec4<bool> {
        Vector4::new(self[0] < other[0],
                     self[1] < other[1],
                     self[2] < other[2],
                     self[3] < other[3])
    }
    
    #[inline(always)]
    pure fn less_than_equal(&self, other: &Vec4<T>) -> Vec4<bool> {
        Vector4::new(self[0] <= other[0],
                     self[1] <= other[1],
                     self[2] <= other[2],
                     self[3] <= other[3])
    }
    
    #[inline(always)]
    pure fn greater_than(&self, other: &Vec4<T>) -> Vec4<bool> {
        Vector4::new(self[0] > other[0],
                     self[1] > other[1],
                     self[2] > other[2],
                     self[3] > other[3])
    }
    
    #[inline(always)]
    pure fn greater_than_equal(&self, other: &Vec4<T>) -> Vec4<bool> {
        Vector4::new(self[0] >= other[0],
                     self[1] >= other[1],
                     self[2] >= other[2],
                     self[3] >= other[3])
    }
}

pub impl<T:Copy Eq> Vec4<T>: EquableVector<T, Vec4<bool>> {
    #[inline(always)]
    pure fn equal(&self, other: &Vec4<T>) -> Vec4<bool> {
        Vector4::new(self[0] == other[0],
                     self[1] == other[1],
                     self[2] == other[2],
                     self[3] == other[3])
    }
    
    #[inline(always)]
    pure fn not_equal(&self, other: &Vec4<T>) -> Vec4<bool> {
        Vector4::new(self[0] != other[0],
                     self[1] != other[1],
                     self[2] != other[2],
                     self[3] != other[3])
    }
}

pub impl Vec4<bool>: BooleanVector {
    #[inline(always)]
    pure fn any(&self) -> bool {
        self[0] || self[1] || self[2] || self[3]
    }
    
    #[inline(always)]
    pure fn all(&self) -> bool {
        self[0] && self[1] && self[2] && self[3]
    }
    
    #[inline(always)]
    pure fn not(&self) -> Vec4<bool> { 
        Vector4::new(!self[0], !self[1], !self[2], !self[3])
    }
}

// GLSL-style type aliases, corresponding to Section 4.1.5 of the [GLSL 4.30.6 specification]
// (http://www.opengl.org/registry/doc/GLSLangSpec.4.30.6.pdf).

pub type vec4  = Vec4<f32>;     // a four-component single-precision floating-point vector
pub type dvec4 = Vec4<f64>;     // a four-component double-precision floating-point vector
pub type bvec4 = Vec4<bool>;    // a four-component Boolean vector
pub type ivec4 = Vec4<i32>;     // a four-component signed integer vector
pub type uvec4 = Vec4<u32>;     // a four-component unsigned integer vector

// Static method wrappers for GLSL-style types

pub impl vec4 {
    #[inline(always)] static pure fn new(x: f32, y: f32, z: f32, w: f32) -> vec4 { Vector4::new(x, y, z, w) }
    #[inline(always)] static pure fn from_value(v: f32) -> vec4 { Vector::from_value(v) }
    #[inline(always)] static pure fn identity() -> vec4 { NumericVector::identity() }
    #[inline(always)] static pure fn zero() -> vec4 { NumericVector::zero() }
    
    #[inline(always)] static pure fn unit_x() -> vec4 { NumericVector4::unit_x() }
    #[inline(always)] static pure fn unit_y() -> vec4 { NumericVector4::unit_y() }
    #[inline(always)] static pure fn unit_z() -> vec4 { NumericVector4::unit_z() }
    #[inline(always)] static pure fn unit_w() -> vec4 { NumericVector4::unit_w() }
    
    #[inline(always)] static pure fn dim() -> uint { 4 }
    #[inline(always)] static pure fn size_of() -> uint { size_of::<vec4>() }
}

pub impl dvec4 {
    #[inline(always)] static pure fn new(x: f64, y: f64, z: f64, w: f64) -> dvec4 { Vector4::new(x, y, z, w) }
    #[inline(always)] static pure fn from_value(v: f64) -> dvec4 { Vector::from_value(v) }
    #[inline(always)] static pure fn identity() -> dvec4 { NumericVector::identity() }
    #[inline(always)] static pure fn zero() -> dvec4 { NumericVector::zero() }
    
    #[inline(always)] static pure fn unit_x() -> dvec4 { NumericVector4::unit_x() }
    #[inline(always)] static pure fn unit_y() -> dvec4 { NumericVector4::unit_y() }
    #[inline(always)] static pure fn unit_z() -> dvec4 { NumericVector4::unit_z() }
    #[inline(always)] static pure fn unit_w() -> dvec4 { NumericVector4::unit_w() }
    
    #[inline(always)] static pure fn dim() -> uint { 4 }
    #[inline(always)] static pure fn size_of() -> uint { size_of::<dvec4>() }
}
 
 
pub impl bvec4 {
    #[inline(always)] static pure fn new(x: bool, y: bool, z: bool, w: bool) -> bvec4 { Vector4::new(x, y, z, w) }
    #[inline(always)] static pure fn from_value(v: bool) -> bvec4 { Vector::from_value(v) }
    
    #[inline(always)] static pure fn dim() -> uint { 4 }
    #[inline(always)] static pure fn size_of() -> uint { size_of::<bvec4>() }
}

pub impl ivec4 {
    #[inline(always)] static pure fn new(x: i32, y: i32, z: i32, w: i32) -> ivec4 { Vector4::new(x, y, z, w) }
    #[inline(always)] static pure fn from_value(v: i32) -> ivec4 { Vector::from_value(v) }
    #[inline(always)] static pure fn identity() -> ivec4 { NumericVector::identity() }
    #[inline(always)] static pure fn zero() -> ivec4 { NumericVector::zero() }
    
    #[inline(always)] static pure fn unit_x() -> ivec4 { NumericVector4::unit_x() }
    #[inline(always)] static pure fn unit_y() -> ivec4 { NumericVector4::unit_y() }
    #[inline(always)] static pure fn unit_z() -> ivec4 { NumericVector4::unit_z() }
    #[inline(always)] static pure fn unit_w() -> ivec4 { NumericVector4::unit_w() }
    
    #[inline(always)] static pure fn dim() -> uint { 4 }
    #[inline(always)] static pure fn size_of() -> uint { size_of::<ivec4>() }
}

pub impl uvec4 {
    #[inline(always)] static pure fn new(x: u32, y: u32, z: u32, w: u32) -> uvec4 { Vector4::new(x, y, z, w) }
    #[inline(always)] static pure fn from_value(v: u32) -> uvec4 { Vector::from_value(v) }
    #[inline(always)] static pure fn identity() -> uvec4 { NumericVector::identity() }
    #[inline(always)] static pure fn zero() -> uvec4 { NumericVector::zero() }
    
    #[inline(always)] static pure fn unit_x() -> uvec4 { NumericVector4::unit_x() }
    #[inline(always)] static pure fn unit_y() -> uvec4 { NumericVector4::unit_y() }
    #[inline(always)] static pure fn unit_z() -> uvec4 { NumericVector4::unit_z() }
    #[inline(always)] static pure fn unit_w() -> uvec4 { NumericVector4::unit_w() }
    
    #[inline(always)] static pure fn dim() -> uint { 4 }
    #[inline(always)] static pure fn size_of() -> uint { size_of::<uvec4>() }
}