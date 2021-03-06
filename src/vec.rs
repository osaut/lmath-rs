use core::cmp::Eq;

use std::cmp::FuzzyEq;

use numeric::Number;

pub use vec2::{Vec2, vec2, dvec2, bvec2, ivec2, uvec2};
pub use vec3::{Vec3, vec3, dvec3, bvec3, ivec3, uvec3};
pub use vec4::{Vec4, vec4, dvec4, bvec4, ivec4, uvec4};


/**
 * The base generic vector trait.
 *
 * # Type parameters
 *
 * * `T` - The type of the components. This is intended to support boolean,
 *         integer, unsigned integer, and floating point types.
 */
pub trait Vector<T>: Index<uint, T> Eq {
    /**
     * Construct the vector from a single value, copying it to each component
     */
    static pure fn from_value(value: T) -> Self;
    
    /**
     * # Return value
     *
     * A pointer to the first component of the vector
     */
    pure fn to_ptr(&self) -> *T;
}

pub trait MutableVector<T>: Vector<T> {
    /**
     * Get a mutable reference to the component at `i`
     */
    fn index_mut(&mut self, i: uint) -> &self/mut T;
    
    /**
     * Swap two components of the vector in place
     */
    fn swap(&mut self, a: uint, b: uint);
}

/**
 * A generic 2-dimensional vector
 */
pub trait Vector2<T>: Vector<T> {
    static pure fn new(x: T, y: T) -> Self;
}

/**
 * A generic 3-dimensional vector
 */
pub trait Vector3<T>: Vector<T> {
    static pure fn new(x: T, y: T, z: T) -> Self;
}

/**
 * A generic 4-dimensional vector
 */
pub trait Vector4<T>: Vector<T> {
    static pure fn new(x: T, y: T, z: T, w: T) -> Self;
}

/**
 * A vector with numeric components
 */
pub trait NumericVector<T>: Vector<T> Neg<Self> {
    /**
     * The standard basis vector
     *
     * # Return value
     *
     * A vector with each component set to one
     */
    static pure fn identity() -> Self;
    
    /**
     * The null vector
     *
     * # Return value
     *
     * A vector with each component set to zero
     */
    static pure fn zero() -> Self;
    
    /**
     * # Return value
     *
     * True if the vector is equal to zero
     */
    pure fn is_zero(&self) -> bool;
    
    /**
     * # Return value
     *
     * The scalar multiplication of the vector and `value`
     */
    pure fn mul_t(&self, value: T) -> Self;
    
    /**
     * # Return value
     *
     * The scalar division of the vector and `value`
     */
    pure fn div_t(&self, value: T) -> Self;
    
    /**
     * Component-wise vector addition
     */
    pure fn add_v(&self, other: &Self) -> Self;
    
    /**
     * Component-wise vector subtraction
     */
    pure fn sub_v(&self, other: &Self) -> Self;
    
    /**
     * Component-wise vector multiplication
     */
    pure fn mul_v(&self, other: &Self) -> Self;
    
    /**
     * Component-wise vector division
     */
    pure fn div_v(&self, other: &Self) -> Self;
    
    /**
     * # Return value
     *
     * The dot product of the vector and `other`
     */
    pure fn dot(&self, other: &Self) -> T;
}

/**
 * A 2-dimensional vector with numeric components
 */
pub trait NumericVector2<T>: NumericVector<T> {
    static pure fn unit_x() -> Self;
    static pure fn unit_y() -> Self;
    
    /**
     * # Return value
     *
     * The perp dot product of the vector and `other`
     */
    pure fn perp_dot(&self, other: &Self) -> T;
}

/**
 * A 3-dimensional vector with numeric components
 */
pub trait NumericVector3<T>: NumericVector<T> {
    static pure fn unit_x() -> Self;
    static pure fn unit_y() -> Self;
    static pure fn unit_z() -> Self;
    
    /**
     * # Return value
     *
     * The cross product of the vector and `other`
     */
    pure fn cross(&self, other: &Self) -> Self;
}

/**
 * A 4-dimensional vector with numeric components
 */
pub trait NumericVector4<T>: NumericVector<T> {
    static pure fn unit_x() -> Self;
    static pure fn unit_y() -> Self;
    static pure fn unit_z() -> Self;
    static pure fn unit_w() -> Self;
}

/**
 * A mutable vector with numeric components
 */
pub trait MutableNumericVector<T>: MutableVector<&self/T>
                                   NumericVector<T> {
    /**
     * Negate the vector
     */
    fn neg_self(&mut self);
    
    /**
     * Multiply the vector by a scalar
     */
    fn mul_self_t(&mut self, value: T);
    
    /**
     * Divide the vector by a scalar
     */
    fn div_self_t(&mut self, value: T);
    
    /**
     * Set the vector to the component-wise vector sum
     */
    fn add_self_v(&mut self, other: &Self);
    
    /**
     * Set the vector to the component-wise vector difference
     */
    fn sub_self_v(&mut self, other: &Self);
    
    /**
     * Set the vector to the component-wise vector product
     */
    fn mul_self_v(&mut self, other: &Self);
    
    /**
     * Set the vector to the component-wise vector quotient
     */
    fn div_self_v(&mut self, other: &Self);
}

/**
 * A mutable 3-dimensional vector with numeric components
 */
pub trait MutableNumericVector3<T>: MutableNumericVector<&self/T> {
    /**
     * Set to the cross product of the vector and `other`
     */
    fn cross_self(&mut self, other: &Self);
}

pub trait ToHomogeneous<H> {
    /**
     * Convert to a homogenous coordinate
     */
    pure fn to_homogeneous(&self) -> H;
}

/**
 * A Euclidean (or Affine) vector
 *
 * # Type parameters
 *
 * * `T` - The type of the components. This should be a floating point type.
 */
pub trait EuclideanVector<T>: NumericVector<T> {
    /**
     * # Return value
     *
     * The squared length of the vector. This is useful for comparisons where
     * the exact length does not need to be calculated.
     */
    pure fn length2(&self) -> T;
    
    /**
     * # Return value
     *
     * The length of the vector
     *
     * # Performance notes
     *
     * For instances where the exact length of the vector does not need to be
     * known, for example for quaternion-quaternion length comparisons,
     * it is advisable to use the `length2` method instead.
     */
    pure fn length(&self) -> T;
    
    /**
     * # Return value
     *
     * The squared distance between the vector and `other`.
     */
    pure fn distance2(&self, other: &Self) -> T;
    
    /**
     * # Return value
     *
     * The distance between the vector and `other`
     */
    pure fn distance(&self, other: &Self) -> T;
    
    /**
     * # Return value
     *
     * The angle between the vector and `other` in radians
     */
    pure fn angle(&self, other: &Self) -> T;
    
    /**
     * # Return value
     *
     * The normalized vector
     */
    pure fn normalize(&self) -> Self;
    
    /**
     * Set the length of the vector whilst preserving the direction
     */
    pure fn normalize_to(&self, length: T) -> Self;
    
    /**
     * Linearly intoperlate between the vector and `other`
     *
     * # Return value
     *
     * The intoperlated vector
     */
    pure fn lerp(&self, other: &Self, amount: T) -> Self;
}

/**
 * A mutable Euclidean (or Affine) vector
 *
 * # Type parameters
 *
 * * `T` - The type of the components. This should be a floating point type.
 */
pub trait MutableEuclideanVector<T>: MutableNumericVector<&self/T>
                                     EuclideanVector<T> {
    /**
     * Normalize the vector
     */
    fn normalize_self(&mut self);
    
    /**
     * Set the vector to a specified length whilst preserving the direction
     */
    fn normalize_self_to(&mut self, length: T);
    
    /**
     * Linearly intoperlate the vector towards `other`
     */
    fn lerp_self(&mut self, other: &Self, amount: T);
}

/**
 * Component-wise vector comparison methods
 *
 * The methods contained in this trait correspond to the relational functions
 * mentioned in Section 8.7 of the [GLSL 4.30.6 specification]
 * (http://www.opengl.org/registry/doc/GLSLangSpec.4.30.6.pdf).
 */
pub trait OrdinalVector<T, BoolVec>: Vector<T> {
    /**
     * Component-wise compare of `self < other`
     */
    pure fn less_than(&self, other: &Self) -> BoolVec;
    
    /**
     * Component-wise compare of `self <= other`
     */
    pure fn less_than_equal(&self, other: &Self) -> BoolVec;
    
    /**
     * Component-wise compare of `self > other`
     */
    pure fn greater_than(&self, other: &Self) -> BoolVec;
    
    /**
     * Component-wise compare of `self >= other`
     */
    pure fn greater_than_equal(&self, other: &Self) -> BoolVec;
}

/**
 * Component-wise equality comparison methods
 *
 * The methods contained in this trait correspond to the relational functions
 * mentioned in Section 8.7 of the [GLSL 4.30.6 specification]
 * (http://www.opengl.org/registry/doc/GLSLangSpec.4.30.6.pdf).
 */
pub trait EquableVector<T, BoolVec>: Vector<T> {
    /**
     * Component-wise compare of `self == other`
     */
    pure fn equal(&self, other: &Self) -> BoolVec;
    
    /**
     * Component-wise compare of `self != other`
     */
    pure fn not_equal(&self, other: &Self) -> BoolVec;
}

/**
 * A vector with boolean components
 *
 * The methods contained in this trait correspond to the relational functions
 * mentioned in Section 8.7 of the [GLSL 4.30.6 specification]
 * (http://www.opengl.org/registry/doc/GLSLangSpec.4.30.6.pdf).
 */
pub trait BooleanVector: Vector<bool> {
    /**
     * # Return value
     *
     * `true` if of any component is `true`
     */
    pure fn any(&self) -> bool;
    
    /**
     * # Return value
     *
     * `true` only if all components are `true`
     */
    pure fn all(&self) -> bool;
    
    /**
     * # Return value
     *
     * the component-wise logical complement
     */
    pure fn not(&self) -> Self;
}

pub trait TrigVec<T>: Vector<T> {
    pure fn radians(&self) -> Self;
    pure fn degrees(&self) -> Self;
    
    // Triganometric functions
    pure fn sin(&self)                      -> Self;
    pure fn cos(&self)                      -> Self;
    pure fn tan(&self)                      -> Self;
    
    // Inverse triganometric functions
    pure fn asin(&self)                     -> Self;
    pure fn acos(&self)                     -> Self;
    pure fn atan(&self)                     -> Self;
    pure fn atan2(&self, other: Self)       -> Self;
    
    // Hyperbolic triganometric functions
    pure fn sinh(&self)                     -> Self;
    pure fn cosh(&self)                     -> Self;
    pure fn tanh(&self)                     -> Self;
    // pure fn asinh()                      -> Self;
    // pure fn acosh()                      -> Self;
    // pure fn atanh()                      -> Self;
}

pub trait ExpVec<T>: Vector<T> {
    // Exponential functions
    pure fn pow_t(&self, n: Self)           -> Self;
    pure fn pow_v(&self, n: T)              -> Self;
    pure fn exp(&self)                      -> Self;
    pure fn exp2(&self)                     -> Self;
    pure fn ln(&self)                       -> Self;
    pure fn ln2(&self)                      -> Self;
    pure fn sqrt(&self)                     -> Self;
    pure fn inv_sqrt(&self)                 -> Self;
}

pub trait ApproxVec<T>: Vector<T> {
    // Whole-number approximation functions
    pure fn floor(&self)                    -> Self;
    pure fn trunc(&self)                    -> Self;
    pure fn round(&self)                    -> Self;
    // pure fn round_even(&self)            -> Self;
    pure fn ceil(&self)                     -> Self;
    pure fn fract(&self)                    -> Self;
}

pub trait SignedVec<T,BV>: Vector<T> {
    pure fn is_positive(&self)    -> BV;
    pure fn is_negative(&self)    -> BV;
    pure fn is_nonpositive(&self) -> BV;
    pure fn is_nonnegative(&self) -> BV;
    
    pure fn abs(&self) -> Self;
    pure fn sign(&self) -> Self;
    pure fn copysign(&self, other: Self) -> Self;
}

pub trait ExtentVec<T>: Vector<T> {
    pure fn min_v(&self, other: &Self) -> Self;
    pure fn max_v(&self, other: &Self) -> Self;
    pure fn clamp_v(&self, mn: &Self, mx: &Self) -> Self;
    
    pure fn min_t(&self, other: T) -> Self;
    pure fn max_t(&self, other: T) -> Self;
    pure fn clamp_t(&self, mn: T, mx: T) -> Self;
}

pub trait MixVec<T>: Vector<T> {
    // Functions for blending numbers together
    pure fn mix(&self, other: Self, value: Self) -> Self;
    pure fn smooth_step(&self, edge0: Self, edge1: Self) -> Self;
    pure fn step(&self, edge: Self) -> Self;
}