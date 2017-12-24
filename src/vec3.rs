/*
 * Copyright (C) 2017, Isaac Woods.
 * See LICENCE.md
 */

use std::ops;

#[derive(Clone,Copy)]
pub struct Vec3
{
    x : f32,
    y : f32,
    z : f32,
}

impl Vec3
{
    pub fn new(x : f32, y : f32, z : f32) -> Vec3
    {
        Vec3
        {
            x : x,
            y : y,
            z : z,
        }
    }
}

impl ops::Add<Vec3> for Vec3
{
    type Output = Vec3;

    fn add(self, rhs : Vec3) -> Vec3
    {
        Vec3
        {
            x : self.x + rhs.x,
            y : self.y + rhs.y,
            z : self.z + rhs.z,
        }
    }
}

impl ops::Sub<Vec3> for Vec3
{
    type Output = Vec3;

    fn sub(self, rhs : Vec3) -> Vec3
    {
        Vec3
        {
            x : self.x - rhs.x,
            y : self.y - rhs.y,
            z : self.z - rhs.z,
        }
    }
}

impl ops::Mul<Vec3> for Vec3
{
    type Output = Vec3;

    fn mul(self, rhs : Vec3) -> Vec3
    {
        Vec3
        {
            x : self.x * rhs.x,
            y : self.y * rhs.y,
            z : self.z * rhs.z,
        }
    }
}

impl ops::Div<Vec3> for Vec3
{
    type Output = Vec3;

    fn div(self, rhs : Vec3) -> Vec3
    {
        Vec3
        {
            x : self.x / rhs.x,
            y : self.y / rhs.y,
            z : self.z / rhs.z,
        }
    }
}

impl ops::Neg for Vec3
{
    type Output = Vec3;

    fn neg(self) -> Vec3
    {
        Vec3
        {
            x : -self.x,
            y : -self.y,
            z : -self.z,
        }
    }
}
