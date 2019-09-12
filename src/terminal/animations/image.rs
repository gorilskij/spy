use std::ops::{Deref, Add};
use std::vec;
use std::slice;

pub struct Image(Vec<String>);

impl From<Vec<String>> for Image {
    fn from(vec: Vec<String>) -> Self {
        Self(vec)
    }
}

impl From<String> for Image {
    fn from(string: String) -> Self {
        Self(string.split("\n").map(m!(into)).collect())
    }
}

impl<'a> From<&'a str> for Image {
    fn from(s: &str) -> Self {
        Self::from(s.to_string())
    }
}

impl Deref for Image {
    type Target = Vec<String>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl IntoIterator for Image {
    type Item = String;
    type IntoIter = vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> IntoIterator for &'a Image {
    type Item = &'a String;
    type IntoIter = slice::Iter<'a, String>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl Add<Image> for Image {
    type Output = Image;

    fn add(self, mut rhs: Image) -> Self::Output {
        let mut vec = self.0;
        vec.append(&mut rhs.0);
        Self(vec)
    }
}

impl Image {
    pub fn from_vec(vec: Vec<impl Into<String>>) -> Self {
        Self(vec.into_iter().map(m!(into)).collect())
    }

    pub fn from_str(s: impl Into<String>) -> Self {
        Self(s.into().split("\n").map(m!(into)).collect())
    }

    pub fn lines(&self) -> impl Iterator<Item=&String> {
        self.0.iter()
    }
}