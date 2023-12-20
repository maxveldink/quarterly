use std::str::FromStr;

/// Represents one of the four quarters of a year
#[derive(Debug, PartialEq)]
pub enum QuarterNumber {
    Q1,
    Q2,
    Q3,
    Q4,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseQuarterError;

impl FromStr for QuarterNumber {
    type Err = ParseQuarterError;

    /// ```
    /// # use quarterly::*;
    /// assert_eq!("Q1".parse::<QuarterNumber>(), Ok(QuarterNumber::Q1));
    /// assert_eq!("q1".parse::<QuarterNumber>(), Ok(QuarterNumber::Q1));
    /// assert_eq!("banana".parse::<QuarterNumber>(), Err(ParseQuarterError));
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "Q1" => Ok(QuarterNumber::Q1),
            "Q2" => Ok(QuarterNumber::Q2),
            "Q3" => Ok(QuarterNumber::Q3),
            "Q4" => Ok(QuarterNumber::Q4),
            _ => Err(ParseQuarterError),
        }
    }
}

impl TryFrom<u8> for QuarterNumber {
    type Error = ParseQuarterError;

    /// ```
    /// # use quarterly::*;
    /// assert_eq!(QuarterNumber::try_from(1), Ok(QuarterNumber::Q1));
    /// assert_eq!(QuarterNumber::try_from(2), Ok(QuarterNumber::Q2));
    /// assert_eq!(QuarterNumber::try_from(5), Err(ParseQuarterError));
    /// ```
    fn try_from(value: u8) -> Result<Self, ParseQuarterError> {
        match value {
            1 => Ok(QuarterNumber::Q1),
            2 => Ok(QuarterNumber::Q2),
            3 => Ok(QuarterNumber::Q3),
            4 => Ok(QuarterNumber::Q4),
            _ => Err(ParseQuarterError),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Quarter {
    pub quarter_number: QuarterNumber,
    pub year: u16,
}

impl Quarter {
    /// ```
    /// # use quarterly::*;
    /// assert_eq!(Quarter::new(QuarterNumber::Q1, 2023), Quarter { quarter_number: QuarterNumber::Q1, year: 2023 })
    /// ```
    pub fn new(quarter_number: QuarterNumber, year: u16) -> Self {
        Quarter {
            quarter_number,
            year,
        }
    }

    /// Determines next calendar quarter, handling year wrapping
    ///
    /// ```
    /// # use quarterly::*;
    /// assert_eq!(Quarter::new(QuarterNumber::Q1, 2023).next_quarter(), Quarter::new(QuarterNumber::Q2, 2023));
    /// assert_eq!(Quarter::new(QuarterNumber::Q4, 2023).next_quarter(), Quarter::new(QuarterNumber::Q1, 2024));
    /// ```
    pub fn next_quarter(&self) -> Self {
        match self.quarter_number {
            QuarterNumber::Q1 => Quarter {
                year: self.year,
                quarter_number: QuarterNumber::Q2,
            },
            QuarterNumber::Q2 => Quarter {
                year: self.year,
                quarter_number: QuarterNumber::Q3,
            },
            QuarterNumber::Q3 => Quarter {
                year: self.year,
                quarter_number: QuarterNumber::Q4,
            },
            QuarterNumber::Q4 => Quarter {
                year: self.year + 1,
                quarter_number: QuarterNumber::Q1,
            },
        }
    }
}

impl FromStr for Quarter {
    type Err = ParseQuarterError;

    /// ```
    /// # use quarterly::*;
    /// assert_eq!("Q2 2023".parse::<Quarter>(), Ok(Quarter { year: 2023, quarter_number: QuarterNumber::Q2 } ));
    /// assert_eq!("Q2".parse::<Quarter>(), Err(ParseQuarterError));
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (quarter_number, year) = s.split_once(' ').ok_or(ParseQuarterError)?;

        let quarter_number_fromstr = quarter_number
            .parse::<QuarterNumber>()
            .map_err(|_| ParseQuarterError)?;
        let year_fromstr: u16 = year.parse().map_err(|_| ParseQuarterError)?;

        Ok(Quarter {
            year: year_fromstr,
            quarter_number: quarter_number_fromstr,
        })
    }
}
