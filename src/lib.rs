#![allow(unused)]
pub mod formatter;

use std::fmt::Display;

pub trait ToPrettyTree {
    fn to_pretty_tree(&self) -> PrettyTree;
}

impl ToPrettyTree for PrettyTree {
    fn to_pretty_tree(&self) -> PrettyTree { self.clone() }
}

pub trait PrettyTreePrinter {
    fn print_pretty_tree(&self);
}

impl<Type> PrettyTreePrinter for Type where Type: ToPrettyTree {
    fn print_pretty_tree(&self) {
        let tree = self.to_pretty_tree().format(&Default::default());
        println!("{tree}")
    }
}

#[derive(Debug, Clone)]
pub enum PrettyTree {
    Empty,
    /// A terminal leaf node.
    Value(PrettyValue),
    /// A branch node.
    Branch(PrettyBranch),
    /// A fragment node.
    Fragment(PrettyFragment),
}

impl PrettyTree {
    pub fn empty() -> Self { Self::Empty }
    pub fn value<T: Display>(value: &T) -> Self {
        Self::Value(PrettyValue { text: format!("{value}") })
    }
    pub fn string<T: ToString>(value: T) -> Self {
        Self::Value(PrettyValue { text: value.to_string() })
    }
    pub fn str(value: impl AsRef<str>) -> Self {
        Self::Value(PrettyValue { text: value.as_ref().to_owned() })
    }
    pub fn fragment<T: Into<PrettyTree>>(list: impl IntoIterator<Item = T>) -> Self {
        Self::Fragment(PrettyFragment { nodes: list.into_iter().map(Into::into).collect() })
    }
    pub fn branch_of(
        label: impl AsRef<str>,
        children: &[PrettyTree]
    ) -> Self {
        Self::Branch(PrettyBranch {
            label: label.as_ref().to_string(),
            children: children.to_owned().into_iter().map(Into::into).collect(),
        })
    }
    pub fn some_value(value: impl Into<PrettyValue>) -> Self {
        Self::Value(value.into())
    }
    pub fn some_branch(branch: impl Into<PrettyBranch>) -> Self {
        Self::Branch(branch.into())
    }
    pub fn some_fragment(fragment: impl Into<PrettyFragment>) -> Self {
        Self::Fragment(fragment.into())
    }
}

impl Default for PrettyTree {
    fn default() -> Self { PrettyTree::Empty }
}

#[derive(Debug, Clone)]
pub struct PrettyValue {
    pub text: String
}

impl PrettyValue {
    pub fn from_str(value: impl AsRef<str>) -> Self {
        Self { text: value.as_ref().to_owned() }
    }
    pub fn from_string(value: impl ToString) -> Self {
        Self { text: value.to_string() }
    }
}

#[derive(Debug, Clone)]
pub struct PrettyBranch {
    pub label: String,
    pub children: Vec<PrettyTree>,
}

impl PrettyBranch {
    pub fn from_iter<Label: ToString, Child: Into<PrettyTree>>(
        label: &Label,
        children: impl IntoIterator<Item = Child>
    ) -> Self {
        PrettyBranch {
            label: label.to_string(),
            children: children.into_iter().map(Into::into).collect(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct PrettyFragment {
    pub nodes: Vec<PrettyTree>
}

impl PrettyFragment {
    pub fn from_iter<Value: Into<PrettyTree>>(list: impl IntoIterator<Item = Value>) -> Self {
        Self { nodes: list.into_iter().map(Into::into).collect() }
    }
}