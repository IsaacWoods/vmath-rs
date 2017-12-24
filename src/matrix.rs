/*
 * Copyright (C) 2017, Isaac Woods.
 * See LICENCE.md
 */

macro_rules! impl_matrix
{
    ($name : ident, $vec : ident, $($component : ident),+) =>
    {
        use $crate::vec::$vec;

        pub struct $name
        {
            $(
                $component : $vec,
             )+
        }

        impl $name
        {
            pub fn new($($component : $vec,)+) -> $name
            {
                $name
                {
                    $(
                        $component : $component,
                     )+
                }
            }
        }
    }
}

impl_matrix!(Mat2, Vec2, x, y);
impl_matrix!(Mat3, Vec3, x, y, z);
impl_matrix!(Mat4, Vec4, x, y, z, w);
