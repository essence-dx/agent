//! Ties a spawned streaming-parser task's lifetime to the stream the consumer
//! holds, so dropping the stream (turn cancel, timeout, client disconnect)
//! aborts the task and releases its socket instead of leaking it.

/// Aborts the wrapped task when dropped. Carry it inside the returned stream's
/// `unfold` state so the abort fires exactly when the consumer drops the
/// stream. `AbortHandle::abort` is a no-op once the task has finished, so the
/// happy path is unaffected.
pub(crate) struct AbortOnDrop(tokio::task::AbortHandle);

impl AbortOnDrop {
    pub(crate) fn new(handle: tokio::task::AbortHandle) -> Self {
        Self(handle)
    }
}

impl Drop for AbortOnDrop {
    fn drop(&mut self) {
        if self.0.is_finished() {
            return;
        }
        self.0.abort();
        ::dx_agent_log::record!(
            DEBUG,
            ::dx_agent_log::Event::new(module_path!(), ::dx_agent_log::Action::Kill)
                .with_category(::dx_agent_log::EventCategory::Provider)
                .with_outcome(::dx_agent_log::EventOutcome::Success),
            "stream: consumer dropped — aborting detached parser task to release socket"
        );
    }
}
