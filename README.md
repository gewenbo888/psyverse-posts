# Psyverse Bounty Hunter

A tiny, **production‑ready** Rust CLI that scans GitHub for open issues that mention a bounty.  
It is one of the 13 micro‑agents described in the *Psyverse Cash Matrix* architecture.

## Features

- Uses the **GitHub GraphQL API** for efficient searching.
- Async, powered by **Tokio** and **reqwest** (rustls TLS).
- Minimal output: repository, title and direct URL.
- Graceful error handling when the `GITHUB_TOKEN` is missing or the API returns an error.
- Fully tested (unit + integration).

## Usage

