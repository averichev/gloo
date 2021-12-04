use std::borrow::Cow;

#[cfg(feature = "query")]
use serde::Serialize;

use crate::browser::BrowserHistory;
#[cfg(feature = "query")]
use crate::error::HistoryResult;
use crate::hash::HashHistory;
use crate::history::History;
use crate::listener::HistoryListener;
use crate::location::Location;

/// A [`History`] that provides a universial API to the underlying history type.
#[derive(Clone, PartialEq, Debug)]
pub enum AnyHistory {
    /// A Browser History.
    Browser(BrowserHistory),
    /// A Hash History
    Hash(HashHistory),
}

impl History for AnyHistory {
    fn len(&self) -> usize {
        match self {
            Self::Browser(m) => m.len(),
            Self::Hash(m) => m.len(),
        }
    }

    fn go(&self, delta: isize) {
        match self {
            Self::Browser(m) => m.go(delta),
            Self::Hash(m) => m.go(delta),
        }
    }

    fn push<'a>(&self, route: impl Into<Cow<'a, str>>) {
        match self {
            Self::Browser(m) => m.push(route),
            Self::Hash(m) => m.push(route),
        }
    }

    fn replace<'a>(&self, route: impl Into<Cow<'a, str>>) {
        match self {
            Self::Browser(m) => m.replace(route),
            Self::Hash(m) => m.replace(route),
        }
    }

    fn push_with_state<'a, T>(&self, route: impl Into<Cow<'a, str>>, state: T)
    where
        T: 'static,
    {
        match self {
            Self::Browser(m) => m.push_with_state(route, state),
            Self::Hash(m) => m.push_with_state(route, state),
        }
    }

    fn replace_with_state<'a, T>(&self, route: impl Into<Cow<'a, str>>, state: T)
    where
        T: 'static,
    {
        match self {
            Self::Browser(m) => m.replace_with_state(route, state),
            Self::Hash(m) => m.replace_with_state(route, state),
        }
    }

    #[cfg(feature = "query")]
    fn push_with_query<'a, Q>(&self, route: impl Into<Cow<'a, str>>, query: Q) -> HistoryResult<()>
    where
        Q: Serialize,
    {
        match self {
            Self::Browser(m) => m.push_with_query(route, query),
            Self::Hash(m) => m.push_with_query(route, query),
        }
    }
    #[cfg(feature = "query")]
    fn replace_with_query<'a, Q>(
        &self,
        route: impl Into<Cow<'a, str>>,
        query: Q,
    ) -> HistoryResult<()>
    where
        Q: Serialize,
    {
        match self {
            Self::Browser(m) => m.replace_with_query(route, query),
            Self::Hash(m) => m.replace_with_query(route, query),
        }
    }

    #[cfg(all(feature = "query"))]
    fn push_with_query_and_state<'a, Q, T>(
        &self,
        route: impl Into<Cow<'a, str>>,
        query: Q,
        state: T,
    ) -> HistoryResult<()>
    where
        Q: Serialize,
        T: Serialize + 'static,
    {
        match self {
            Self::Browser(m) => m.push_with_query_and_state(route, query, state),
            Self::Hash(m) => m.push_with_query_and_state(route, query, state),
        }
    }

    #[cfg(all(feature = "query"))]
    fn replace_with_query_and_state<'a, Q, T>(
        &self,
        route: impl Into<Cow<'a, str>>,
        query: Q,
        state: T,
    ) -> HistoryResult<()>
    where
        Q: Serialize,
        T: Serialize + 'static,
    {
        match self {
            Self::Browser(m) => m.replace_with_query_and_state(route, query, state),
            Self::Hash(m) => m.replace_with_query_and_state(route, query, state),
        }
    }

    fn listen<CB>(&self, callback: CB) -> HistoryListener
    where
        CB: Fn() + 'static,
    {
        match self {
            Self::Browser(m) => m.listen(callback),
            Self::Hash(m) => m.listen(callback),
        }
    }

    fn location(&self) -> Location {
        match self {
            Self::Browser(m) => m.location(),
            Self::Hash(m) => m.location(),
        }
    }
}

impl From<BrowserHistory> for AnyHistory {
    fn from(m: BrowserHistory) -> AnyHistory {
        AnyHistory::Browser(m)
    }
}

impl From<HashHistory> for AnyHistory {
    fn from(m: HashHistory) -> AnyHistory {
        AnyHistory::Hash(m)
    }
}
