use std::fmt::Debug;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;

pub trait ASTData {
    type ProgramData: Clone + DataDisplay = ();
    type AssignmentData: Clone + DataDisplay = ();
    type ApplicationData: Clone + DataDisplay = ();
    type LambdaData: Clone + DataDisplay = ();
}

pub trait DataDisplay {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult;
}

pub struct NoData;
impl ASTData for NoData {}

impl<T: Debug> DataDisplay for T {
    default fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "[data = {:?}] ", self)
    }
}

impl DataDisplay for () {
    fn fmt(&self, _: &mut Formatter<'_>) -> FmtResult {
        Ok(())
    }
}

pub type Identifier<'i> = &'i str;

#[derive(Clone)]
pub struct Lambda<'i, D: ASTData> {
    pub argument: Identifier<'i>,
    pub body: Application<'i, D>,
    pub data: D::LambdaData
}

#[derive(Clone)]
pub enum Expression<'i, D: ASTData> {
    Lambda(Lambda<'i, D>),
    Parenthesis(Application<'i, D>),
    Identifier(Identifier<'i>),
}

#[derive(Clone)]
pub struct Application<'i, D: ASTData> {
    pub head: Box<Expression<'i, D>>,
    pub tail: Option<Box<Application<'i, D>>>,
    pub data: D::ApplicationData
}

#[derive(Clone)]
pub struct Assignment<'i, D: ASTData> {
    pub target: Identifier<'i>,
    pub value: Application<'i, D>,
    pub data: D::AssignmentData
}

#[derive(Clone)]
pub struct Program<'i, D: ASTData> {
    pub assignments: Vec<Assignment<'i, D>>,
    pub data: D::ProgramData
}

pub struct ApplicationIter<'a, 'i, D: ASTData> (
    Option<&'a Application<'i, D>>
);

impl<'a, 'i, D: ASTData> Iterator for ApplicationIter<'a, 'i, D> {
    type Item = &'a Expression<'i, D>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(app) = self.0 {
            let expr = &app.head;

            self.0 = app.tail.as_ref()
                .map(|app| app.as_ref());

            Some(expr)
        } else {
            None
        }
    }
}

impl<'i, D: ASTData> Application<'i, D> {
    pub fn iter(&self) -> impl Iterator<Item = &Expression<'i, D>> {
        ApplicationIter(Some(self))
    }
}

impl<'i, D: ASTData> Program<'i, D> {
    pub fn iter(&self) -> impl Iterator<Item = &Assignment<'i, D>> {
        self.assignments.iter()
    }
}
