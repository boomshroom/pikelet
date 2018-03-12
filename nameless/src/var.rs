use std::fmt;

use {AlphaEq, Debruijn, FreeName, LocallyNameless, Named};

/// A variable that can either be free or bound
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Var<N, B> {
    /// A free variable
    Free(N),
    /// A variable that is bound by a lambda or pi binder
    Bound(Named<N, B>),
}

impl<N: AlphaEq, B: AlphaEq> AlphaEq for Var<N, B> {
    fn alpha_eq(&self, other: &Var<N, B>) -> bool {
        match (self, other) {
            (&Var::Free(ref lhs), &Var::Free(ref rhs)) => N::alpha_eq(lhs, rhs),
            (&Var::Bound(ref lhs), &Var::Bound(ref rhs)) => Named::alpha_eq(lhs, rhs),
            (_, _) => false,
        }
    }
}

impl<N: FreeName> LocallyNameless for Var<N, Debruijn> {
    type Name = N;

    fn close_at(&mut self, index: Debruijn, name: &N) {
        *self = match *self {
            Var::Free(ref n) if n == name => Var::Bound(Named::new(n.clone(), index)),
            Var::Bound(_) | Var::Free(_) => return,
        };
    }

    fn open_at(&mut self, index: Debruijn, name: &N) {
        *self = match *self {
            Var::Bound(Named { inner: i, .. }) if i == index => Var::Free(name.clone()),
            Var::Bound(_) | Var::Free(_) => return,
        };
    }
}

impl<N: fmt::Display, B: fmt::Display> fmt::Display for Var<N, B> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Var::Bound(ref bound) if f.alternate() => write!(f, "{}{}", bound.name, bound.inner),
            Var::Bound(Named { ref name, .. }) | Var::Free(ref name) => write!(f, "{}", name),
        }
    }
}
