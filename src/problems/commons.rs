use std::collections::VecDeque;
use std::env::consts::ARCH;
use std::fmt::Debug;
use std::hash::{Hash};
use std::num::NonZero;
use std::ops::{Add, Div, Mul, Sub};
use std::thread::available_parallelism;
use ascii::{AsAsciiStr, AsciiChar, AsciiStr, AsciiString};
//use ascii::*;

#[derive(Clone)]
pub struct CharGrid<T> where T: AsRef<AsciiStr> {
    pub chars: T,
    pub bounds: [usize; 2],
    pub newline_lengh: usize,
}



impl<T: AsRef<AsciiStr>> CharGrid<T> {

    pub fn vec_index_to_uquard(&self, index: usize) -> Ucoord {
        Ucoord(index % (self.bounds[0]+self.newline_lengh), index / (self.bounds[0]+self.newline_lengh))
    }

    #[allow(dead_code)]
    pub fn debug_print(&self) {
        //#[cfg(test)]
        {
            println!();
            for y in 0..self.bounds[1] {
                for x in 0..self.bounds[0] {
                    print!("{}", { self.chars.as_ref()[y*(self.bounds[0]) + x] });
                }
                println!();
            }
        }
    }


}

impl CharGrid<AsciiString> {
    pub fn index(&self, quard: Ucoord) -> Option<&AsciiChar> {
        self.index_usize(quard).map(|index| &self.chars[index])
    }

    pub fn index_usize(&self, quard: Ucoord) -> Option<usize> {
        if quard.0>=self.bounds[0] || quard.1>=self.bounds[1] {None} else {Some(quard.1*self.bounds[0] + quard.0)}
    }
    pub fn index_mut(&mut self, quard: Ucoord) -> Option<&mut AsciiChar> {
        self.index_usize(quard).map(|index| &mut self.chars[index])
    }
    pub fn new(input: &str) -> CharGrid<AsciiString> {
        let input = input.as_ascii_str().unwrap();
        let mut chars = AsciiString::with_capacity(input.len());
        chars.extend(input.chars().filter(|char| !(char ==&'\n'|| char ==&'\r')));
        CharGrid {
            bounds: [input.lines().next().unwrap().chars().count(), input.lines().count()],
            chars,
            newline_lengh: 0,
        }
    }
}

impl CharGrid<&AsciiStr> {
    pub fn index(&self, quard: Ucoord) -> Option<&AsciiChar> {
        self.index_usize(quard).map(|index| &self.chars[index])
    }

    pub fn index_usize(&self, quard: Ucoord) -> Option<usize> {
        if quard.0>=self.bounds[0] || quard.1>=self.bounds[1] {None} else {Some(quard.1*(self.bounds[0]+self.newline_lengh) + quard.0)}
    }
    pub fn new(input: &str) -> CharGrid<&AsciiStr> {
        let input = input.as_ascii_str().unwrap();
        //let mut chars = AsciiString::with_capacity(input.len());
        //chars.extend(input.chars().filter(|char| !(char ==&'\n'|| char ==&'\r')));
        let mut newline_lengh = 0;
        if input.chars().any(|char| char == '\n') {newline_lengh += 1}
        if input.chars().any(|char| char == '\r') {newline_lengh += 1}
        CharGrid {
            bounds: [input.lines().next().unwrap().chars().count(), input.lines().count()],
            chars: input,
            newline_lengh,
        }
    }
}

pub struct VecGrid<T: Default + Debug> {
    pub bounds: [usize; 2],
    pub vec: Vec<T>,
}

impl <T: Default + Debug> VecGrid<T> {
    pub fn new(bounds: [usize; 2]) -> VecGrid<T> {
        let mut vec = Vec::with_capacity(bounds[0]*bounds[1]);
        vec.extend((0..bounds[0]*bounds[1]).map(|_|T::default()));
        assert_eq!(vec.capacity(), vec.len());
        VecGrid {
            bounds,
            vec
        }
    }

    pub fn from_iter<I>(width:usize, iter: I, capacity_hint: usize) -> VecGrid<T>
    where
        I: IntoIterator<Item=T>,
    {
        let mut vec = Vec::with_capacity(capacity_hint);
        vec.extend(iter);
        assert_eq!(vec.len() % width, 0);
        VecGrid{
            bounds:[width,vec.len()/width],
            vec
        }
    }
    #[allow(dead_code)]
    pub fn index(&self, quard: Ucoord) -> Option<&T> {
        self.index_usize(quard).map(|index| &self.vec[index])
    }

    #[allow(dead_code)]
    pub fn index_usize(&self, quard: Ucoord) -> Option<usize> {
        if quard.0>=self.bounds[0] || quard.1>=self.bounds[1] {None} else {Some(quard.1*self.bounds[0] + quard.0)}
    }
    pub fn index_mut(&mut self, quard: Ucoord) -> Option<&mut T> {
        self.index_usize(quard).map(|index| &mut self.vec[index])
    }
    #[allow(dead_code)]
    pub fn vec_index_to_uquard(&self, index: usize) -> Ucoord {
        Ucoord(index % self.bounds[0], index / self.bounds[0])
    }
    #[allow(dead_code)]
    pub fn debug_print(&self) {
        //#[cfg(test)]
        {
            println!();
            for y in 0..self.bounds[1] {
                for x in 0..self.bounds[0] {
                    print!("{:?}", &self.vec[y*(self.bounds[0]) + x]);
                }
                println!();
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Default)]
pub struct Ucoord(pub usize, pub usize);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Default)]
pub struct Icoord(pub i32, pub i32);

impl Add for Ucoord {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Ucoord(self.0+rhs.0, self.1+rhs.1)
    }

}

impl Mul<usize> for Ucoord {
    type Output = Self;
    fn mul(self, rhs: usize) -> Self::Output {
        Ucoord(self.0*rhs, self.1*rhs)
    }

}

// impl Hash for Uquard {
//     fn hash<H: Hasher>(&self, state: &mut H) {
//         (self.0*(self.1*1000)).hash(state);
//     }
// }

impl Sub for Ucoord {
    type Output = Option<Self>;
    fn sub(self, rhs: Self) -> Self::Output {
        if self.0>=rhs.0 && self.1>=rhs.1 {
            Some(Ucoord(self.0-rhs.0, self.1-rhs.1))
        } else { None }
    }
}

// impl PartialEq<Self> for Uquard {
//     fn eq(&self, other: &Self) -> bool {
//         self.0==other.0 && self.1==other.1
//     }
// }

impl Add for Icoord {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Icoord(self.0+rhs.0, self.1+rhs.1)
    }
}
impl Sub for Icoord {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Icoord(self.0-rhs.0, self.1-rhs.1)
    }
}
impl Div<i32> for Icoord {
    type Output = Self;
    fn div(self, rhs: i32) -> Self::Output {
        Icoord(self.0/rhs, self.1/rhs)
    }
}
impl From<&Ucoord> for Icoord {
    fn from(u: &Ucoord) -> Self {
        Icoord(u.0 as i32, u.1 as i32)
    }
}
impl From<Icoord> for Option<Ucoord> {
    fn from(i: Icoord) -> Self {
        if i.0>= 0 && i.1>= 0 {Some(Ucoord(i.0 as usize, i.1 as usize))} else {None}
    }
}

#[derive(Copy, Clone, Hash, Eq, PartialEq)]//TODO:try hash as u32
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct u24([u8; 3]);
impl Add for u24 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self::from_u32(self.to_u32() + rhs.to_u32())
    }
}

impl Mul for u24 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self::from_u32(self.to_u32() * rhs.to_u32())
    }
}
impl Mul<u32> for u24 {
    type Output = Self;
    fn mul(self, rhs: u32) -> Self::Output {
        Self::from_u32(self.to_u32() * rhs)
    }
}
impl u24 {
    fn to_u32(self) -> u32 {
        let u24([a, b, c]) = self;
        u32::from_le_bytes([a, b, c, 0])
    }
    fn from_u32(n: u32) -> Self {
        let [a, b, c, d] = n.to_le_bytes();
        debug_assert!(d == 0);
        u24([a, b, c])
    }
}
impl From<u32> for u24 {
    fn from(n: u32) -> Self {
        Self::from_u32(n)
    }
}
impl From<u24> for u32 {
    fn from(n: u24) -> Self {
        n.to_u32()
    }
}

#[allow(dead_code)]
pub fn get_avalible_phsical_parralelism() -> usize {
    let physical = NonZero::new(num_cpus::get_physical()).unwrap_or_else(|| NonZero::new(1).unwrap());
    let paral = available_parallelism().unwrap_or_else(|_| NonZero::new(1).unwrap());
    let out = if paral > physical {
        physical.get()
    } else if ARCH == "x86_64" && paral.get() > 1 {
        paral.get()/2
    } else if ARCH == "powerpc64" {
        panic!("put powerpc hyperthreading here");
    } else { 
        paral.get()
    };
    //println!("{},{},{}",physical.get(),paral.get(),out);
    out
}

pub struct EnumeratedVecDeque<T>{
    vec: VecDeque<T>,
    popped_from_front_count: usize,
}

#[allow(dead_code)]
impl<T> EnumeratedVecDeque<T> {
    pub fn new(vec: VecDeque<T>) -> Self {
        EnumeratedVecDeque {vec, popped_from_front_count: 0}
    }
    pub fn front(&self) -> Option<(usize, &T)> {
        Some((
            self.popped_from_front_count,
            self.vec.front()?
        ))
    }
    pub fn back(&self) -> Option<(usize,&T)> {
        Some((
            self.popped_from_front_count + self.vec.len() - 1,
            self.vec.back()?
        ))
    }
    pub fn front_mut(&mut self) -> Option<(usize,&mut T)> {
        Some((
            self.popped_from_front_count,
            self.vec.front_mut()?
        ))
    }
    pub fn back_mut(&mut self) -> Option<(usize,&mut T)> {
        Some((
            self.popped_from_front_count + self.vec.len() - 1,
            self.vec.back_mut()?
        ))
    }
    pub fn pop_front(&mut self) -> Option<(usize, T)> {
        self.popped_from_front_count += 1;
        Some((
            self.popped_from_front_count - 1,
            self.vec.pop_front()?
        ))
    }
    pub fn pop_back(&mut self) -> Option<(usize,T)> {
        Some((
            self.popped_from_front_count + self.vec.len() - 1,
            self.vec.pop_back()?
        ))
    }
    pub fn len(&self) -> usize {
        self.vec.len()
    }
}