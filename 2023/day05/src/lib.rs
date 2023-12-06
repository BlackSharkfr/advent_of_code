use std::ops::Range;

use aoc::Aoc;
use rayon::prelude::*;

pub struct Day;

impl Aoc for Day {
    type OUTPUT = u64;
    const DAY_NUMBER: u8 = 5;
    const INPUT: &'static str = include_str!("../inputs/input.txt");
    const SAMPLE_PART1: &'static str = include_str!("../inputs/sample1.txt");
    const SAMPLE_PART2: &'static str = include_str!("../inputs/sample2.txt");

    fn part1(input: &str) -> u64 {
        let (_, (seeds, ranges)) =
            parsers::part1(input).unwrap_or_else(|e| panic!("Parser failed {e:?}"));

        seeds
            .into_iter()
            .map(|mut seed| {
                for range in &ranges {
                    seed = range.translate(seed);
                }
                seed
            })
            .min()
            .expect("Seeds expects to have at least one seed")
    }

    fn part2(input: &str) -> u64 {
        let (_, (mut seed_ranges, range_maps)) =
            parsers::part2(input).unwrap_or_else(|e| panic!("Parser failed {e:?}"));

        for range_map in range_maps {
            range_map.translate_ranges(&mut seed_ranges);
        }

        seed_ranges
            .into_iter()
            .map(|range| range.start)
            .min()
            .expect("There should be at least one range")
    }
}

struct CompleteMap(Vec<Translate>);
impl CompleteMap {
    fn translate(&self, location: u64) -> u64 {
        self.0
            .iter()
            .find_map(|translate| {
                translate
                    .origin
                    .contains(&location)
                    .then(|| translate.translate(location))
            })
            .unwrap_or(location)
    }

    fn translate_ranges(&self, ranges: &mut Vec<Range<u64>>) {
        // Optionnal : Saves a few µs
        merge_ranges(ranges);

        let mut moved = Vec::new();
        for mapper in &self.0 {
            *ranges = ranges
                .iter()
                .flat_map(|range| {
                    let (destination, remain) = mapper.translate_range(range);
                    if let Some(dest) = destination {
                        moved.push(dest);
                    }
                    remain
                })
                .collect();
        }
        ranges.extend_from_slice(&mut moved);
    }
}

/// Reduces the amount of ranges. Saves a few µs overall
fn merge_ranges(ranges: &mut Vec<Range<u64>>) {
    ranges.par_sort_by_key(|range| range.start);
    for i in (1..ranges.len()).rev() {
        if ranges[i].start <= ranges[i - 1].end {
            ranges[i - 1].end = ranges[i - 1].end.max(ranges[i].end);
            ranges.remove(i);
        }
    }
}

#[derive(Debug)]
struct Translate {
    origin: Range<u64>,
    destination: u64,
}
impl Translate {
    fn translate(&self, location: u64) -> u64 {
        location - self.origin.start + self.destination
    }

    fn destination_end(&self) -> u64 {
        self.destination + (self.origin.end - self.origin.start)
    }

    /// Returns : (Moved range, Remaining ranges)
    ///
    /// There may not be a moved range
    /// The remaining range may have been split in two
    fn translate_range(&self, range: &Range<u64>) -> (Option<Range<u64>>, Vec<Range<u64>>) {
        match (range.start < self.origin.start, self.origin.end < range.end) {
            // Total overlap : range extends before and after the origin range
            (true, true) => (
                Some(self.destination..self.destination_end()),
                vec![range.start..self.origin.start, self.origin.end..range.end],
            ),
            // Inside : range is completely included within the origin range
            (false, false) => (
                Some(self.translate(range.start)..self.translate(range.end)),
                vec![],
            ),
            // Partial overlap : range extends before the origin range
            (true, false) if self.origin.start < range.end => (
                Some(self.destination..self.translate(range.end)),
                vec![range.start..self.origin.start],
            ),
            // Partial overlap : range extends after the origin range
            (false, true) if range.start < self.origin.end => (
                Some(self.translate(range.start)..self.destination_end()),
                vec![self.origin.end..range.end],
            ),
            // Outside : range has no common part with the origin range
            _ => (None, vec![range.clone()]),
        }
    }
}

mod parsers {
    use std::ops::Range;

    use nom::{
        bytes::complete::{tag, take_until1},
        character::complete::{line_ending, u64},
        multi::separated_list1,
        sequence::separated_pair,
        IResult, Parser,
    };
    use nom_supreme::ParserExt;

    use super::*;

    fn seeds(input: &str) -> IResult<&str, Vec<u64>> {
        separated_list1(tag(" "), u64)
            .preceded_by(tag("seeds: "))
            .terminated(line_ending)
            .parse(input)
    }

    fn seed_ranges(input: &str) -> IResult<&str, Vec<Range<u64>>> {
        separated_list1(
            tag(" "),
            separated_pair(u64, tag(" "), u64).map(|(start, len)| start..start + len),
        )
        .preceded_by(tag("seeds: "))
        .parse(input)
    }

    fn single_map(input: &str) -> IResult<&str, Translate> {
        let (input, destination) = u64(input)?;
        let (input, _) = tag(" ")(input)?;
        let (input, start) = u64(input)?;
        let (input, _) = tag(" ")(input)?;
        let (input, len) = u64(input)?;

        let origin = start..start + len;
        let transpose = Translate {
            origin,
            destination,
        };
        Ok((input, transpose))
    }

    fn complete_map(input: &str) -> IResult<&str, CompleteMap> {
        separated_list1(line_ending, single_map)
            .preceded_by(
                take_until1(":")
                    .terminated(tag(":"))
                    .terminated(line_ending),
            )
            .terminated(line_ending)
            .map(CompleteMap)
            .parse(input)
    }

    fn almanac(input: &str) -> IResult<&str, [CompleteMap; 7]> {
        let (input, m0) = complete_map(input)?;
        let (input, _) = line_ending(input)?;
        let (input, m1) = complete_map(input)?;
        let (input, _) = line_ending(input)?;
        let (input, m2) = complete_map(input)?;
        let (input, _) = line_ending(input)?;
        let (input, m3) = complete_map(input)?;
        let (input, _) = line_ending(input)?;
        let (input, m4) = complete_map(input)?;
        let (input, _) = line_ending(input)?;
        let (input, m5) = complete_map(input)?;
        let (input, _) = line_ending(input)?;
        let (input, m6) = complete_map(input)?;

        Ok((input, [m0, m1, m2, m3, m4, m5, m6]))
    }

    pub fn part1(input: &str) -> IResult<&str, (Vec<u64>, [CompleteMap; 7])> {
        let (input, seeds) = seeds(input)?;
        let (input, _) = line_ending(input)?;
        let (input, ranges) = almanac(input)?;

        Ok((input, (seeds, ranges)))
    }

    pub fn part2(input: &str) -> IResult<&str, (Vec<Range<u64>>, [CompleteMap; 7])> {
        let (input, seeds) = seed_ranges(input)?;
        let (input, _) = line_ending(input)?;
        let (input, ranges) = almanac(input)?;

        Ok((input, (seeds, ranges)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        Day::test_part1(35)
    }

    #[test]
    fn test_part2() {
        Day::test_part2(46)
    }
}
