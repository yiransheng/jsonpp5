use std::iter::IntoIterator;

use pretty::{DocAllocator, DocBuilder};
use serde_json::map::Map;
use serde_json::Value;

macro_rules! alloc_block {
    ($delim: ident, $o:expr, $c: expr, $value: expr, $nest: expr, $alloc:expr) => {{
        let allocator = $alloc;

        allocator
            .text($o)
            .append(allocator.$delim().append($value).nest($nest))
            .append(allocator.$delim())
            .append(allocator.text($c))
    }};
}
macro_rules! bracket {
    ($delim: ident, $value: expr, $nest: expr, $alloc:expr) => {
        alloc_block!($delim, "[", "]", $value, $nest, $alloc)
    };
}
macro_rules! brace {
    ($delim: ident, $value: expr, $nest: expr, $alloc:expr) => {
        alloc_block!($delim, "{", "}", $value, $nest, $alloc)
    };
}

pub fn value<'b, D, A>(js_val: &'b Value, allocator: &'b D) -> DocBuilder<'b, D, A>
where
    D: DocAllocator<'b, A>,
    D::Doc: Clone,
    A: Clone,
{
    match js_val {
        Value::Null => allocator.text("null"),
        Value::Bool(true) => allocator.text("true"),
        Value::Bool(false) => allocator.text("false"),
        Value::Number(ref n) => allocator.text(n.to_string()),
        ref s @ Value::String(_) => allocator.text(s.to_string()),
        Value::Array(ref xs) => {
            let oneline_allowed = xs.into_iter().all(allow_oneline);
            let values = array_values(&*xs, oneline_allowed, allocator);

            if oneline_allowed {
                bracket!(space, values, 2, allocator)
            } else {
                bracket!(newline, values, 2, allocator)
            }
        }
        Value::Object(ref map) => obj_values(map, allocator),
    }
}

fn obj_values<'b, D, A>(vals: &'b Map<String, Value>, allocator: &'b D) -> DocBuilder<'b, D, A>
where
    D: DocAllocator<'b, A>,
    D::Doc: Clone,
    A: Clone,
{
    if vals.is_empty() {
        allocator.text("{}")
    } else {
        brace!(newline, pairs(vals, allocator), 2, allocator).group()
    }
}

fn pairs<'b, I, D, A>(pairs: I, allocator: &'b D) -> DocBuilder<'b, D, A>
where
    I: IntoIterator<Item = (&'b String, &'b Value)>,
    D: DocAllocator<'b, A>,
    D::Doc: Clone,
    A: Clone,
{
    let sep = allocator.text(",").append(allocator.newline());
    allocator.intersperse(
        pairs.into_iter().map(|(k, v)| pair((&*k, v), allocator)),
        sep,
    )
}

fn pair<'b, D, A>(pair: (&'b str, &'b Value), allocator: &'b D) -> DocBuilder<'b, D, A>
where
    D: DocAllocator<'b, A>,
    D::Doc: Clone,
    A: Clone,
{
    let (key, val) = pair;
    allocator
        .text(js_string(key))
        .append(allocator.text(": "))
        .append(value(val, allocator))
        .group()
}

fn array_values<'b, D, A>(
    vals: &'b [Value],
    oneline: bool,
    allocator: &'b D,
) -> DocBuilder<'b, D, A>
where
    D: DocAllocator<'b, A>,
    D::Doc: Clone,
    A: Clone,
{
    if vals.is_empty() {
        return allocator.nil();
    }
    let sep = if oneline {
        allocator.text(",").append(allocator.space())
    } else {
        allocator.text(",").append(allocator.newline())
    };

    allocator.intersperse(vals.into_iter().map(|v| value(v, allocator)), sep)
}

fn allow_oneline(val: &Value) -> bool {
    match val {
        Value::Array(ref xs) => !xs.into_iter().any(|v| !allow_oneline(v)),
        Value::Object(ref map) => map.is_empty(),
        _ => true,
    }
}

fn js_string(s: &str) -> String {
    Value::from(s.to_string()).to_string()
}
