# Session Tool Routing

Version: 1

This note is the source-owned gate for DX Agents session tool routing and interruption semantics. It keeps the first contract intentionally small: the product may report tool routing state and readiness, but must not export tool argument values, tool result payloads, provider secrets, browser storage values, or shell input.

## State Model

- `pending`: the tool call has been announced and is waiting to run.
- `running`: the tool call is executing or waiting for a result.
- `interrupted`: the tool call was stopped by abort, stuck-session recovery, or operator intervention. It is terminal and retryable.
- `completed`: the tool call produced a result. It is terminal and not retryable.

## Interruption Rules

- Dashboard aborts and gateway session abort commands must be treated as interruption sources.
- Stuck-session recovery may interrupt multiple gateway sessions, but must keep local recovery explicit.
- Operators may resolve stale local state only through the existing confirmed `sessions resolve` path.
- Retry accounting may count attempts and interruptions, but must not persist payload values.

## Export Rules

- Status exports may include state ids, readiness checks, counts, and redaction policy.
- Status exports may include tool ids or names when already visible in the transcript.
- Status exports must not include arguments, result bodies, browser storage values, auth tokens, shell input, or payload fingerprints.
