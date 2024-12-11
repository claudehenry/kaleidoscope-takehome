use std::sync::{Arc, RwLock};

mod shape;
use shape::*;

type Handle<S> = Arc<RwLock<S>>;

struct Canvas {
    // the downside of this handle / storage solution is its triple indirection, necessary though
    // it is to allow individual shape manipulation and heterogeneous storage
    shapes: Vec<Handle<dyn Shape>>,
}

impl Canvas {
    fn new() -> Self {
        Canvas { shapes: Vec::new() }
    }

    fn add<S: Shape + 'static>(&mut self, shape: S) -> Handle<S> {
        let shape = Arc::new(RwLock::from(shape));
        self.shapes.push(shape.clone());
        shape
    }

    fn get_area<S: Shape>(&self, shape: &Handle<S>) -> Option<f64> {
        shape.read().ok().map(|s| s.get_area())
    }

    fn get_origin<S: Shape>(&self, shape: &Handle<S>) -> Option<(f64, f64)> {
        shape.read().ok().map(|s| s.origin())
    }

    fn set_origin<S: Shape>(&self, shape: &Handle<S>, origin: (f64, f64)) {
        // todo: possibly useful to return a Result here indicating whether the shape existed and
        // was modified
        if let Some(mut s) = shape.write().ok() {
            s.set_origin(origin)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::seq::SliceRandom;
    use rand::thread_rng;
    use rand::Rng;
    use std::thread;

    #[test]
    fn shared_handle() {
        let mut canvas = Canvas::new();
        let circle = canvas.add(Circle {
            radius: 5.0,
            origin: (10.0, 10.0),
        });
        let rectangle = canvas.add(Rectangle {
            width: 4.0,
            height: 6.0,
            origin: (20.0, 20.0),
        });
        let triangle = canvas.add(Triangle {
            base: 3.0,
            height: 4.0,
            origin: (30.0, 30.0),
        });

        assert_eq!(circle.read().unwrap().origin(), (10.0, 10.0));
        assert_eq!(rectangle.read().unwrap().origin(), (20.0, 20.0));
        assert_eq!(triangle.read().unwrap().origin(), (30.0, 30.0));

        circle.write().unwrap().origin = (11.0, 11.0);
        assert_eq!(canvas.get_origin(&circle), Some((11.0, 11.0)));

        rectangle.write().unwrap().origin = (21.0, 21.0);
        assert_eq!(canvas.get_origin(&rectangle), Some((21.0, 21.0)));

        canvas.set_origin(&triangle, (31.0, 31.0));
        assert_eq!(canvas.get_origin(&triangle), Some((31.0, 31.0)));
    }

    #[test]
    fn concurrent_access() {
        let mut canvas = Canvas::new();
        let handles = (0..100)
            .map(|_| {
                canvas.add(Rectangle {
                    width: 4.0,
                    height: 5.0,
                    origin: (10.0, 10.0),
                })
            })
            .collect::<Vec<_>>();

        let handle1 = thread::spawn({
            let mut handles = handles.clone();
            move || {
                handles.shuffle(&mut thread_rng());

                for handle in handles {
                    handle.write().unwrap().set_origin((0.0, 0.0));
                }
            }
        });

        let handle2 = thread::spawn({
            let mut handles = handles.clone();
            move || {
                handles.shuffle(&mut thread_rng());

                for handle in handles {
                    handle.write().unwrap().set_origin((0.0, 0.0));
                }
            }
        });

        handle1.join().unwrap();
        handle2.join().unwrap();

        for handle in handles {
            assert_eq!(canvas.get_origin(&handle), Some((0.0, 0.0)));
        }
    }

    #[test]
    fn internal_layout() {
        // not a fan of unit tests that 'know' implementation details, in this case the Canvas'
        // buffer, so this is not as much a test as an illustration of handle tracking and uniqueness
        let mut canvas = Canvas::new();

        // adds 100 identical shapes, tracks a single shape
        let (single_handle, index) = {
            let mut single_handle = None;
            let range = 0..100;
            let index = rand::thread_rng().gen_range(range.clone());

            for i in range {
                let handle = canvas.add(Circle {
                    radius: 5.0,
                    origin: (10.0, 10.0),
                });

                if i == index {
                    single_handle = Some(handle.clone());
                }
            }

            (single_handle.expect("index should be in range"), index)
        };

        // mutate the tracked shape
        canvas.set_origin(&single_handle, (20.0, 20.0));
        assert_eq!(canvas.get_origin(&single_handle), Some((20.0, 20.0)));

        // check the layout of the canvas buffer reflects the
        for (i, handle) in canvas.shapes.iter().enumerate() {
            assert_eq!(
                handle.read().unwrap().origin(),
                if i == index {
                    (20.0, 20.0)
                } else {
                    (10.0, 10.0)
                }
            );
        }
    }
}

fn main() {}
