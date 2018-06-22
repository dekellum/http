use {Uri, Result};
use convert::{HttpTryFrom, HttpTryInto};
use super::{Authority, Scheme, Parts, PathAndQuery};

/// dox
#[derive(Debug)]
pub struct Builder {
    parts: Result<Parts>,
}

impl Builder {
    /// Creates a new default instance of `Builder` to construct a `Uri`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use http::*;
    ///
    /// let uri = uri::Builder::new()
    ///     .scheme("https")
    ///     .authority("hyper.rs")
    ///     .path_and_query("/")
    ///     .build()
    ///     .unwrap();
    /// ```
    #[inline]
    pub fn new() -> Builder {
        Builder::default()
    }

    /// Set the `Scheme` for this URI.
    ///
    /// # Examples
    ///
    /// ```
    /// # use http::*;
    ///
    /// let builder = uri::Builder::new()
    ///     .scheme("https");
    /// ```
    pub fn scheme<T>(self, scheme: T) -> Builder
        where Scheme: HttpTryFrom<T>,
    {
        self.map(|parts| {
            parts.scheme = Some(scheme.http_try_into()?);
            Ok(())
        })
    }

    /// Set the `Authority` for this URI.
    ///
    /// # Examples
    ///
    /// ```
    /// # use http::*;
    ///
    /// let builder = uri::Builder::new()
    ///     .authority("tokio.rs");
    /// ```
    pub fn authority<T>(self, auth: T) -> Builder
        where Authority: HttpTryFrom<T>,
    {
        self.map(|parts| {
            parts.authority = Some(auth.http_try_into()?);
            Ok(())
        })
    }

    /// Set the `PathAndQuery` for this URI.
    ///
    /// # Examples
    ///
    /// ```
    /// # use http::*;
    ///
    /// let builder = uri::Builder::new()
    ///     .path_and_query("/hello?foo=bar");
    /// ```
    pub fn path_and_query<T>(self, p_and_q: T) -> Builder
        where PathAndQuery: HttpTryFrom<T>,
    {
        self.map(|parts| {
            parts.path_and_query = Some(p_and_q.http_try_into()?);
            Ok(())
        })
    }

    /// Consumes this builder, and tries to construct a valid `Uri` from
    /// the configured pieces.
    ///
    /// # Errors
    ///
    /// This function may return an error if any previously configured argument
    /// failed to parse or get converted to the internal representation. For
    /// example if an invalid `scheme` was specified via `scheme("!@#%/^")`
    /// the error will be returned when this function is called rather than
    /// when `scheme` was called.
    ///
    /// Additionally, the various forms of URI require certain combinations of
    /// parts to be set to be valid. If the parts don't fit into any of the
    /// valid forms of URI, a new error is returned.
    ///
    /// # Examples
    ///
    /// ```
    /// # use http::*;
    ///
    /// let uri = Uri::builder()
    ///     .build()
    ///     .unwrap();
    /// ```
    pub fn build(self) -> Result<Uri> {
        Ok(self
            .parts?
            .http_try_into()?)
    }

    fn map<F>(mut self, f: F) -> Builder
        where F: FnOnce(&mut Parts) -> Result<()>,
    {
        let res = if let Ok(ref mut parts) = self.parts {
            f(parts)
        } else {
            return self;
        };

        if let Err(err) = res {
            self.parts = Err(err);
        }

        self
    }
}

impl Default for Builder {
    #[inline]
    fn default() -> Builder {
        Builder {
            parts: Ok(Parts::default()),
        }
    }
}

impl From<Uri> for Builder {
    fn from(src: Uri) -> Builder {
        Builder {
            parts: Ok(src.into_parts()),
        }
    }
}

