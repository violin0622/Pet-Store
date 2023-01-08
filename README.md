# pet-store
Playground about Rust, Tokio, tonic, etc.

Aims to build a full feature gRPC service.

## Features
- [x] Unary call
- [ ] Client streaming call
- [x] Server streaming call
- [ ] Binary streaming call
- [ ] PostgreSQL backend (Dissel)
  - [x] diesel migration
  - [x] CURD AST
  - [ ] async connection 
  - [x] onnection pool
  - [ ] SQL debug log
  - [ ] OpenTelemetry integrition
- [x] Multi-thread tokio runtime
- [ ] OpenTelemetry support
  - [ ] Tracing
  - [ ] Metrix
  - [ ] Logging
- [ ] Complete errors define(using `anyhow` and `thiserror`)

