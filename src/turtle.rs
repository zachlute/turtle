use radians::{self, Radians};
use turtle_window::TurtleWindow;
use event::MouseButton;
use {Speed, Color, Event};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AngleUnit {
    Degrees,
    Radians,
}

impl AngleUnit {
    fn to_radians(&self, angle: Angle) -> Radians {
        match *self {
            AngleUnit::Degrees => Radians::from_degrees_value(angle),
            AngleUnit::Radians => Radians::from_radians_value(angle),
        }
    }

    fn to_angle(&self, angle: Radians) -> Angle {
        match *self {
            AngleUnit::Degrees => angle.to_degrees(),
            AngleUnit::Radians => angle.to_radians(),
        }
    }
}

/// A point in 2D space: [x, y]
///
/// ```rust
/// # extern crate turtle;
/// # use turtle::Point;
/// # fn main() {
/// let p: Point = [100., 120.];
/// // get x coordinate
/// let x = p[0];
/// assert_eq!(x, 100.);
/// // get y coordinate
/// let y = p[1];
/// assert_eq!(y, 120.);
/// # }
pub type Point = [f64; 2];

/// Any distance value
pub type Distance = f64;

/// An angle value without a unit
///
/// The unit of the angle represented by this value depends on what
/// unit the Turtle was set to when this angle was retrieved
pub type Angle = f64;

/// A turtle with a pen attached to its tail
pub struct Turtle {
    window: TurtleWindow,
    angle_unit: AngleUnit,
}

impl Turtle {
    /// Initialize a new Turtle instance
    pub fn new() -> Turtle {
        Turtle {
            window: TurtleWindow::new(),
            angle_unit: AngleUnit::Degrees,
        }
    }

    /// Returns the current speed of the turtle
    pub fn speed(&self) -> Speed {
        self.window.turtle().speed
    }

    /// Returns the turtle's current location (x, y)
    pub fn position(&self) -> Point {
        self.window.turtle().position
    }

    /// Returns the turtle's current heading
    ///
    /// Units are by default degrees, but can be set using the methods
    /// [`Turtle::use_degrees`](struct.Turtle.html#method.use_degrees) or
    /// [`Turtle::use_radians`](struct.Turtle.html#method.use_radians).
    pub fn heading(&self) -> Angle {
        let heading = self.window.turtle().heading;
        self.angle_unit.to_angle(heading)
    }

    /// Returns true if the turtle is visible
    pub fn is_visible(&self) -> bool {
        self.window.turtle().visible
    }

    /// Returns true if Angle values will be interpreted as degrees
    pub fn is_using_degrees(&self) -> bool {
        self.angle_unit == AngleUnit::Degrees
    }

    /// Returns true if Angle values will be interpreted as radians
    pub fn is_using_radians(&self) -> bool {
        self.angle_unit == AngleUnit::Radians
    }

    /// Return true if pen is down, false if it’s up.
    pub fn is_pen_down(&self) -> bool {
        self.window.drawing().pen.enabled
    }

    /// Returns the size (thickness) of the pen
    pub fn pen_size(&self) -> f64 {
        self.window.drawing().pen.thickness
    }

    /// Returns the color of the pen
    pub fn pen_color(&self) -> Color {
        self.window.drawing().pen.color
    }

    /// Returns the color of the background
    pub fn background_color(&self) -> Color {
        self.window.drawing().background
    }

    /// Returns the current fill color
    ///
    /// This will be used to fill the shape when `begin_fill()` and `end_fill()` are called.
    //TODO: Hyperlink begin_fill() and end_fill() methods to their docs
    pub fn fill_color(&self) -> Color {
        self.window.drawing().fill_color
    }

    /// Begin filling the shape drawn by the turtle's movements
    pub fn begin_fill(&mut self) {
        self.window.begin_fill();
    }

    /// Stop filling the shape drawn by the turtle's movements
    pub fn end_fill(&mut self) {
        self.window.end_fill();
    }

    /// Pull the pen down so that the turtle draws while moving
    pub fn pen_down(&mut self) {
        self.window.drawing_mut().pen.enabled = true;
    }

    /// Pick the pen up so that the turtle does not draw while moving
    pub fn pen_up(&mut self) {
        self.window.drawing_mut().pen.enabled = false;
    }

    /// Sets the thickness of the pen to the given size
    //TODO: Document this more like set_speed
    pub fn set_pen_size(&mut self, thickness: f64) {
        self.window.drawing_mut().pen.thickness = thickness;
    }

    /// Sets the color of the pen to the given color
    //TODO: Document this more like set_speed
    pub fn set_pen_color<C: Into<Color>>(&mut self, color: C) {
        self.window.drawing_mut().pen.color = color.into();
    }

    /// Sets the color of the background to the given color
    //TODO: Document this more like set_speed
    pub fn set_background_color<C: Into<Color>>(&mut self, color: C) {
        self.window.drawing_mut().background = color.into();
    }

    /// Sets the fill color to the given color
    ///
    /// **Note:** Only the fill color set **before** `begin_fill()` is called will be used to fill
    /// the shape.
    //TODO: Document this more like set_speed
    pub fn set_fill_color<C: Into<Color>>(&mut self, color: C) {
        self.window.drawing_mut().fill_color = color.into();
    }

    /// Set the turtle's movement speed to the given setting. This speed affects the animation of
    /// the turtle's movement and rotation.
    ///
    /// This method's types make it so that it can be called in a number of different ways:
    ///
    /// ```rust,no_run
    /// # extern crate turtle;
    /// # use turtle::*;
    /// # fn main() {
    /// # let mut turtle = Turtle::new();
    /// turtle.set_speed("normal");
    /// turtle.set_speed("fast");
    /// turtle.set_speed(2);
    /// turtle.set_speed(10);
    /// // Directly using a Speed variant works, but the methods above are usually more convenient.
    /// turtle.set_speed(Speed::Six);
    /// # }
    /// ```
    ///
    /// If input is a number greater than 10 or smaller than 1,
    /// speed is set to 0 (`Speed::Instant`). Strings are converted as follows:
    ///
    /// | String      | Value          |
    /// | ----------- | -------------- |
    /// | `"slowest"` | `Speed::One`     |
    /// | `"slow"`    | `Speed::Three`   |
    /// | `"normal"`  | `Speed::Six`     |
    /// | `"fast"`    | `Speed::Eight`   |
    /// | `"fastest"` | `Speed::Ten`     |
    /// | `"instant"` | `Speed::Instant` |
    ///
    /// Anything else will cause the program to `panic!` at runtime.
    ///
    /// ## Moving Instantly
    ///
    /// A speed of zero (`Speed::Instant`) results in no animation. The turtle moves instantly
    /// and turns instantly. This is very useful for moving the turtle from its "home" position
    /// before you start drawing. By setting the speed to instant, you don't have to wait for
    /// the turtle to move into position.
    ///
    /// ## Learning About Conversion Traits
    ///
    /// Using this method is an excellent way to learn about conversion
    /// traits `From` and `Into`. This method takes a *generic type* as its speed parameter. That type
    /// is specified to implement the `Into` trait for the type `Speed`. That means that *any* type
    /// that can be converted into a `Speed` can be passed to this method.
    ///
    /// We have implemented that trait for several types like strings and 32-bit integers so that
    /// those values can be passed into this method.
    /// Rather than calling this function and passing `Speed::Six` directly, you can use just `6`.
    /// Rust will then allow us to call `.into()` as provided by the `Into<Speed>` trait to get the
    /// corresponding `Speed` value.
    ///
    /// You can pass in strings, 32-bit integers, and even `Speed` enum variants because they all
    /// implement the `Into<Speed>` trait.
    pub fn set_speed<S: Into<Speed>>(&mut self, speed: S) {
        self.window.turtle_mut().speed = speed.into();
    }

    /// Delete the turtle's drawings from the screen.
    ///
    /// Do not move turtle. Position and heading of the turtle are not affected.
    pub fn clear(&mut self) {
        self.window.clear();
    }

    /// Makes the turtle invisible. The shell will not be shown, but drawings will continue.
    ///
    /// Useful for some complex drawings.
    pub fn hide(&mut self) {
        self.window.turtle_mut().visible = false;
    }

    /// Makes the turtle visible.
    pub fn show(&mut self) {
        self.window.turtle_mut().visible = true;
    }

    /// Change the angle unit to degrees.
    pub fn use_degrees(&mut self) {
        self.angle_unit = AngleUnit::Degrees;
    }

    /// Change the angle unit to radians.
    pub fn use_radians(&mut self) {
        self.angle_unit = AngleUnit::Radians;
    }

    /// Move the turtle forward by the given amount of `distance`.
    ///
    /// `distance` is given in "pixels" which are like really small turtle steps.
    /// `distance` can be negative in which case the turtle can move backward
    /// using this method.
    pub fn forward(&mut self, distance: Distance) {
        self.window.forward(distance);
    }

    /// Move the turtle backward by the given amount of `distance`.
    ///
    /// `distance` is given in "pixels" which are like really small turtle steps.
    /// `distance` can be negative in which case the turtle can move forwards
    /// using this method.
    pub fn backward(&mut self, distance: Distance) {
        // Moving backwards is essentially moving forwards with a negative distance
        self.window.forward(-distance);
    }

    /// Rotate the turtle right (clockwise) by the given angle.
    ///
    /// Units are by default degrees, but can be set using the methods
    /// [`Turtle::use_degrees`](struct.Turtle.html#method.use_degrees) or
    /// [`Turtle::use_radians`](struct.Turtle.html#method.use_radians).
    pub fn right(&mut self, angle: Angle) {
        let angle = self.angle_unit.to_radians(angle);
        self.window.rotate(angle, true);
    }

    /// Rotate the turtle left (counterclockwise) by the given angle.
    ///
    /// Units are by default degrees, but can be set using the methods
    /// [`Turtle::use_degrees`](struct.Turtle.html#method.use_degrees) or
    /// [`Turtle::use_radians`](struct.Turtle.html#method.use_radians).
    pub fn left(&mut self, angle: Angle) {
        let angle = self.angle_unit.to_radians(angle);
        self.window.rotate(angle, false);
    }

    /// Rotates the turtle to face the given coordinates.
    /// Coordinates are relative to the center of the window.
    ///
    /// If the coordinates are the same as the turtle's current position, no rotation takes place.
    /// Always rotates the least amount necessary in order to face the given point.
    ///
    /// ## UNSTABLE
    /// This feature is currently unstable and completely buggy. Do not use it until it is fixed.
    pub fn turn_towards(&mut self, target: Point) {
        let target_x = target[0];
        let target_y = target[1];

        let position = self.position();
        let x = position[0];
        let y = position[1];

        if (target_x - x).abs() < 0.1 && (target_y - y).abs() < 0.1 {
            return;
        }

        let heading = self.window.turtle().heading;

        let angle = (target_y - y).atan2(target_x - x);
        let angle = Radians::from_radians_value(angle);
        let angle = (angle - heading) % radians::TWO_PI;
        // Try to rotate as little as possible
        let angle = if angle.abs() > radians::PI {
            // Using signum to deal with negative angles properly
            angle.signum()*(radians::TWO_PI - angle.abs())
        }
        else {
            angle
        };
        self.window.rotate(angle, false);
    }

    /// Returns the next event (if any).
    //TODO: Example of usage with an event loop
    pub fn poll_event(&mut self) -> Option<Event> {
        self.window.poll_event()
    }

    /// Convenience function that waits for a click to occur before returning.
    ///
    /// Useful for when you want your program to wait for the user to click before continuing so
    /// that it doesn't start right away.
    pub fn wait_for_click(&mut self) {
        loop {
            if let Some(Event::MouseButtonReleased(MouseButton::Left)) = self.poll_event() {
                break;
            }
        }
    }
}