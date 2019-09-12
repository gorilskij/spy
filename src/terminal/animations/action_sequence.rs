use crate::terminal::animations::image::Image;
use std::ops::RangeInclusive;
use std::thread::sleep;
use std::time::Duration;
use crate::terminal::animations::stage::Stage;
use std::cmp::min;

fn blank(len: usize) -> Image {
    vec!["".to_string(); len].into()
}

fn from_iter(len: usize, iter: impl Iterator<Item=impl Into<String>>) -> Image {
    iter.take(len).map(m!(into)).collect::<Vec<_>>().into()
}

fn frame_sleep() {
    sleep(Duration::from_millis(50));
}

pub enum Action {
    Animate {
        repeat: RangeInclusive<usize>,
        f: Box<dyn Fn(usize, &Image) -> Image>,
        rev: bool,
    },
    Wait(Duration),
    Prompt(String),
}

macro_rules! animate {
    ($range: expr, $f: expr) => {
        Action::Animate { repeat: $range, f: Box::new($f), rev: false }
    };
    (rev $range: expr, $f: expr) => {
        Action::Animate { repeat: $range, f: Box::new($f), rev: true }
    }
}

pub struct ActionSequence<'a> {
    image: &'a Image,
    actions: Vec<Action>,
}

impl<'a> ActionSequence<'a> {
    pub fn build_with(image: &Image) -> Builder {
        Builder::new(image)
    }

    pub fn execute(&self, stage: &Stage) {
        for action in &self.actions {
            match action {
                Action::Animate {
                    repeat,
                    f,
                    rev,
                } => {
                    let paint = |i| {
                        stage.replace(&(*f)(i, &self.image));
                        frame_sleep();
                    };

                    if *rev {
                        for i in repeat.clone().rev() { paint(i) }
                    } else {
                        for i in repeat.clone() { paint(i) }
                    }
                },
                Action::Wait(duration) => sleep(*duration),
                Action::Prompt(text) => stage.prompt(text),
            }
        }
    }
}

pub struct Builder<'a> {
    image: &'a Image,
    actions: Vec<Action>,
    fixed_bottom: usize,
}

impl<'a> Builder<'a> {
    fn new(image: &'a Image) -> Self {
        Self {
            image,
            actions: vec![],
            fixed_bottom: 0,
        }
    }

    pub fn fix_bottom(mut self, lines: usize) -> Self {
        self.fixed_bottom = lines;
        self
    }

    pub fn unfix_all(mut self) -> Self {
        self.fixed_bottom = 0;
        self
    }

    pub fn in_from_below(mut self) -> Self {
        self.actions.push(if self.fixed_bottom == 0 {
            animate!(
                0..=self.image.len(),
                |nl, im| blank(im.len()-nl) + from_iter(nl, im.lines())
            )
        } else {
            let fb = self.fixed_bottom;
            animate!(
                0..=self.image.len()-fb,
                move |nl, im| blank(im.len()-fb-nl) + from_iter(nl, im.lines())
                    + blank(fb - min(fb, nl)) + from_iter(min(fb, nl), im.lines().skip(im.len() - fb))
            )
        });
        self
    }

    pub fn out_to_below(mut self) -> Self {
        self.actions.push(if self.fixed_bottom == 0 {
            animate!(
                rev 0..=self.image.len(),
                |nl, im| blank(im.len() - nl) + from_iter(nl, im.lines())
            )
        } else {
            let fb = self.fixed_bottom;
            animate!(
                rev 0..=self.image.len()-fb,
                move |nl, im| blank(im.len()-fb-nl) + from_iter(nl, im.lines())
                    + blank(fb - min(fb, nl)) + from_iter(min(fb, nl), im.lines().skip(im.len() - fb))
            )
        });
        self
    }

    pub fn wait_millis(mut self, millis: u64) -> Self {
        self.actions.push(Action::Wait(Duration::from_millis(millis)));
        self
    }

    pub fn prompt(mut self, text: impl Into<String>) -> Self {
        self.actions.push(Action::Prompt(text.into()));
        self
    }

    pub fn build(self) -> ActionSequence<'a> {
        ActionSequence {
            image: self.image,
            actions: self.actions,
        }
    }
}