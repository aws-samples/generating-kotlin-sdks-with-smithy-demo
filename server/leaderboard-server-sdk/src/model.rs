// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// Describes one specific validation failure for an input member.
#[derive(::std::clone::Clone, ::std::cmp::Eq, ::std::cmp::PartialEq, ::std::fmt::Debug, ::std::hash::Hash)]
pub struct ValidationExceptionField  {
    /// A JSONPointer expression to the structure member whose value failed to satisfy the modeled constraints.
    pub path: ::std::string::String,
    /// A detailed description of the validation failure.
    pub message: ::std::string::String,
}
impl  ValidationExceptionField  {
    /// A JSONPointer expression to the structure member whose value failed to satisfy the modeled constraints.
    pub fn path(&self) -> & str {
        use std::ops::Deref; self.path.deref()
    }
    /// A detailed description of the validation failure.
    pub fn message(&self) -> & str {
        use std::ops::Deref; self.message.deref()
    }
}
impl  ValidationExceptionField  {
    /// Creates a new builder-style object to manufacture [`ValidationExceptionField`](crate::model::ValidationExceptionField).
    pub fn builder() -> crate::model::validation_exception_field::Builder  {
        crate::model::validation_exception_field::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[derive(::std::clone::Clone, ::std::cmp::Eq, ::std::cmp::PartialEq, ::std::fmt::Debug, ::std::hash::Hash)]
pub struct ScoreEvent  {
    #[allow(missing_docs)] // documentation missing in model
    pub user: ::std::option::Option<::std::string::String>,
    #[allow(missing_docs)] // documentation missing in model
    pub score: ::std::option::Option<i32>,
}
impl  ScoreEvent  {
    #[allow(missing_docs)] // documentation missing in model
    pub fn user(&self) -> ::std::option::Option<& str> {
        self.user.as_deref()
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn score(&self) -> ::std::option::Option<i32> {
        self.score
    }
}
impl  ScoreEvent  {
    /// Creates a new builder-style object to manufacture [`ScoreEvent`](crate::model::ScoreEvent).
    pub fn builder() -> crate::model::score_event::Builder  {
        crate::model::score_event::Builder::default()
    }
}
impl crate::constrained::Constrained for crate::model::ScoreEvent {
                type Unconstrained = crate::model::score_event::Builder;
            }

#[allow(missing_docs)] // documentation missing in model
/// 
/// This is a constrained type because its corresponding modeled Smithy shape has one or more
/// [constraint traits]. Use [`LeaderboardId::try_from`] to construct values of this type.
/// 
/// [constraint traits]: https://awslabs.github.io/smithy/1.0/spec/core/constraint-traits.html
/// 
#[derive(::std::clone::Clone, ::std::cmp::Eq, ::std::cmp::PartialEq, ::std::fmt::Debug, ::std::hash::Hash)]
pub struct LeaderboardId(pub(crate) ::std::string::String);
impl LeaderboardId {
                /// Extracts a string slice containing the entire underlying `String`.
                pub fn as_str(&self) -> &str {
                    &self.0
                }

                /// Returns an immutable reference to the underlying [`::std::string::String`].
                pub fn inner(&self) -> &::std::string::String {
                    &self.0
                }

                /// Consumes the value, returning the underlying [`::std::string::String`].
                pub fn into_inner(self) -> ::std::string::String {
                    self.0
                }
            }
impl LeaderboardId {
            fn check_length(string: &str) -> Result<(), crate::model::leaderboard_id::ConstraintViolation> {
                    let length = string.chars().count();

                    if (4..=64).contains(&length) {
                        Ok(())
                    } else {
                        Err(crate::model::leaderboard_id::ConstraintViolation::Length(length))
                    }
                }
        }
impl ::std::convert::TryFrom<::std::string::String> for LeaderboardId {
            type Error = crate::model::leaderboard_id::ConstraintViolation;

            /// Constructs a `LeaderboardId` from an [`::std::string::String`], failing when the provided value does not satisfy the modeled constraints.
            fn try_from(value: ::std::string::String) -> Result<Self, Self::Error> {
              Self::check_length(&value)?;

              Ok(Self(value))
            }
        }
impl crate::constrained::Constrained for LeaderboardId  {
                type Unconstrained = ::std::string::String;
            }

            impl ::std::convert::From<::std::string::String> for crate::constrained::MaybeConstrained<crate::model::LeaderboardId> {
                fn from(value: ::std::string::String) -> Self {
                    Self::Unconstrained(value)
                }
            }

            impl ::std::fmt::Display for LeaderboardId {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                   self.0.fmt(f)
                }
            }


            impl ::std::convert::From<LeaderboardId> for ::std::string::String {
                fn from(value: LeaderboardId) -> Self {
                    value.into_inner()
                }
            }


#[allow(missing_docs)] // documentation missing in model
/// 
/// This is a constrained type because its corresponding modeled Smithy shape has one or more
/// [constraint traits]. Use [`MaxEntries::try_from`] to construct values of this type.
/// 
/// [constraint traits]: https://awslabs.github.io/smithy/1.0/spec/core/constraint-traits.html
/// 
#[derive(::std::clone::Clone, ::std::cmp::Eq, ::std::cmp::PartialEq, ::std::fmt::Debug, ::std::hash::Hash)]
pub struct MaxEntries(pub(crate) i32);
impl MaxEntries {
                /// Returns an immutable reference to the underlying [`i32`].
                pub fn inner(&self) -> &i32 {
                    &self.0
                }

                /// Consumes the value, returning the underlying [`i32`].
                pub fn into_inner(self) -> i32 {
                    self.0
                }
            }

            impl crate::constrained::Constrained for MaxEntries  {
                type Unconstrained = i32;
            }

            impl ::std::convert::From<i32> for crate::constrained::MaybeConstrained<crate::model::MaxEntries> {
                fn from(value: i32) -> Self {
                    Self::Unconstrained(value)
                }
            }

            impl ::std::fmt::Display for MaxEntries {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                   self.0.fmt(f)
                }
            }

            impl ::std::convert::From<MaxEntries> for i32 {
                fn from(value: MaxEntries) -> Self {
                    value.into_inner()
                }
            }
impl MaxEntries {
            fn check_range(value: i32) -> Result<(), crate::model::max_entries::ConstraintViolation> {
                    if (1..=32).contains(&value) {
                        Ok(())
                    } else {
                        Err(crate::model::max_entries::ConstraintViolation::Range(value))
                    }
                }
        }
impl ::std::convert::TryFrom<i32> for MaxEntries {
            type Error = crate::model::max_entries::ConstraintViolation;

            /// Constructs a `MaxEntries` from an [`i32`], failing when the provided value does not satisfy the modeled constraints.
            fn try_from(value: i32) -> Result<Self, Self::Error> {
              Self::check_range(value)?;

              Ok(Self(value))
            }
        }


#[allow(missing_docs)] // documentation missing in model
#[derive(::std::clone::Clone, ::std::cmp::Eq, ::std::cmp::PartialEq, ::std::fmt::Debug, ::std::hash::Hash)]
pub struct LeaderboardSummary  {
    #[allow(missing_docs)] // documentation missing in model
    pub id: ::std::option::Option<crate::model::LeaderboardId>,
    #[allow(missing_docs)] // documentation missing in model
    pub name: ::std::option::Option<::std::string::String>,
    #[allow(missing_docs)] // documentation missing in model
    pub max_entries: ::std::option::Option<crate::model::MaxEntries>,
}
impl  LeaderboardSummary  {
    #[allow(missing_docs)] // documentation missing in model
    pub fn id(&self) -> ::std::option::Option<& crate::model::LeaderboardId> {
        self.id.as_ref()
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn name(&self) -> ::std::option::Option<& str> {
        self.name.as_deref()
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn max_entries(&self) -> ::std::option::Option<& crate::model::MaxEntries> {
        self.max_entries.as_ref()
    }
}
impl  LeaderboardSummary  {
    /// Creates a new builder-style object to manufacture [`LeaderboardSummary`](crate::model::LeaderboardSummary).
    pub fn builder() -> crate::model::leaderboard_summary::Builder  {
        crate::model::leaderboard_summary::Builder::default()
    }
}
/// See [`ValidationExceptionField`](crate::model::ValidationExceptionField).
/// 
pub mod validation_exception_field {
    
    #[derive(::std::cmp::PartialEq, ::std::fmt::Debug)]
    /// Holds one variant for each of the ways the builder can fail.
    #[non_exhaustive]
    
                #[allow(clippy::enum_variant_names)]
                pub enum ConstraintViolation {
        /// `path` was not provided but it is required when building `ValidationExceptionField`.
        MissingPath,
        /// `message` was not provided but it is required when building `ValidationExceptionField`.
        MissingMessage,
    }
    impl ::std::fmt::Display for ConstraintViolation {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                ConstraintViolation::MissingPath => write!(f, "`path` was not provided but it is required when building `ValidationExceptionField`"),
                ConstraintViolation::MissingMessage => write!(f, "`message` was not provided but it is required when building `ValidationExceptionField`"),
            }
        }
    }
    impl ::std::error::Error for ConstraintViolation { }
    impl  ::std::convert::TryFrom<Builder > for crate::model::ValidationExceptionField {
                    type Error = ConstraintViolation;
    
                    fn try_from(builder: Builder ) -> Result<Self, Self::Error> {
                        builder.build()
                    }
                }
    /// A builder for [`ValidationExceptionField`](crate::model::ValidationExceptionField).
    #[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
    pub struct Builder {
        pub(crate) path: ::std::option::Option<::std::string::String>,
        pub(crate) message: ::std::option::Option<::std::string::String>,
    }
    impl  Builder  {
        /// A JSONPointer expression to the structure member whose value failed to satisfy the modeled constraints.
        pub fn path(mut self, input: ::std::string::String) -> Self {
            self.path =
                Some(
                    input
                )
            ; self
        }
        /// A detailed description of the validation failure.
        pub fn message(mut self, input: ::std::string::String) -> Self {
            self.message =
                Some(
                    input
                )
            ; self
        }
        /// Consumes the builder and constructs a [`ValidationExceptionField`](crate::model::ValidationExceptionField).
        /// 
        /// The builder fails to construct a [`ValidationExceptionField`](crate::model::ValidationExceptionField) if a [`ConstraintViolation`] occurs.
        /// 
        /// If the builder fails, it will return the _first_ encountered [`ConstraintViolation`].
        pub fn build(self) -> Result<crate::model::ValidationExceptionField , ConstraintViolation> {
                        self.build_enforcing_all_constraints()
                    }
        fn build_enforcing_all_constraints(self) -> Result<crate::model::ValidationExceptionField , ConstraintViolation> {
            Ok(
                crate::model::ValidationExceptionField {
                    path: self.path
                        .ok_or(ConstraintViolation::MissingPath)?
                    ,
                    message: self.message
                        .ok_or(ConstraintViolation::MissingMessage)?
                    ,
                }
            )
        }
    }
    
    
    
    
}
/// See [`ScoreEvent`](crate::model::ScoreEvent).
/// 
pub mod score_event {
    
    impl ::std::convert::From<Builder > for crate::model::ScoreEvent  {
                    fn from(builder: Builder) -> Self {
                        builder.build()
                    }
                }
    /// A builder for [`ScoreEvent`](crate::model::ScoreEvent).
    #[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
    pub struct Builder {
        pub(crate) user: ::std::option::Option<::std::string::String>,
        pub(crate) score: ::std::option::Option<i32>,
    }
    impl  Builder  {
        #[allow(missing_docs)] // documentation missing in model
        pub fn user(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
            self.user =
                input
            ; self
        }
        #[allow(missing_docs)] // documentation missing in model
        pub(crate) fn set_user(mut self, input: Option<impl ::std::convert::Into<::std::string::String>>) -> Self {
            self.user = input.map(|v| v.into());
                            self
        }
        #[allow(missing_docs)] // documentation missing in model
        pub fn score(mut self, input: ::std::option::Option<i32>) -> Self {
            self.score =
                input
            ; self
        }
        #[allow(missing_docs)] // documentation missing in model
        pub(crate) fn set_score(mut self, input: Option<impl ::std::convert::Into<i32>>) -> Self {
            self.score = input.map(|v| v.into());
                            self
        }
        /// Consumes the builder and constructs a [`ScoreEvent`](crate::model::ScoreEvent).
        pub fn build(self) -> crate::model::ScoreEvent {
                        self.build_enforcing_all_constraints()
                    }
        fn build_enforcing_all_constraints(self) -> crate::model::ScoreEvent {
            crate::model::ScoreEvent {
                user: self.user
                ,
                score: self.score
                ,
            }
        }
    }
    
    
    
    
}
/// See [`LeaderboardId`](crate::model::LeaderboardId).
pub mod leaderboard_id {
    
    #[derive(Debug, PartialEq)]
                pub enum ConstraintViolation {
                    
    /// Error when a string doesn't satisfy its `@length` requirements.
    Length(usize)
                }
    impl ConstraintViolation {
                        pub(crate) fn as_validation_exception_field(self, path: ::std::string::String) -> crate::model::ValidationExceptionField {
                        match self {
                            Self::Length(length) => crate::model::ValidationExceptionField {
                            message: format!("Value with length {} at '{}' failed to satisfy constraint: Member must have length between 4 and 64, inclusive", length, &path),
                            path,
                        },
                        }
                    }
                    }
    
    
    
    
}
/// See [`MaxEntries`](crate::model::MaxEntries).
pub mod max_entries {
    
    #[derive(Debug, PartialEq)]
                    pub enum ConstraintViolation {
                        Range(i32),
                    }
    impl ConstraintViolation {
                            pub(crate) fn as_validation_exception_field(self, path: ::std::string::String) -> crate::model::ValidationExceptionField {
                        match self {
                            Self::Range(_) => crate::model::ValidationExceptionField {
                            message: format!("Value at '{}' failed to satisfy constraint: Member must be between 1 and 32, inclusive", &path),
                            path,
                        },
                        }
                    }
                        }
    
    
    
    
}
/// See [`LeaderboardSummary`](crate::model::LeaderboardSummary).
/// 
pub mod leaderboard_summary {
    
    impl ::std::convert::From<Builder > for crate::model::LeaderboardSummary  {
                    fn from(builder: Builder) -> Self {
                        builder.build()
                    }
                }
    /// A builder for [`LeaderboardSummary`](crate::model::LeaderboardSummary).
    #[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
    pub struct Builder {
        pub(crate) id: ::std::option::Option<crate::model::LeaderboardId>,
        pub(crate) name: ::std::option::Option<::std::string::String>,
        pub(crate) max_entries: ::std::option::Option<crate::model::MaxEntries>,
    }
    impl  Builder  {
        #[allow(missing_docs)] // documentation missing in model
        pub fn id(mut self, input: ::std::option::Option<crate::model::LeaderboardId>) -> Self {
            self.id =
                input
            ; self
        }
        #[allow(missing_docs)] // documentation missing in model
        pub fn name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
            self.name =
                input
            ; self
        }
        #[allow(missing_docs)] // documentation missing in model
        pub fn max_entries(mut self, input: ::std::option::Option<crate::model::MaxEntries>) -> Self {
            self.max_entries =
                input
            ; self
        }
        /// Consumes the builder and constructs a [`LeaderboardSummary`](crate::model::LeaderboardSummary).
        pub fn build(self) -> crate::model::LeaderboardSummary {
                        self.build_enforcing_all_constraints()
                    }
        fn build_enforcing_all_constraints(self) -> crate::model::LeaderboardSummary {
            crate::model::LeaderboardSummary {
                id: self.id
                ,
                name: self.name
                ,
                max_entries: self.max_entries
                ,
            }
        }
    }
    
    
    
    
}
