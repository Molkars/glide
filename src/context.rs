
pub struct Context(Box<ContextNode>);

struct ContextNode {

    parent: Option<Context>,
}

impl Context {

}