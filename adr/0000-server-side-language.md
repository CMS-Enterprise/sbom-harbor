---
# Server Side Language Selection
parent: Decisions
nav_order: 100
title: Justification of Rust

### status: accepted
### date: 2023-02-06
### deciders: Derek Strickland, Quinn Peters, Richard Jones
### consulted: N/A
### informed: N/A
---

# Use Rust for Server Side Code

## Context and Problem Statement

We decided to select a new server side language for our next generation of the Harbor platform because of the following issues we ran
into using Python and Lambda.  

* We ran into issues related to large SBOM upload size. API Gateway has a hard limit and we were hitting it on occasion. The team
  began investigating a Fargate based approach for the single SBOM upload endpoint. 
* We started to hit zip artifact size limits on every Lambda due to vendoring in large amounts of non-compiled Python libraries.
* Because we hit that limit, and we were already planning on having a Fargate container, we began the discussion moving off of Lambdas entirely.
* Moving away from Lambdas would mean a significant amount of code rewrite. Lambdas + API Gateway result in unique runtime that is not
  native HTTP. Because of that, a significant amount of the existing code was concerned with handling these programming paradigms. The code 
  could not simply be lifted and shifted to a different project structure. It would have to be largely rewritten no matter
  what in order to move away from the Lambda/API Gateway approach.
* This led to the conversation about whether selecting a different language was an option.

## Decision Drivers

* The team has a strong preference for the compile-time safety guarantees strongly-typed languages provide.
* The team has a strong preference for languages that encourage code reuse.
* The team has a strong preference for languages with first-class generics support.
* Because of the nature of the problem domain, the team has a strong preference for a language with striong memory safety guarantees.
* The Python ecosystem has a bad security reputation. No study 100% unbiased, but there is [some support](https://www.mend.io/most-secure-programming-languages/)
  for that notion. Given that we'd need to rely on OSS dependencies from that ecosystem, this bubbled up as a concern.

## Considered Options

* Python
* Java
* C#
* Go
* Rust

## Decision Outcome

Chosen option: "Rust", because it:

* Provides the highest level of native memory safety.
* Provides the highest level of thread safety.
* Emphasizes correctness of implementation at compile time rather than simply enforcing runtime error handling.
* Is an active, evolving language and ecosystem with best of breed native tooling.

### Positive Consequences

* Rust enforces memory-safety at compile-time.
* The design of the language encourages, and in most common cases enforces, correctness during development when defects
  are less-expensive to remediate. Other languages emphasize exception handling which means the defect has already reached
  the runtime.
* Rust traits allow default function sharing across implementations.
* Rust has strong support for code generation and aspect oriented programming via generics and macros. This reduces boilerplate,
  copy/paste errors, and inconsistency of implementation. It also promotes early detection of defects.
* Rust enforces exhaustive pattern matching. The compiler will not allow unhandled cases when handling matchable variants
  (Enums, Options, Results). This ensures all cases are addressed at development time, and do not result in a runtime error.
* Rust enforces comprehensive field initialization of structs. This can be leveraged to simplify type conversion and obviate
  a common source of runtime defects entirely (i.e. the new field problem).
* Rust error handling is very elegantly designed, and leads to easy to read code without a lot of vertical scrolling or
  boilerplate typing.
* Rust includes a built-in tool called `rustdoc` that makes it simple to write documentation that is actually integrated
  with the code. With `rustdoc`, you can add examples to the documentation that will actually be run as tests and fail a
  build if the example is no longer valid. This creates a natural trigger in terms of mitigating documentation staleness.
* Rust compiles to native machine code, this simplifies the developer portability story because there is no runtime that
  needs to be configured in an OS or version specific way.

### Negative Consequences

* The real and perceived cost of the Rust learning curve. Rust is still saddled with a reputation of taking time to learn.
  While the tooling has greatly advanced in the past couple of years making this less of an issue, both Rust and Go are
  part of a newer generation of languages that innovate on both OOP and functional programming. Some aspects are familiar
  and some aren't.

## Pros and Cons of the Options

### Python

* Good, because it would raise less concern with non-technical team members.
* Bad, because it is not strongly-typed. The team were disatisfied that they already found themselves spending too much time
  working on defects that a strongly-typed language would have helped to prevent.
* Bad, because it has no native generics capability.
* Bad, because it was not on the list of memory safe languages.
* Bad, because getting a consistent developer experience across 3 different operating systems with variable organization security policies
  was difficult enough that it was never achieved.
* Bad, because Python was not the desired language of the first engineer on the project, Java was. However, after some initial work the 
  slow start times of Java-based Lambdas became a show-stopper. 
* Bad, because Python was selected out of a desire to move fast on demonstrating value with the POC, but none of the engineers on the team
  are actually native Python engineers.

### Java

* Good, because it was on the list of memory safe languages.
* Good, because it is strongly-typed.
* Good, because it has first class generics support.
* Bad, because of the developer ergonomics (e.g. code bloat, readability, exception handling rather than correctness).
* Bad, because we only have one core team member that has experience in Java.
* Bad, because it does not natively detect overflows.
* Bad, because it allows data races (i.e. is not natively thread safe).
* Bad, because you can easily create memory leaks.
* Bad, because the runtime setup doesn't help with our machine portability problem.
* Bad, because the language is not evolving as rapidly as other options.

### C#

* Good, because it was on the list of memory safe languages.
* Good, because the language is consistently evolving.
* Good, because it is strongly-typed.
* Good, because it has first class generics support.
* Bad, because of the developer ergonomics (e.g. code bloat, readability, exception handling rather than correctness).
* Bad, because we only have one core team member that has experience in C#.
* Bad, because it does not natively detect overflows.
* Bad, because it allows data races (i.e. is not natively thread safe).
* Bad, because you can easily create memory leaks.
* Bad, because the runtime setup doesn't help with our machine portability problem.
* Bad, because the default package manager NuGet is notoriously difficult to work with.

### Go

* Good, because it was on the list of memory safe languages.
* Good, because the initial learning curve is not too bad.
* Good, because it is strongly-typed.
* Good, because we have two core team members that have experience in Go. Caveat: only one of them is primarily writing code.
* Good, because, while it still requires a runtime, that is compiled and packaged with the binary, leading to less
  portability issues.
* Bad, because generics have only recently been introduced, the implementation is incomplete, and there is not yet a lot
  of high quality examples to reference.
* Bad, because the language designers didn't really want to include generics, however the community essentially demanded
  it. Because they never intended to include generics, they are having some issues with the fundamental language design
  that is making it difficult for them to retrofit generics in after the fact.
* Bad, because of the developer ergonomics (e.g. code bloat, readability, exception handling rather than correctness).
* Bad, because the `if err != nil` boilerplate is cognitive noise when reading code.
* Bad, because the `if err != nil` approach leads to inconsistent error handling.
* Bad, because Go interfaces do not allow default function implementations (no code sharing for interfaces).
* Bad, because the Go community buys heavily into copy/paste programming, which we do not.
* Bad, because it does not natively detect overflows.
* Bad, because it allows data races (i.e. is not natively thread safe).
* Bad, because you can easily create memory leaks.
* Bad, because the default package manager `go mod` is notoriously difficult to work with.

### Rust

* Good, because it was on the list of memory safe languages.
* Good, because the language is consistently evolving.
* Good, because it is strongly-typed.
* Good, because it has first class generics support.
* Good, because it encourages, and the majority of cases, enforces correctness at compile time.
* Good, because by enforcing ownership at compile-time, the language teaches engineers to think in a safer way. This
  mindset is transferable when programming in other languages.
* Good, because of the developer ergonomics (e.g. terse/readable code, error handling).
* Good, because its error handling approach lends itself to consistency and centralization.
* Good, because Rust traits allow default function implementations (code sharing for interfaces).
* Good, because it natively detects overflows.
* Good, because it does not allow data races (i.e. is natively thread safe).
* Good, because the default package manager `cargo` is well-documented, easy to work with in the vast majority of cases,
  and has a rich plugin ecosystem.
* Good, because `rustdoc` helps prevent documentation drift natively.
* Neutral, because while you cannot easily create memory leaks, it is still possible to do so.
* Neutral, because while in the vast majority of cases, compiling to machine-code obviates the issues related to runtime
  portability, some prevalent libraries (OpenSSL) still require OS related hoop jumping.
* Bad, because we only have one core team members that has experience in Rust.
* Bad, because of the both real and imagined learning curve.
* Bad, because of compile times.
