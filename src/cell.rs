use anyhow::anyhow;

/// Represents a single cell of the grid.
#[derive(Clone, Debug, Copy)]
pub struct Cell {
    /// The content of the cell
    pub value: CellValue,
    /// Heat represents how long ago the cell was last "visited" by a cursor.
    pub heat: i8,
}

impl From<CellValue> for Cell {
    fn from(value: CellValue) -> Self {
        Cell { value, heat: 0 }
    }
}

impl From<char> for Cell {
    fn from(value: char) -> Self {
        Cell::from(CellValue::from(value))
    }
}

#[cfg_attr(test, derive(Hash, PartialEq, Eq))]
#[derive(Clone, Debug, Copy)]
pub enum CellValue {
    Empty,
    Op(Operator),
    Dir(Direction),
    If(IfDir),
    StringMode,
    Bridge,
    End,
    Number(u32),
    Char(char),
}

impl From<char> for CellValue {
    fn from(value: char) -> Self {
        match value {
            ' ' => CellValue::Empty,
            '\"' => CellValue::StringMode,
            '#' => CellValue::Bridge,
            '@' => CellValue::End,
            v @ '0'..='9' => CellValue::Number(u32::from(v)),
            c => {
                if let Ok(op) = Operator::try_from(c) {
                    CellValue::Op(op)
                } else if let Ok(dir) = Direction::try_from(c) {
                    CellValue::Dir(dir)
                } else {
                    CellValue::Char(c)
                }
            }
        }
    }
}

impl From<CellValue> for char {
    fn from(value: CellValue) -> Self {
        match value {
            CellValue::Empty => ' ',
            CellValue::Op(operator) => operator.into(),
            CellValue::Dir(dir) => dir.into(),
            CellValue::If(dir) => dir.into(),
            CellValue::StringMode => '"',
            CellValue::Bridge => '#',
            CellValue::End => '@',
            CellValue::Number(num) => num.to_string().chars().next().unwrap(),
            CellValue::Char(c) => c,
        }
    }
}

#[cfg_attr(test, derive(Hash, PartialEq, Eq))]
#[derive(Clone, Debug, Copy)]
pub enum Operator {
    Nullary(NullaryOperator),
    Unary(UnaryOperator),
    Binary(BinaryOperator),
    Ternary(TernaryOperator),
}

impl TryFrom<char> for Operator {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(if let Ok(nullary) = NullaryOperator::try_from(value) {
            Operator::Nullary(nullary)
        } else if let Ok(unary) = UnaryOperator::try_from(value) {
            Operator::Unary(unary)
        } else if let Ok(binary) = BinaryOperator::try_from(value) {
            Operator::Binary(binary)
        } else if let Ok(ternary) = TernaryOperator::try_from(value) {
            Operator::Ternary(ternary)
        } else {
            return Err(anyhow!("Invalid operator `{value}`"));
        })
    }
}

impl From<Operator> for char {
    fn from(value: Operator) -> Self {
        match value {
            Operator::Nullary(nullary) => char::from(nullary),
            Operator::Unary(unary) => char::from(unary),
            Operator::Binary(binary) => char::from(binary),
            Operator::Ternary(ternary) => char::from(ternary),
        }
    }
}

macro_rules! char_mapping {
    ($($enum:ident : $($variant:ident = $c:literal),* $(,)?);* $(;)?) => {
        $(
            impl TryFrom<char> for $enum {
                type Error = anyhow::Error;

                fn try_from(value: char) -> Result<Self, Self::Error> {
                    Ok(match value {
                        $(
                            $c => $enum::$variant,
                        )*
                        c => return Err(anyhow!("Invalid {} `{}`", stringify!($enum), c)),
                    })
                }
            }

            impl From<$enum> for char {
                fn from(value: $enum) -> char {
                    match value {
                        $(
                            $enum::$variant => $c,
                        )*
                    }
                }
            }
        )*
    };
}

char_mapping! {
    NullaryOperator:
        Integer = '&',
        Ascii = '~';

    UnaryOperator:
        Negate = '!',
        Duplicate = ':',
        Pop = '$',
        WriteNumber = '.',
        WriteASCII = ',';

    BinaryOperator:
        Greater = '`',
        Add = '+',
        Subtract = '-',
        Multiply = '*',
        Divide = '/',
        Modulo = '%',
        Swap = '\\',
        Get = 'g';

    TernaryOperator:
        Put = 'p';

    IfDir:
        Horizontal = '_',
        Vertical = '|';

    Direction:
        Up = '^',
        Down = 'v',
        Left = '<',
        Right = '>',
        Random = '?';
}

#[cfg_attr(test, derive(Hash, Eq))]
#[derive(Default, PartialEq, Clone, Debug, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    #[default]
    Right,
    Random,
}

#[cfg_attr(test, derive(Hash, PartialEq, Eq))]
#[derive(Clone, Debug, Copy)]
pub enum NullaryOperator {
    Integer,
    Ascii,
}

#[cfg_attr(test, derive(Hash, PartialEq, Eq))]
#[derive(Clone, Debug, Copy)]
pub enum UnaryOperator {
    Negate,
    Duplicate,
    Pop,
    WriteNumber,
    WriteASCII,
}

#[cfg_attr(test, derive(Hash, PartialEq, Eq))]
#[derive(Clone, Debug, Copy)]
pub enum BinaryOperator {
    Greater,
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Swap,
    Get,
}

#[cfg_attr(test, derive(Hash, PartialEq, Eq))]
#[derive(Clone, Debug, Copy)]
pub enum TernaryOperator {
    Put,
}

#[cfg_attr(test, derive(Hash, PartialEq, Eq))]
#[derive(Clone, Debug, Copy)]
pub enum IfDir {
    Horizontal,
    Vertical,
}

#[cfg(test)]
mod test {
    use super::*;

    macro_rules! collection {
        ($($k:expr => $v:expr),* $(,)?) => {{
            core::convert::From::from([$(($k, $v),)*])
        }};
    }

    #[test]
    fn serialize() {
        let map: Vec<(CellValue, char)> = collection! {
            CellValue::Empty => ' ',
            CellValue::Op(Operator::Nullary(NullaryOperator::Integer)) => '&',
            CellValue::Op(Operator::Nullary(NullaryOperator::Ascii)) => '~',
            CellValue::Op(Operator::Unary(UnaryOperator::Negate)) => '!',
            CellValue::Op(Operator::Unary(UnaryOperator::Duplicate)) => ':',
            CellValue::Op(Operator::Unary(UnaryOperator::Pop)) => '$',
            CellValue::Op(Operator::Unary(UnaryOperator::WriteNumber)) => '.',
            CellValue::Op(Operator::Unary(UnaryOperator::WriteASCII)) => ',',
            CellValue::Op(Operator::Binary(BinaryOperator::Greater)) => '`',
            CellValue::Op(Operator::Binary(BinaryOperator::Add)) => '+',
            CellValue::Op(Operator::Binary(BinaryOperator::Subtract)) => '-',
            CellValue::Op(Operator::Binary(BinaryOperator::Multiply)) => '*',
            CellValue::Op(Operator::Binary(BinaryOperator::Divide)) => '/',
            CellValue::Op(Operator::Binary(BinaryOperator::Modulo)) => '%',
            CellValue::Op(Operator::Binary(BinaryOperator::Swap)) => '\\',
            CellValue::Op(Operator::Binary(BinaryOperator::Get)) => 'g',
            CellValue::Op(Operator::Ternary(TernaryOperator::Put)) => 'p',
            CellValue::Dir(Direction::Up) => '^',
            CellValue::Dir(Direction::Down) => 'v',
            CellValue::Dir(Direction::Left) => '<',
            CellValue::Dir(Direction::Right) => '>',
            CellValue::If(IfDir::Horizontal) => '_',
            CellValue::If(IfDir::Vertical) => '|',
            CellValue::StringMode => '"',
            CellValue::Bridge => '#',
            CellValue::End => '@',
            CellValue::Number(5) => '5',
            CellValue::Char('c') => 'c',
        };

        for (cell_value, expected) in map.iter() {
            let got = char::from(*cell_value);
            assert_eq!(*expected, got, "Failed to serialize {cell_value:?}: {got}",);
        }
    }
}
