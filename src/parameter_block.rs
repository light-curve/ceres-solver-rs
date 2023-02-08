//! Parameter block and related structures for [NllsProblem](crate::nlls_problem::NllsProblem).

use crate::error::ParameterBlockStorageError;

use std::pin::Pin;

/// Parameter vector representation to use with [NllsProblem](crate::nlls_problem::NllsProblem).
pub struct ParameterBlock {
    values: Pin<Vec<f64>>,
    pointer: *mut f64,
    lower_bounds: Option<Vec<Option<f64>>>,
    upper_bounds: Option<Vec<Option<f64>>>,
}

#[allow(clippy::len_without_is_empty)]
impl ParameterBlock {
    // Create a new parameter vector.
    pub fn new(values: impl Into<Vec<f64>>) -> Self {
        let mut values = Pin::new(values.into());
        assert!(!values.is_empty());
        let pointer = values.as_mut_ptr();
        Self {
            values,
            pointer,
            lower_bounds: None,
            upper_bounds: None,
        }
    }

    /// Add lower bounds to the parameter vector. [None] means no lower bound.
    pub fn with_lower_bounds(&mut self, lower_bounds: impl Into<Vec<Option<f64>>>) -> &mut Self {
        let lower_bounds = lower_bounds.into();
        assert_eq!(lower_bounds.len(), self.len());
        self.lower_bounds = Some(lower_bounds);
        self
    }

    /// Add upper bounds to the parameter vector. [None] means no upper bound.
    pub fn with_upper_bounds(&mut self, upper_bounds: impl Into<Vec<Option<f64>>>) -> &mut Self {
        let upper_bounds = upper_bounds.into();
        assert_eq!(upper_bounds.len(), self.len());
        self.upper_bounds = Some(upper_bounds);
        self
    }

    /// Add lower bounds to the parameter vector.
    pub fn with_all_lower_bounds(&mut self, lower_bounds: impl Into<Vec<f64>>) -> &mut Self {
        let lower_bounds = lower_bounds.into();
        assert_eq!(lower_bounds.len(), self.len());
        self.with_lower_bounds(lower_bounds.into_iter().map(Some).collect::<Vec<_>>())
    }

    /// Add upper bounds to the parameter vector.
    pub fn with_all_upper_bounds(&mut self, upper_bounds: impl Into<Vec<f64>>) -> &mut Self {
        let upper_bounds = upper_bounds.into();
        assert_eq!(upper_bounds.len(), self.len());
        self.with_upper_bounds(upper_bounds.into_iter().map(Some).collect::<Vec<_>>())
    }

    /// Number of parameters.
    pub fn len(&self) -> usize {
        self.values.len()
    }

    /// Lower bounds of the parameters, if any. [None] means no lower bound.
    pub fn lower_bounds(&self) -> Option<&[Option<f64>]> {
        self.lower_bounds.as_deref()
    }

    /// Upper bounds of the parameters, if any. [None] means no upper bound.
    pub fn upper_bounds(&self) -> Option<&[Option<f64>]> {
        self.upper_bounds.as_deref()
    }

    /// Components of the parameter.
    pub fn values(&self) -> &[f64] {
        &self.values
    }

    pub(crate) fn pointer_mut(&self) -> *mut f64 {
        self.pointer
    }

    /// Convert to vector of parameters, each parameter is vector of floats.
    pub fn to_values(self) -> Vec<f64> {
        Pin::into_inner(self.values)
    }
}

impl From<Vec<f64>> for ParameterBlock {
    fn from(values: Vec<f64>) -> Self {
        Self::new(values)
    }
}

pub enum ParameterBlockOrIndex {
    Block(ParameterBlock),
    Index(usize),
}

impl From<ParameterBlock> for ParameterBlockOrIndex {
    fn from(block: ParameterBlock) -> Self {
        Self::Block(block)
    }
}

impl From<usize> for ParameterBlockOrIndex {
    fn from(index: usize) -> Self {
        Self::Index(index)
    }
}

impl From<Vec<f64>> for ParameterBlockOrIndex {
    fn from(values: Vec<f64>) -> Self {
        Self::Block(ParameterBlock::new(values))
    }
}

pub struct ParameterBlockStorage {
    storage: Vec<ParameterBlock>,
}

impl ParameterBlockStorage {
    pub fn new() -> Self {
        Self {
            storage: Vec::new(),
        }
    }

    pub fn extend<P>(
        &mut self,
        parameter_blocks: impl IntoIterator<Item = P>,
    ) -> Result<Vec<usize>, ParameterBlockStorageError>
    where
        P: Into<ParameterBlockOrIndex>,
    {
        let mut indices = Vec::new();
        for parameter_block in parameter_blocks {
            let parameter_block = parameter_block.into();
            let len = self.storage.len();
            match parameter_block {
                ParameterBlockOrIndex::Block(block) => {
                    indices.push(len);
                    self.storage.push(block);
                }
                ParameterBlockOrIndex::Index(index) => {
                    if index >= self.storage.len() {
                        return Err(ParameterBlockStorageError::IndexOutOfBounds { index, len });
                    }
                    indices.push(index);
                }
            }
        }
        Ok(indices)
    }

    #[inline]
    pub fn blocks(&self) -> &[ParameterBlock] {
        &self.storage
    }

    #[inline]
    pub fn get_block(&self, index: usize) -> Result<&ParameterBlock, ParameterBlockStorageError> {
        self.storage
            .get(index)
            .ok_or(ParameterBlockStorageError::IndexOutOfBounds {
                index,
                len: self.storage.len(),
            })
    }

    pub fn to_values(self) -> Vec<Vec<f64>> {
        self.storage.into_iter().map(|p| p.to_values()).collect()
    }
}

impl Default for ParameterBlockStorage {
    fn default() -> Self {
        Self::new()
    }
}
