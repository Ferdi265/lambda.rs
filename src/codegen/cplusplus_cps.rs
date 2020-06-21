use std::collections::BTreeSet;
use std::collections::BTreeMap;
use std::collections::btree_map::Entry::Occupied;

use super::*;

static RESERVED_WORDS: [&str; 101] = [
    "alignas",
    "alignof",
    "and",
    "and_eq",
    "asm",
    "atomic_cancel",
    "atomic_commit",
    "atomic_noexcept",
    "auto",
    "bitand",
    "bitor",
    "bool",
    "break",
    "case",
    "catch",
    "char",
    "char8_t",
    "char16_t",
    "char32_t",
    "class",
    "compl",
    "concept",
    "const",
    "consteval",
    "constexpr",
    "constinit",
    "const_cast",
    "continue",
    "co_await",
    "co_return",
    "co_yield",
    "decltypedefault",
    "delete",
    "do",
    "double",
    "dynamic_cast",
    "else",
    "enum",
    "explicit",
    "export",
    "extern",
    "false",
    "float",
    "for",
    "friend",
    "goto",
    "if",
    "inline",
    "int",
    "long",
    "mutable",
    "namespace",
    "new",
    "noexcept",
    "not",
    "not_eq",
    "nullptr",
    "operator",
    "or",
    "or_eq",
    "private",
    "protected",
    "public",
    "register",
    "reinterpret_cast",
    "requires",
    "return",
    "short",
    "signed",
    "sizeof",
    "static",
    "static_assert",
    "static_cast",
    "struct",
    "switch",
    "synchronized",
    "template",
    "this",
    "thread_local",
    "throw",
    "true",
    "try",
    "typedef",
    "typeid",
    "typename",
    "union",
    "unsigned",
    "using",
    "virtual",
    "void",
    "volatile",
    "wchar_t",
    "while",
    "xor",
    "xor_eq ",
    "Lambda",
    "Cont",
    "LambdaFn",
    "arg",
    "self",
    "cont"
];

static CODEGEN_PRELUDE: &str = include_str!("prelude_cps.cpp");

#[derive(Debug, Clone, Copy)]
pub struct CPlusPlusCPS;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ArgName<'i> {
    Unnamed,
    Anonymous(usize),
    Identifier(Identifier<'i>)
}

#[derive(Debug)]
struct Implementation<'i, 'a> {
    id: usize,
    arg_name: ArgName<'i>,
    function: Option<&'a Literal<'i>>,
    argument: &'a Literal<'i>,
    captures: &'a BTreeSet<Identifier<'i>>,
    anonymous_captures: &'a BTreeSet<usize>,
    next: Option<&'a Continuation<'i>>
}

#[derive(Debug)]
struct ImplementationChain<'i, 'a> {
    arg_name: Option<Identifier<'i>>,
    continuations: &'a [Continuation<'i>],
    result_literal: &'a Literal<'i>,
    captures: &'a BTreeSet<Identifier<'i>>
}

struct AssignmentContext<'i> {
    cur_assignment: Identifier<'i>,
    cur_lambda_id: Option<usize>,
    impls: Vec<String>,
}

impl<'i> AssignmentContext<'i> {
    fn new(cur_assignment: Identifier<'i>, cur_lambda_id: Option<usize>) -> Self {
        AssignmentContext {
            cur_assignment,
            cur_lambda_id,
            impls: Vec::new()
        }
    }

    fn add_impl(&mut self, s: String) {
        self.impls.push(s)
    }
}

#[derive(Debug, Clone, Default)]
struct ImplementationContext<'i> {
    arg_name: Option<ArgName<'i>>,
    arg_references: usize,
    global_references: BTreeMap<Identifier<'i>, usize>,
    capture_references: BTreeMap<Identifier<'i>, usize>,
    anonymous_references: BTreeMap<usize, usize>,
}

impl<'i> ImplementationContext<'i> {
    fn new(arg_name: ArgName<'i>, captures: &BTreeSet<Identifier<'i>>, anonymous_captures: &BTreeSet<usize>) -> Self {
        ImplementationContext {
            arg_name: Some(arg_name),
            arg_references: 0,
            global_references: BTreeMap::new(),
            capture_references: captures.iter()
                .cloned()
                .map(|c| (c, 0))
                .collect(),
            anonymous_references: anonymous_captures.iter()
                .cloned()
                .map(|c| (c, 0))
                .collect()
        }
    }

    fn reference_identifier(&mut self, ident: Identifier<'i>) {
        if Some(ArgName::Identifier(ident)) == self.arg_name {
            self.arg_references += 1;
        } else if let Occupied(mut entry) = self.capture_references.entry(ident) {
            *entry.get_mut() += 1;
        } else {
            *self.global_references.entry(ident).or_insert(0) += 1;
        }
    }

    fn reference_anonymous(&mut self, anon: usize) {
        if Some(ArgName::Anonymous(anon)) == self.arg_name {
            self.arg_references += 1;
        } else if let Occupied(mut entry) = self.anonymous_references.entry(anon) {
            *entry.get_mut() += 1;
        } else {
            panic!("uncaptured anonymous literal '{}' referenced!", anon);
        }
    }
}

fn generate_identifier(ident: Identifier<'_>) -> String {
    util::generate_suffix_identifier(ident, &RESERVED_WORDS, None)
}

fn generate_anonymous_identifier(id: usize) -> String {
    util::generate_suffix_identifier(
        "ret", &RESERVED_WORDS,
        Some(format!("_{}", id))
    )
}

fn generate_cont_identifier(ident: Identifier<'_>, lambda_id: Option<usize>, cont_id: usize) -> String {
    let lambda_id = lambda_id.map(|i| i + 1).unwrap_or(0);
    util::generate_suffix_identifier(
        ident, &RESERVED_WORDS,
        Some(format!("_{}_{}", lambda_id, cont_id))
    )
}

fn generate_arg_name_identifier(arg_name: ArgName<'_>) -> String {
    match arg_name {
        ArgName::Unnamed => util::generate_suffix_identifier(
            "arg", &RESERVED_WORDS, None
        ),
        ArgName::Anonymous(id) => generate_anonymous_identifier(id),
        ArgName::Identifier(ident) => generate_identifier(ident)
    }
}

fn generate_literal<'i>(lit: &Literal<'i>, actx: &mut AssignmentContext<'i>, ictx: &mut ImplementationContext<'i>) -> String {
    match lit {
        Literal::Anonymous(id) => {
            ictx.reference_anonymous(*id);
            generate_anonymous_identifier(*id)
        }
        Literal::Identifier(ident) => {
            ictx.reference_identifier(ident);
            generate_identifier(ident)
        },
        Literal::Lambda(lambda) => generate_lambda(&lambda, actx, ictx)
    }
}

fn generate_captures<'i>(captures: &BTreeSet<Identifier<'i>>, anonymous_captures: &BTreeSet<usize>, ictx: &mut ImplementationContext<'i>) -> String {
    let cap: Vec<_> = captures.iter()
        .map(|ident| {
            ictx.reference_identifier(ident);
            generate_identifier(ident)
        }).collect();
    let anon_cap = anonymous_captures.iter()
        .map(|anon| {
            ictx.reference_anonymous(*anon);
            generate_anonymous_identifier(*anon)
        });

    format!("{{{}}}", cap.into_iter().chain(anon_cap).collect::<Vec<_>>().join(", "))
}

fn generate_continuation<'i>(cont: &Continuation<'i>, actx: &mut AssignmentContext<'_>, ictx: &mut ImplementationContext<'i>) -> String {
    let cont_name = generate_cont_identifier(actx.cur_assignment, actx.cur_lambda_id, cont.id);
    let n = cont.captures.len() + cont.anonymous_captures.len();

    format!("Cont::mk<{}>({}, {}, cont)", n, cont_name, generate_captures(&cont.captures, &cont.anonymous_captures, ictx))
}

fn generate_implementation<'i>(imp: Implementation<'i, '_>, actx: &mut AssignmentContext<'i>) {
    let cont_name = generate_cont_identifier(actx.cur_assignment, actx.cur_lambda_id, imp.id);
    let arg_name = generate_arg_name_identifier(imp.arg_name);

    println!("generate_implementation: {:?}", imp);

    let mut res = format!("Lambda* {}(Lambda* {}, Lambda* self, Cont* cont) {{\n",
        cont_name, arg_name
    );

    let mut ictx = ImplementationContext::new(imp.arg_name, imp.captures, imp.anonymous_captures);

    let next = imp.next.map(|next| generate_continuation(next, actx, &mut ictx));
    let func = imp.function.map(|lit| generate_literal(lit, actx, &mut ictx));
    let arg = generate_literal(imp.argument, actx, &mut ictx);

    let mut i = 0;

    let cap = ictx.capture_references.into_iter()
        .map(|(cap, refcount)|
             (generate_identifier(cap), refcount)
        );

    let anon_cap = ictx.anonymous_references.into_iter()
        .map(|(cap, refcount)|
            (generate_anonymous_identifier(cap), refcount)
        );

    for (capture, refcount) in cap.chain(anon_cap) {
        if refcount > 0 {
            res += &format!("    Lambda* {} = self->captures[{}]->ref({});\n",
                capture, i, refcount
            );
        }

        i += 1;
    }

    for (capture, refcount) in ictx.global_references.into_iter() {
        if refcount > 0 {
            res += &format!("    {}->ref({});\n", capture, refcount);
        }

        i += 1;
    }

    res += "    self->unref();\n";

    if let Some(func) = func {
        let cont = next.unwrap_or_else(|| String::from("cont"));

        res += &format!("    return {}->call({}, {});\n",
            func, arg, cont
        );
    } else {
        res += &format!("    return cont->call({});\n", arg);
    }

    res += "}\n";

    actx.add_impl(res);
}

fn generate_implementation_chain<'i>(chain: ImplementationChain<'i, '_>, actx: &mut AssignmentContext<'i>, ictx: &mut ImplementationContext<'i>) -> String {
    let mut arg_name = match chain.arg_name {
        None => ArgName::Unnamed,
        Some(ident) => ArgName::Identifier(ident)
    };
    let mut next = None;

    if chain.continuations.is_empty() {
        generate_implementation(Implementation {
            id: 0,
            arg_name,
            function: None,
            argument: &chain.result_literal,
            captures: chain.captures,
            anonymous_captures: &BTreeSet::new(),
            next
        }, actx);
    } else {
        for cont in chain.continuations.iter().rev() {
            generate_implementation(Implementation {
                id: cont.id,
                arg_name,
                function: Some(&cont.function),
                argument: &cont.argument,
                captures: &cont.captures,
                anonymous_captures: &cont.anonymous_captures,
                next
            }, actx);

            arg_name = ArgName::Anonymous(cont.id);
            next = Some(cont);
        }
    }

    let lambda_name = generate_cont_identifier(actx.cur_assignment, actx.cur_lambda_id, 0);
    let n = chain.captures.len();

    format!("Lambda::mk<{}>({}, {})", n, lambda_name, generate_captures(chain.captures, &BTreeSet::new(), ictx))
}


fn generate_lambda<'i>(lambda: &Lambda<'i>, actx: &mut AssignmentContext<'i>, ictx: &mut ImplementationContext<'i>) -> String {
    let mut sub_actx = AssignmentContext::new(actx.cur_assignment, Some(lambda.data.id));

    let res = generate_implementation_chain(ImplementationChain {
        arg_name: Some(lambda.argument),
        continuations: &lambda.data.continuations,
        result_literal: &lambda.data.result_literal,
        captures: &lambda.data.captures
    }, &mut sub_actx, ictx);

    actx.impls.extend(sub_actx.impls);

    res
}


fn generate_assignment(ass: &Assignment<'_>) -> String {
    let mut actx = AssignmentContext::new(ass.target, None);
    let mut ictx = Default::default();

    let target = generate_identifier(ass.target);

    let value = if ass.data.continuations.is_empty() {
        generate_literal(&ass.data.result_literal, &mut actx, &mut ictx)
    } else {
        let lambda = generate_implementation_chain(ImplementationChain {
            arg_name: None,
            continuations: &ass.data.continuations,
            result_literal: &ass.data.result_literal,
            captures: &BTreeSet::new()
        }, &mut actx, &mut ictx);

        lambda + "->ret()"
    };

    let mut res = String::new();

    for imp in actx.impls {
        res += &format!("{}\n", imp);
    }

    res + &format!("Lambda* {} = {};\n\n", target, value)
}

impl CodegenTarget for CPlusPlusCPS {
    fn generate(&self, program: &Program<'_>) -> String {
        let mut res = String::new();

        res += CODEGEN_PRELUDE;

        for ass in program.iter() {
            res += &generate_assignment(ass);
        }

        res
    }
}
