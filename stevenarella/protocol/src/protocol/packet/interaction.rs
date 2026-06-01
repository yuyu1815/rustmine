pub enum DigType {
    StartDestroyBlock,
    AbortDestroyBlock,
    FinishDestroyBlock,
    DropAllItems,
    DropItem,
    ReleaseUseItem,
    SwapItemInHand,
    Invalid(i32),
}

impl DigType {
    pub fn ordinal(&self) -> i32 {
        match self {
            DigType::StartDestroyBlock => 0,
            DigType::AbortDestroyBlock => 1,
            DigType::FinishDestroyBlock => 2,
            DigType::DropAllItems => 3,
            DigType::DropItem => 4,
            DigType::ReleaseUseItem => 5,
            DigType::SwapItemInHand => 6,
            DigType::Invalid(id) => *id,
        }
    }
}

impl From<i32> for DigType {
    fn from(src: i32) -> Self {
        match src {
            0 => DigType::StartDestroyBlock,
            1 => DigType::AbortDestroyBlock,
            2 => DigType::FinishDestroyBlock,
            3 => DigType::DropAllItems,
            4 => DigType::DropItem,
            5 => DigType::ReleaseUseItem,
            6 => DigType::SwapItemInHand,
            _ => DigType::Invalid(src),
        }
    }
}

#[derive(Debug, Default)]
pub enum Hand {
    #[default]
    MainHand,
    OffHand,
    Invalid(i32),
}

impl Hand {
    pub fn ordinal(&self) -> i32 {
        match self {
            Hand::MainHand => 0,
            Hand::OffHand => 1,
            Hand::Invalid(id) => *id,
        }
    }
}

impl From<i32> for Hand {
    fn from(src: i32) -> Self {
        match src {
            0 => Hand::MainHand,
            1 => Hand::OffHand,
            _ => Hand::Invalid(src),
        }
    }
}
