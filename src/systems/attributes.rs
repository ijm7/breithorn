use std::default;
use std::error;

const MIN_ATTRIBUTE_SCORE: u32 = 0;
const MAX_ATTRIBUTE_SCORE: u32 = 100;

pub struct Attributes {
    strength: AttributeScore,
	dexterity: AttributeScore,
	charisma: AttributeScore,
	intelligence: AttributeScore,
	willpower: AttributeScore,
}

struct AttributeScore(u32);

#[derive(Debug, PartialEq)]
struct AttributeScoreError {
    min: u32,
    max: u32,
}

impl AttributeScore {
    pub fn set(&self, score: u32) -> Result<(), AttributeScoreError> {
        if score <= 100 {
            Self(score);
            Ok(())
        }
        else {
            Err(AttributeScoreError { min: MIN_ATTRIBUTE_SCORE, max: MAX_ATTRIBUTE_SCORE })
        }
    }
}

impl Default for AttributeScore {
    fn default() -> Self { AttributeScore(0) }
}

#[test]
fn valid_zero_attribute_score() {
    let attribute: AttributeScore = Default::default();
    assert_eq!(attribute.set(0), Ok(()));
}

#[test]
fn valid_normal_attribute_score() {
    let attribute: AttributeScore = Default::default();
    assert_eq!(attribute.set(20), Ok(()));
}

#[test]
fn valid_extreme_attribute_score() {
    let attribute: AttributeScore = Default::default();
    assert_eq!(attribute.set(100), Ok(()));
}

#[test]
fn invalid_attribute_score() {
    let attribute: AttributeScore = Default::default();
    assert_eq!(attribute.set(101), Err(AttributeScoreError { min: MIN_ATTRIBUTE_SCORE, max: MAX_ATTRIBUTE_SCORE }));
}
