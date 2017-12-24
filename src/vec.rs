/*
 * Copyright (C) 2017, Isaac Woods.
 * See LICENCE.md
 */

use std::ops;

macro_rules! impl_vector
{
    ($name : ident, $($component : ident),+) =>
    {
        #[derive(Clone,Copy,Debug,PartialEq)]
        pub struct $name
        {
            $(
                pub $component : f32,
             )+
        }

        impl $name
        {
            pub fn new($($component : f32,)+) -> $name
            {
                $name
                {
                    $(
                        $component : $component,
                     )+
                }
            }
        }

        impl ops::Add<$name> for $name
        {
            type Output = $name;

            fn add(self, rhs : $name) -> $name
            {
                $name
                {
                    $(
                        $component : self.$component + rhs.$component,
                     )+
                }
            }
        }

        impl ops::Sub<$name> for $name
        {
            type Output = $name;
        
            fn sub(self, rhs : $name) -> $name
            {
                $name
                {
                    $(
                        $component : self.$component - rhs.$component,
                     )+
                }
            }
        }
        
        impl ops::Mul<$name> for $name
        {
            type Output = $name;
        
            fn mul(self, rhs : $name) -> $name
            {
                $name
                {
                    $(
                        $component : self.$component * rhs.$component,
                     )+
                }
            }
        }
        
        impl ops::Div<$name> for $name
        {
            type Output = $name;
        
            fn div(self, rhs : $name) -> $name
            {
                $name
                {
                    $(
                        $component : self.$component / rhs.$component,
                     )+
                }
            }
        }
        
        impl ops::Neg for $name
        {
            type Output = $name;
        
            fn neg(self) -> $name
            {
                $name
                {
                    $(
                        $component : -self.$component,
                     )+
                }
            }
        }
    }
}

impl_vector!(Vec2, x, y);
impl_vector!(Vec3, x, y, z);
impl_vector!(Vec4, x, y, z, w);

#[cfg(test)]
mod tests
{
    use super::{Vec2,Vec3,Vec4};

    #[test]
    fn test_vec2()
    {
        let v = Vec2::new(1.0, 2.0);
        assert_eq!(v.x, 1.0);
        assert_eq!(v.y, 2.0);
        assert_eq!(-v, Vec2::new(-1.0, -2.0));

        let u = Vec2::new(10.0, 11.0);
        assert_eq!(v + u, Vec2::new(11.0,       13.0));
        assert_eq!(v - u, Vec2::new(-9.0,       -9.0));
        assert_eq!(v * u, Vec2::new(10.0,       22.0));
        assert_eq!(v / u, Vec2::new(1.0/10.0,   2.0/11.0));
    }

    #[test]
    fn test_vec3()
    {
        let v = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(v.x, 1.0);
        assert_eq!(v.y, 2.0);
        assert_eq!(v.z, 3.0);
        assert_eq!(-v, Vec3::new(-1.0, -2.0, -3.0));

        let u = Vec3::new(10.0, 11.0, 12.0);
        assert_eq!(v + u, Vec3::new(11.0,       13.0,       15.0));
        assert_eq!(v - u, Vec3::new(-9.0,       -9.0,       -9.0));
        assert_eq!(v * u, Vec3::new(10.0,       22.0,       36.0));
        assert_eq!(v / u, Vec3::new(1.0/10.0,   2.0/11.0,   3.0/12.0));
    }

    #[test]
    fn test_vec4()
    {
        let v = Vec4::new(1.0, 2.0, 3.0, 4.0);
        assert_eq!(v.x, 1.0);
        assert_eq!(v.y, 2.0);
        assert_eq!(v.z, 3.0);
        assert_eq!(v.w, 4.0);
        assert_eq!(-v, Vec4::new(-1.0, -2.0, -3.0, -4.0));

        let u = Vec4::new(10.0, 11.0, 12.0, 13.0);
        assert_eq!(v + u, Vec4::new(11.0,       13.0,       15.0,       17.0));
        assert_eq!(v - u, Vec4::new(-9.0,       -9.0,       -9.0,       -9.0));
        assert_eq!(v * u, Vec4::new(10.0,       22.0,       36.0,       52.0));
        assert_eq!(v / u, Vec4::new(1.0/10.0,   2.0/11.0,   3.0/12.0,   4.0/13.0));
    }
}
