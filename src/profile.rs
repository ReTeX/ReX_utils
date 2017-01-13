#![feature(test)]

extern crate rex;
extern crate json;
extern crate test;

use json::JsonValue;
use rex::render::*;
use rex::dimensions::{Pixels, Float};

fn profile_svg() {
    println!("SVG Renderer");
    if let JsonValue::Array(examples) =
        json::parse(include_str!("../examples.json"))
        .expect("failed to parse examples.json")
    {
        for case in &examples {
            let title = case["title"].as_str().unwrap();
            let tex = case["latex"].as_str().expect("'tex' is not a string");
            
            let samples = test::bench::benchmark(|b| {
                b.iter(move || -> String {
                    SVGRenderer::new(&RenderSettings::default()).render(tex)
                });
            });
            println!("{:50} {}", title, test::fmt_bench_samples(&samples));
        }
    }
}

enum Item {
    Symbol(Float, Float, u32, Float),
    Rule(Float, Float, Float, Float)
}

struct FakeRenderer<'a> {
    settings: &'a RenderSettings
}

impl<'a> FakeRenderer<'a> {
    fn new(settings: &'a RenderSettings) -> FakeRenderer<'a> {
        FakeRenderer {
            settings: settings
        }
    }
}
impl<'a> Renderer for FakeRenderer<'a> {
    type Out = Vec<Item>;
    
    fn settings(&self) -> &RenderSettings {
        self.settings
    }
    
    fn symbol(&self, out: &mut Vec<Item>, pos: Cursor, symbol: u32, scale: Float) {
        out.push(Item::Symbol(*pos.x, *pos.y, symbol, scale));
    }
    
    fn rule(&self, out: &mut Vec<Item>, pos: Cursor, width: Pixels, height: Pixels) {
        out.push(Item::Rule(*pos.x, *pos.y, *width, *height));
    }

    fn color<F>(&self, out: &mut Vec<Item>, _color: &str, mut contents: F)
        where F: FnMut(&Self, &mut Self::Out)
    {
        contents(self, out);
    }
}

fn profile_fake() {
    println!("Fake Renderer");
    if let JsonValue::Array(examples) =
        json::parse(include_str!("../examples.json"))
        .expect("failed to parse examples.json")
    {
        for case in &examples {
            let title = case["title"].as_str().unwrap();
            let tex = case["latex"].as_str().expect("'tex' is not a string");
            
            let samples = test::bench::benchmark(|b| {
                b.iter(move || {
                    FakeRenderer::new(&RenderSettings::default())
                    .render(tex)
                });
            });
            println!("{:50} {}", title, test::fmt_bench_samples(&samples));
        }
    }
}

fn main() {
    profile_fake();
    println!();
    profile_svg();
}
