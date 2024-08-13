use std::ops::Deref;

use super::{Command, Parameters, Position};
use crate::node::Value;
// use crate::parser::{Error, Reader, Result};

/// A [data][1] attribute.
///
/// [1]: https://www.w3.org/TR/SVG/paths.html#PathData
#[derive(Clone, Debug, Default)]
pub struct Data(Vec<Command>);

// struct Parser<'l> {
//     reader: Reader<'l>,
// }

impl Data {
    /// Create a data attribute.
    #[inline]
    pub fn new() -> Self {
        Default::default()
    }

    // /// Parse a data attribute.
    // #[inline]
    // pub fn parse(content: &str) -> Result<Self> {
    //     Parser::new(content).process()
    // }

    /// Add a command.
    #[inline]
    pub fn add(mut self, command: Command) -> Self {
        self.append(command);
        self
    }

    /// Append a command.
    #[inline]
    pub fn append(&mut self, command: Command) {
        self.0.push(command);
    }
}

macro_rules! implement {
    (@one #[$doc:meta] fn $method:ident($command:ident, $position:ident)) => (
        #[$doc]
        pub fn $method<T>(mut self, parameters: T) -> Self
        where
            T: Into<Parameters>,
        {
            self.0.push(Command::$command(Position::$position, parameters.into()));
            self
        }
    );
    (@one #[$doc:meta] fn $method:ident($command:ident)) => (
        #[$doc]
        pub fn $method(mut self) -> Self {
            self.0.push(Command::$command);
            self
        }
    );
    ($(#[$doc:meta] fn $method:ident($($argument:tt)*))*) => (
        impl Data {
            $(implement! { @one #[$doc] fn $method($($argument)*) })*
        }
    );
}

implement! {
    #[doc = "Add an absolute `Command::Move` command."]
    fn move_to(Move, Absolute)

    #[doc = "Add a relative `Command::Move` command."]
    fn move_by(Move, Relative)

    #[doc = "Add an absolute `Command::Line` command."]
    fn line_to(Line, Absolute)

    #[doc = "Add a relative `Command::Line` command."]
    fn line_by(Line, Relative)

    #[doc = "Add an absolute `Command::HorizontalLine` command."]
    fn horizontal_line_to(HorizontalLine, Absolute)

    #[doc = "Add a relative `Command::HorizontalLine` command."]
    fn horizontal_line_by(HorizontalLine, Relative)

    #[doc = "Add an absolute `Command::VerticalLine` command."]
    fn vertical_line_to(VerticalLine, Absolute)

    #[doc = "Add a relative `Command::VerticalLine` command."]
    fn vertical_line_by(VerticalLine, Relative)

    #[doc = "Add an absolute `Command::QuadraticCurve` command."]
    fn quadratic_curve_to(QuadraticCurve, Absolute)

    #[doc = "Add a relative `Command::QuadraticCurve` command."]
    fn quadratic_curve_by(QuadraticCurve, Relative)

    #[doc = "Add an absolute `Command::SmoothQuadraticCurve` command."]
    fn smooth_quadratic_curve_to(SmoothQuadraticCurve, Absolute)

    #[doc = "Add a relative `Command::SmoothQuadraticCurve` command."]
    fn smooth_quadratic_curve_by(SmoothQuadraticCurve, Relative)

    #[doc = "Add an absolute `Command::CubicCurve` command."]
    fn cubic_curve_to(CubicCurve, Absolute)

    #[doc = "Add a relative `Command::CubicCurve` command."]
    fn cubic_curve_by(CubicCurve, Relative)

    #[doc = "Add an absolute `Command::SmoothCubicCurve` command."]
    fn smooth_cubic_curve_to(SmoothCubicCurve, Absolute)

    #[doc = "Add a relative `Command::SmoothCubicCurve` command."]
    fn smooth_cubic_curve_by(SmoothCubicCurve, Relative)

    #[doc = "Add an absolute `Command::EllipticalArc` command."]
    fn elliptical_arc_to(EllipticalArc, Absolute)

    #[doc = "Add a relative `Command::EllipticalArc` command."]
    fn elliptical_arc_by(EllipticalArc, Relative)

    #[doc = "Add a `Command::Close` command."]
    fn close(Close)
}

impl Deref for Data {
    type Target = [Command];

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<Vec<Command>> for Data {
    #[inline]
    fn from(commands: Vec<Command>) -> Self {
        Data(commands)
    }
}

impl From<Data> for Vec<Command> {
    #[inline]
    fn from(Data(commands): Data) -> Self {
        commands
    }
}

impl From<Data> for Value {
    #[inline]
    fn from(Data(mut inner): Data) -> Self {
        inner
            .drain(..)
            .map(String::from)
            .collect::<Vec<_>>()
            .join(" ")
            .into()
    }
}

#[cfg(test)]
mod tests {
    // use crate::node::element::path::data::Parser;
    use crate::node::element::path::{Command, Data, Position};
    use crate::node::Value;

    #[test]
    fn data_append() {
        let mut data = Data::new();
        data.append(Command::Line(Position::Absolute, (1, 2).into()));
        data.append(Command::Close);
        assert_eq!(Value::from(data).to_string(), "L1,2 z");
    }

    #[test]
    fn data_into_value() {
        let data = Data::new()
            .line_to((1, 2))
            .cubic_curve_by((1, 2, 3, 4, 5, 6))
            .close();

        assert_eq!(Value::from(data).to_string(), "L1,2 c1,2.5,3,4,5,6 z");
    }
}
