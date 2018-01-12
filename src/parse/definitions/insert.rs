use super::select::SelectStatement;
use super::select::SelectSource;
use super::update::UpdateAssignment;
use super::update::UpdateValue;

#[derive(Debug, PartialEq, Clone, Serialize)]
pub enum InsertPriority {
    Unknown,
    Low,
    Delayed,
    High,
}

#[derive(Debug, PartialEq, Clone, Serialize)]
pub enum InsertSource<'a> {
    Assignment {
        assignments: Vec<UpdateAssignment<'a>>,
    },
    Selection {
        columns: Option<Vec<&'a str>>,
        statement: SelectStatement<'a>,
    },
    Values {
        columns: Option<Vec<&'a str>>,
        rows: Vec<Vec<UpdateValue<'a>>>,
    },
}

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct InsertStatement<'a> {
    pub priority: InsertPriority,
    pub ignore: bool,
    pub target: SelectSource<'a>,
    pub source: InsertSource<'a>,
    pub on_duplicate_key_update: Option<Vec<UpdateAssignment<'a>>>,
}
