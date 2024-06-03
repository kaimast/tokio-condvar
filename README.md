# A Condition Variable for Tokio

[![ci-badge](https://github.com/kaimast/tokio-condvar/actions/workflows/ci.yml/badge.svg)](https://github.com/kaimast/tokio-condvar/actions)
[![license-badge](https://img.shields.io/crates/l/tokio-condvar)](https://github.com/kaimast/tokio-condvar-rs/blob/main/LICENSE)
[![crates-badge](https://img.shields.io/crates/v/tokio-condvar)](https://crates.io/crates/tokio-condvar)

**Not Cancellation-Safe:** If your future gets canceled while inside wait, no other future may get woken up.

**Spurious Wakeups Possible:** Like any condition variable, there can be spurious wakeups, so always re-check the condition after waking up.
Note that, because this crate uses `tokio`'s `Notify` internally and its semantics are slightly different than those of condition variables, spurious wake ups are even more likely with this library.

**This is not part of the Tokio project:** See the discussion [here](https://github.com/tokio-rs/tokio/issues/3892) on why Tokio does not have a built-in Condvar implementation (yet).
