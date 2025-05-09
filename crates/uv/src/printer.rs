use anstream::{eprint, print};
use indicatif::ProgressDrawTarget;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Printer {
    /// A printer that suppresses all output.
    Silent,
    /// A printer that suppresses most output, but preserves "important" stdout.
    Quiet,
    /// A printer that prints to standard streams (e.g., stdout).
    Default,
    /// A printer that prints all output, including debug messages.
    Verbose,
    /// A printer that prints to standard streams, excluding all progress outputs
    NoProgress,
}

impl Printer {
    /// Return the [`ProgressDrawTarget`] for this printer.
    pub(crate) fn target(self) -> ProgressDrawTarget {
        match self {
            Self::Silent => ProgressDrawTarget::hidden(),
            Self::Quiet => ProgressDrawTarget::hidden(),
            Self::Default => ProgressDrawTarget::stderr(),
            // Confusingly, hide the progress bar when in verbose mode.
            // Otherwise, it gets interleaved with debug messages.
            Self::Verbose => ProgressDrawTarget::hidden(),
            Self::NoProgress => ProgressDrawTarget::hidden(),
        }
    }

    /// Return the [`Stdout`] for this printer.
    #[allow(dead_code, reason = "to be adopted incrementally")]
    pub(crate) fn stdout_important(self) -> Stdout {
        match self {
            Self::Silent => Stdout::Disabled,
            Self::Quiet => Stdout::Enabled,
            Self::Default => Stdout::Enabled,
            Self::Verbose => Stdout::Enabled,
            Self::NoProgress => Stdout::Enabled,
        }
    }

    /// Return the [`Stdout`] for this printer.
    pub(crate) fn stdout(self) -> Stdout {
        match self {
            Self::Silent => Stdout::Disabled,
            Self::Quiet => Stdout::Disabled,
            Self::Default => Stdout::Enabled,
            Self::Verbose => Stdout::Enabled,
            Self::NoProgress => Stdout::Enabled,
        }
    }

    /// Return the [`Stderr`] for this printer.
    pub(crate) fn stderr(self) -> Stderr {
        match self {
            Self::Silent => Stderr::Disabled,
            Self::Quiet => Stderr::Disabled,
            Self::Default => Stderr::Enabled,
            Self::Verbose => Stderr::Enabled,
            Self::NoProgress => Stderr::Enabled,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Stdout {
    Enabled,
    Disabled,
}

impl std::fmt::Write for Stdout {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        match self {
            Self::Enabled => {
                #[allow(clippy::print_stdout, clippy::ignored_unit_patterns)]
                {
                    print!("{s}");
                }
            }
            Self::Disabled => {}
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Stderr {
    Enabled,
    Disabled,
}

impl std::fmt::Write for Stderr {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        match self {
            Self::Enabled => {
                #[allow(clippy::print_stderr, clippy::ignored_unit_patterns)]
                {
                    eprint!("{s}");
                }
            }
            Self::Disabled => {}
        }

        Ok(())
    }
}
