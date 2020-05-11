# Teaclave: A Universal Secure Computing Platform

[![License](https://img.shields.io/badge/license-Apache-green.svg)](LICENSE)
[![Coverage Status](https://coveralls.io/repos/github/apache/incubator-teaclave/badge.svg?branch=master)](https://coveralls.io/github/apache/incubator-teaclave?branch=master)

Apache Teaclave (incubating) is an open source ***universal secure computing***
platform, making computation on privacy-sensitive data safe and simple.

## Highlights

- **Security**:
  Teaclave adopts multiple security technologies to enable secure computing, in
  particular, Teaclave uses Intel SGX to serve the most security-sensitive tasks
  with *hardware-based isolation*, *memory encryption* and *attestation*.
  Also, Teaclave is written in Rust to prevent *memory-safety* issues.
- **Functionality**:
  Teaclave is provided as a *function-as-a-service platform*. With many built-in
  functions, it supports tasks like machine learning, private set intersection,
  crypto computation, etc. In addition, developers can also deploy and execute
  Python scripts in Teaclave. More importantly, unlike traditional FaaS,
  Teaclave supports both general secure computing tasks and *flexible
  single- and multi-party secure computation*.
- **Usability**:
  Teaclave builds its components in containers, therefore, it supports
  deployment both locally and within cloud infrastructures. Teaclave also
  provides convenient endpoint APIs, client SDKs and command line tools.
- **Modularity**:
  Components in Teaclave are designed in modular, and can be easily embedded in
  other projects.

## Getting Started

### Try Teaclave

- [My First Function](docs/my-first-function.md)

### Design

- [Threat Model](docs/threat-model.md)
- [Mutual Attestation: Why and How](docs/mutual-attestation.md)

### Codebase

- [Teaclave Services](services)
- [Configurations in Teaclave](config)
- [Data Center Attestation Service](dcap)
- [Keys and Certificates](keys)
- [Dockerfile and Compose File](docker)

## Contributing

Teaclave is open source in [The Apache Way](https://www.apache.org/theapacheway/),
we aim to create a project that is maintained and owned by the community. All
kinds of contributions are welcome.

## Community

- Join us on our [mailing list](https://lists.apache.org/list.html?dev@teaclave.apache.org).
