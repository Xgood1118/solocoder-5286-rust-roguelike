use std::fmt::Debug;

#[derive(Debug)]
pub enum BTNode<Context: 'static> {
    Sequence(Vec<BTNode<Context>>),
    Selector(Vec<BTNode<Context>>),
    Inverter(Box<BTNode<Context>>),
    Repeat(Box<BTNode<Context>>, usize),
    Action(Box<dyn BTAction<Context> + Send + Sync>),
    Condition(Box<dyn BTCondition<Context> + Send + Sync>),
}

pub trait BTAction<Context>: Debug {
    fn execute(&self, context: &mut Context) -> BTResult;
}

pub trait BTCondition<Context>: Debug {
    fn check(&self, context: &Context) -> bool;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BTResult {
    Success,
    Failure,
    Running,
}

impl<Context: 'static> BTNode<Context> {
    pub fn tick(&self, context: &mut Context) -> BTResult {
        match self {
            BTNode::Sequence(children) => tick_sequence(children, context),
            BTNode::Selector(children) => tick_selector(children, context),
            BTNode::Inverter(child) => {
                let result = child.tick(context);
                match result {
                    BTResult::Success => BTResult::Failure,
                    BTResult::Failure => BTResult::Success,
                    BTResult::Running => BTResult::Running,
                }
            }
            BTNode::Repeat(child, times) => {
                for _ in 0..*times {
                    let result = child.tick(context);
                    if result != BTResult::Success {
                        return result;
                    }
                }
                BTResult::Success
            }
            BTNode::Action(action) => action.execute(context),
            BTNode::Condition(condition) => {
                if condition.check(context) {
                    BTResult::Success
                } else {
                    BTResult::Failure
                }
            }
        }
    }
}

fn tick_sequence<Context: 'static>(children: &[BTNode<Context>], context: &mut Context) -> BTResult {
    for child in children {
        match child.tick(context) {
            BTResult::Failure => return BTResult::Failure,
            BTResult::Running => return BTResult::Running,
            BTResult::Success => continue,
        }
    }
    BTResult::Success
}

fn tick_selector<Context: 'static>(children: &[BTNode<Context>], context: &mut Context) -> BTResult {
    for child in children {
        match child.tick(context) {
            BTResult::Success => return BTResult::Success,
            BTResult::Running => return BTResult::Running,
            BTResult::Failure => continue,
        }
    }
    BTResult::Failure
}

#[derive(Debug)]
pub struct LowHealthCondition {
    pub threshold: f32,
}

#[derive(Debug)]
pub struct TargetInRangeCondition {
    pub range: f32,
}

#[derive(Debug)]
pub struct MoveTowardsTargetAction;

#[derive(Debug)]
pub struct AttackAction;

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug)]
    struct TestContext {
        value: i32,
    }

    #[derive(Debug)]
    struct IncrementAction;

    impl BTAction<TestContext> for IncrementAction {
        fn execute(&self, context: &mut TestContext) -> BTResult {
            context.value += 1;
            BTResult::Success
        }
    }

    #[derive(Debug)]
    struct IsPositiveCondition;

    impl BTCondition<TestContext> for IsPositiveCondition {
        fn check(&self, context: &TestContext) -> bool {
            context.value > 0
        }
    }

    #[test]
    fn test_sequence_success() {
        let mut context = TestContext { value: 0 };

        let sequence = BTNode::Sequence(vec![
            BTNode::Action(Box::new(IncrementAction)),
            BTNode::Action(Box::new(IncrementAction)),
        ]);

        let result = sequence.tick(&mut context);
        assert_eq!(result, BTResult::Success);
        assert_eq!(context.value, 2);
    }

    #[test]
    fn test_selector_success() {
        let mut context = TestContext { value: 0 };

        let selector = BTNode::Selector(vec![
            BTNode::Condition(Box::new(IsPositiveCondition)),
            BTNode::Action(Box::new(IncrementAction)),
        ]);

        let result = selector.tick(&mut context);
        assert_eq!(result, BTResult::Success);
        assert_eq!(context.value, 1);
    }

    #[test]
    fn test_inverter() {
        let context = TestContext { value: -1 };

        let inverter = BTNode::Inverter(Box::new(BTNode::Condition(Box::new(
            IsPositiveCondition,
        ))));

        let result = inverter.tick(&mut TestContext { value: -1 });
        assert_eq!(result, BTResult::Success);
    }
}
