# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

A real-time chat service written in Rust. The service handles user authentication via secret URLs, real-time messaging via a message queue, chat history storage, online/offline presence detection, and image sharing (via external object_store_rust service).

## Build & Run Commands

- **Build:** `cargo build`
- **Run:** `cargo run`
- **Test all:** `cargo test`
- **Test single:** `cargo test <test_name>`
- **Check (fast compile check):** `cargo check`
- **Lint:** `cargo clippy`
- **Format:** `cargo fmt`

## Rust Edition

Uses Rust edition 2024 (requires Rust 1.85+).

## Architecture

Project is in early development. See `design_doc.md` for planned functionality and system component decisions still to be made:

- **Authentication:** Secret URL/hash-based login (email delivery planned for later)
- **Real-time messaging:** Message queue-based inbox pattern — sender writes to recipient's inbox; online recipients consume immediately, offline messages are stored until reconnection
- **Storage:** Chat history (between two parties) and recipient history (previous contacts) — storage backend TBD
- **Presence detection:** Service monitors whether users are online/offline
- **Image support:** Delegates to external [object_store_rust](https://github.com/hotlatteiceamericano/object_store_rust) service
Message 

## Notes
Claude should provide research and suggestion first before implementing the code
