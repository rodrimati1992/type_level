#[derive(Debug, Copy, Clone)]
pub struct DisabledEnabled<T> {
    stored: Option<T>,
    flag: EnabledDisabledState,
}

impl<T> DisabledEnabled<T> {
    pub fn new(v: T) -> Self {
        DisabledEnabled {
            stored: Some(v),
            flag: EnabledDisabledState::Disabled,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum EnabledDisabledState {
    Disabled,
    Enabled,
    Finished,
}

impl<T> Iterator for DisabledEnabled<T>
where
    T: Default,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.flag {
            EnabledDisabledState::Disabled => {
                self.flag = EnabledDisabledState::Enabled;
                Some(T::default())
            }
            EnabledDisabledState::Enabled => {
                self.flag = EnabledDisabledState::Finished;
                Some(self.stored.take().unwrap())
            }
            EnabledDisabledState::Finished => None,
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let size = match self.flag {
            EnabledDisabledState::Disabled => 2,
            EnabledDisabledState::Enabled { .. } => 1,
            EnabledDisabledState::Finished => 0,
        };
        (size, Some(size))
    }
}

impl<T> ExactSizeIterator for DisabledEnabled<T> where T: Default {}

#[test]
fn test_enabled_disabled_iter() {
    assert_eq!(DisabledEnabled::new(100).collect::<Vec<_>>(), vec![0, 100]);
    assert_eq!(DisabledEnabled::new(200).collect::<Vec<_>>(), vec![0, 200]);
    assert_eq!(DisabledEnabled::new(300).collect::<Vec<_>>(), vec![0, 300]);
    assert_eq!(DisabledEnabled::new(400).collect::<Vec<_>>(), vec![0, 400]);
    assert_eq!(
        DisabledEnabled::new("what").collect::<Vec<_>>(),
        vec!["", "what"]
    );
    assert_eq!(
        DisabledEnabled::new(true).collect::<Vec<_>>(),
        vec![false, true]
    );
}
