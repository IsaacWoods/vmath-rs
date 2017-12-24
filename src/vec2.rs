/*
 * Copyright (C) 2017, Isaac Woods.
 * See LICENCE.md
 */

use std::ops;

#[derive(Clone,Copy)]
pub struct Vec2
{
    x : f32,
    y : f32,
}

impl Vec2
{
    pub fn new(x : f32, y : f32) -> Vec2
    {
        Vec2
        {
            x : x,
            y : y,
        }
    }
}

impl ops::Add<Vec2> for Vec2
{
    type Output = Vec2;

    fn add(self, rhs : Vec2) -> Vec2
    {
        Vec2
        {
            x : self.x + rhs.x,
            y : self.y + rhs.y,
        }
    }
}

impl ops::Sub<Vec2> for Vec2
{
    type Output = Vec2;

    fn sub(self, rhs : Vec2) -> Vec2
    {
        Vec2
        {
            x : self.x - rhs.x,
            y : self.y - rhs.y,
        }
    }
}

impl ops::Mul<Vec2> for Vec2
{
    type Output = Vec2;

    fn mul(self, rhs : Vec2) -> Vec2
    {
        Vec2
        {
            x : self.x * rhs.x,
            y : self.y * rhs.y,
        }
    }
}

impl ops::Div<Vec2> for Vec2
{
    type Output = Vec2;

    fn div(self, rhs : Vec2) -> Vec2
    {
        Vec2
        {
            x : self.x / rhs.x,
            y : self.y / rhs.y,
        }
    }
}

impl ops::Neg for Vec2
{
    type Output = Vec2;

    fn neg(self) -> Vec2
    {
        Vec2
        {
            x : -self.x,
            y : -self.y,
        }
    }
}
