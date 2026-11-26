# Curriculum and prerequisite graph

The table order is the teaching sequence. Every prerequisite edge goes
from an earlier chapter to the chapter that names it. Prerequisites are
direct, so completing a prerequisite also entails its own prerequisites.
The last column identifies the work a reader should be able to verify.
The [table of contents](src/SUMMARY.md) lists chapters with content.

| Stage | Reader's work |
| --- | --- |
| I–IV | Trace machines, reason with finite representations, and write small programs |
| V–IX | Make ownership, types, abstraction, and errors express a contract |
| X–XIII | Derive algorithms and inspect how programs and data are implemented |
| XIV–XVIII | Reason about tasks, memory boundaries, and systems I/O |
| XIX–XXIII | Design services, durable storage, distributed protocols, and trust boundaries |
| XXIV–XXV | Verify, ship, observe, and evaluate complete applications |

The 125 chapters reserve five programming problems and two interview
questions each, for 625 problems and 250 interview questions. Separate
written traces and quizzes can supplement those problems. A quota in
this plan does not count toward shipped content.

## Part I — Computer Science Foundations

| ID | Chapter | Direct prerequisites | Evidence |
| --- | --- | --- | --- |
| C001 | How computers execute instructions | — | Specify a state machine and trace a short input/output program |
| C002 | Bits, binary, hexadecimal, and bounded arithmetic | C001 | Prove positional conversion and check carry across a byte boundary |
| C003 | Boolean algebra and digital logic | C002 | Derive a full adder and distinguish combinational and clocked state |
| C004 | CPU architecture, memory, and assembly | C003 | Decode and trace RV32I operations, branch decisions, and an abstract cache |
| C005 | Operating systems, translation, and models of computation | C004 | Trace loading, processes, threads, virtual memory, and interpreter state |

## Part II — Mathematical Foundations

| ID | Chapter | Direct prerequisites | Evidence |
| --- | --- | --- | --- |
| C006 | Propositions, predicates, and proof | C003 | Prove one claim directly, by contrapositive, and by contradiction |
| C007 | Sets, relations, functions, and graphs | C006 | Model equivalence, composition, reachability, and graph representations |
| C008 | Induction, strong induction, and recurrence relations | C007 | Prove a structural bound and solve a divide-and-conquer recurrence |
| C009 | Counting, probability, and modular arithmetic | C008 | Count arrangements, derive an expectation, and justify modular identities |
| C010 | Asymptotic reasoning and cost models | C008,C009 | Derive tight and amortized bounds and state an average-case distribution |

## Part III — Rust Fundamentals

| ID | Chapter | Direct prerequisites | Evidence |
| --- | --- | --- | --- |
| C011 | Rust history, rustup, rustc, Cargo, and the first program | C005 | Build and run a first program on 1.75.0 and explain each token and tool |
| C012 | Names, mutation, primitive types, and expressions | C002,C011 | Track type inference, constants, statements, and eventful evaluation |
| C013 | Numeric conversions, overflow, and floating point | C009,C012 | Compare checked and wrapping arithmetic, narrowing, and IEEE rounding |
| C014 | Compound types, Unicode, and strings | C012,C013 | Trace tuples, arrays, and UTF-8 through byte and scalar operations |
| C015 | Branches, loops, functions, and recursion | C008,C014 | Derive a terminating loop and implement a recursive alternative |

## Part IV — Memory and Computer Architecture

| ID | Chapter | Direct prerequisites | Evidence |
| --- | --- | --- | --- |
| C016 | Addresses, pointer values, and stack frames | C004,C015 | Trace a call on a named ABI and distinguish abstract and machine state |
| C017 | Stack, heap, static storage, and destruction | C016 | Explain allocation ownership and lifetime through nested scopes |
| C018 | Alignment, padding, endianness, and layout | C013,C016 | Compare wire bytes with a C layout and measure layout on a named target |
| C019 | Hierarchy, allocation, fragmentation, and locality | C005,C010,C017,C018 | Model allocator fragmentation and compare sequential and scattered access |
| C020 | Address spaces, pages, and resource limits | C005,C019 | Trace translation, working sets, page faults, and allocation failure modes |

## Part V — Ownership and Borrowing

| ID | Chapter | Direct prerequisites | Evidence |
| --- | --- | --- | --- |
| C021 | Ownership, moves, Copy, and Clone | C015,C017 | Predict resource effects of moves, copies, clones, and partial moves |
| C022 | Shared and mutable borrowing | C016,C021 | Repair conflicting borrows and explain reborrows and non-lexical lifetimes |
| C023 | Lifetimes, elision, and lifetime bounds | C022 | Derive a function's lifetime relation and distinguish the uses of static |
| C024 | Variance, subtyping, and aliasing | C007,C023 | Explain why mutation changes variance and identify an invalid coercion |
| C025 | RAII, Drop, and resource invariants | C017,C021,C024 | Design a resource guard and trace drop order and deliberate leakage |

## Part VI — Data Modeling

| ID | Chapter | Direct prerequisites | Evidence |
| --- | --- | --- | --- |
| C026 | Structs, products, methods, and associated functions | C007,C021 | Encode a product with named, tuple and unit forms and attach constructors |
| C027 | Enums, sums, destructuring, and exhaustive matches | C026 | Count cases and verify pattern coverage and move behavior |
| C028 | Option, Result, and domain states | C027 | Design distinct types for absence, invalid data, and valid states |
| C029 | Newtypes and type-driven API design | C023,C026,C028 | Enforce unit and validation boundaries through constructors |
| C030 | Typestate and state transitions | C025,C029 | Prove allowed transitions through a typed builder and negative examples |

## Part VII — Type Systems and Abstraction

| ID | Chapter | Direct prerequisites | Evidence |
| --- | --- | --- | --- |
| C031 | Generic functions, data, and parametric reasoning | C026,C028 | Derive what a body may assume about a type parameter |
| C032 | Traits, associated items, and supertraits | C031 | Express an interface with bounds, defaults, associated types and constants |
| C033 | Coherence, orphan rules, and blanket implementations | C029,C032 | Explain and repair overlapping or illegal implementations |
| C034 | Monomorphization, trait objects, and dispatch | C033 | Compare static and dynamic dispatch, object safety, and code size |
| C035 | PhantomData, higher-ranked bounds, and GATs | C024,C034 | Model borrowing families and read a universally quantified bound |

## Part VIII — Functional Programming

| ID | Chapter | Direct prerequisites | Evidence |
| --- | --- | --- | --- |
| C036 | Pure functions and composition | C015,C031 | Refactor effectful steps into immutable transformations and an I/O boundary |
| C037 | Closures and capture | C022,C032,C036 | Explain capture and derive Fn, FnMut, and FnOnce constraints |
| C038 | Iterator contracts and laziness | C032,C037 | Implement a stateful iterator with a justified stopping condition |
| C039 | Map, filter, fold, reduce, and short-circuiting | C028,C038 | Compare eager and lazy pipelines and trace consumed values |
| C040 | Iterator API design and cost | C010,C034,C039 | Verify equivalent outputs and compare allocation and dispatch trade-offs |

## Part IX — Error Handling and Reliability

| ID | Chapter | Direct prerequisites | Evidence |
| --- | --- | --- | --- |
| C041 | Recoverable failures, invariants, and assertions | C025,C028 | Classify errors, internal invariant violations, and expected absence |
| C042 | Propagation, question mark, and error conversion | C033,C041 | Trace branching on Result and preserve underlying failure information |
| C043 | Custom errors, source chains, and context | C034,C042 | Design typed errors with useful context and safe display |
| C044 | Panic, unwinding, and recovery boundaries | C025,C043 | Distinguish unwind, abort, drops, and catch_unwind boundaries |
| C045 | Robust APIs and defensive recovery | C030,C040,C044 | Test error precedence, fallible parsing, rollback, and bounded error paths |

## Part X — Data Structures

| ID | Chapter | Direct prerequisites | Evidence |
| --- | --- | --- | --- |
| C046 | Arrays, dynamic arrays, stacks, and queues | C010,C018,C025,C045 | Compare arrays and Vec, derive growth amortization, and implement stack and queue APIs |
| C047 | Linked lists, doubly linked lists, rings, and deques | C024,C046 | Verify link and index invariants, wraparound, and cache trade-offs |
| C048 | Hash tables, sets, Bloom filters, and LRU caches | C009,C033,C047 | Derive load and false-positive bounds and test collisions and eviction |
| C049 | Binary search trees, AVL, red-black, B/B+ trees, and tries | C007,C008,C046 | Prove ordering and height invariants and test redistribution and rotations |
| C050 | Heaps, priority queues, graphs, disjoint sets, Fenwick and segment trees | C007,C048,C049 | Verify heap restoration, connectivity, and range-query decompositions |

## Part XI — Algorithms and Problem Solving

| ID | Chapter | Direct prerequisites | Evidence |
| --- | --- | --- | --- |
| C051 | Linear and binary search, two pointers, sliding windows, and prefix sums | C010,C045,C046 | Prove half-open interval and window invariants with exact boundary reasoning |
| C052 | Comparison sorting and integer sorting | C008,C050,C051 | Prove bubble, selection, insertion, merge, quick, heap, counting, and radix sorts |
| C053 | Divide and conquer, backtracking, greedy choice, and dynamic programming | C008,C052 | Derive recurrences, pruning, exchange arguments, memoization and tabulation |
| C054 | Graph traversals, shortest paths, and spanning trees | C050,C053 | Prove BFS, DFS, topological sort, Dijkstra, Bellman–Ford, Floyd–Warshall, A*, Kruskal, and Prim |
| C055 | String matching, bit algorithms, and number theory | C009,C014,C052 | Verify KMP, Rabin–Karp, bit masks, Euclid and modular exponentiation |

## Part XII — Advanced Memory Management

| ID | Chapter | Direct prerequisites | Evidence |
| --- | --- | --- | --- |
| C056 | Box, owning indirection, and recursive values | C021,C025,C049 | Measure recursive ownership layout and reason about destructor depth |
| C057 | Rc, Arc, Weak, and ownership graphs | C007,C025,C056 | Trace strong and weak counts and break a reference cycle |
| C058 | Cell, RefCell, Cow, and interior mutability | C022,C024,C057 | Compare copied, runtime-checked, and clone-on-write access |
| C059 | Pin, Unpin, and address-sensitive types | C035,C056,C058 | Specify projection and destruction obligations for an address-sensitive value |
| C060 | Arenas, pools, and allocator design | C019,C045,C059 | Specify recycling and arena lifetimes, and examine stale handle failure |

## Part XIII — Metaprogramming and Compiler Internals

| ID | Chapter | Direct prerequisites | Evidence |
| --- | --- | --- | --- |
| C061 | Tokens, parsing, syntax trees, and diagnostics | C005,C045,C055 | Parse a small grammar and report bounded, located errors |
| C062 | Declarative macros and expansion | C032,C061 | Trace token trees, matching, repetition, hygiene, and expansion |
| C063 | Procedural macros, derive, and attributes | C035,C062 | Validate token streams and surface compile-time input errors |
| C064 | HIR, MIR, borrow checking, and monomorphization | C024,C034,C063 | Map a program to simplified IR and inspect available versioned outputs |
| C065 | LLVM IR, optimization, code generation, linking, and debug information | C018,C064 | Compare captured builds with target, flags, and compiler documented |

## Part XIV — Concurrency and Parallelism

| ID | Chapter | Direct prerequisites | Evidence |
| --- | --- | --- | --- |
| C066 | Threads, scheduling, channels, Send, and Sync | C005,C045,C057 | Validate transfer and shared access with timelines and compiler checks |
| C067 | Mutexes, read-write locks, condition variables, and barriers | C044,C058,C066 | Prove a condition predicate remains guarded through spurious wakeups |
| C068 | Atomics, ordering, and happens-before | C024,C067 | Derive a release/acquire publication argument and show relaxed limitations |
| C069 | Deadlock, livelock, starvation, and logical races | C007,C067,C068 | Analyze lock order and model liveness separately from data race freedom |
| C070 | Thread pools, work stealing, Rayon, and lock-free ideas | C010,C050,C069 | Implement a bounded pool and interpret load balance and progress guarantees |

## Part XV — Asynchronous Programming

| ID | Chapter | Direct prerequisites | Evidence |
| --- | --- | --- | --- |
| C071 | Async I/O, Future, Poll, Waker, and Context | C059,C066 | Trace readiness and explain why a pending future needs notification |
| C072 | State machines, pinning, and an educational executor | C068,C071 | Implement and test a small executor without losing a wake notification |
| C073 | Tokio, task ownership, and async traits | C035,C070,C072 | Test task lifetimes and use Rust 1.75 async traits within their bounds |
| C074 | Cancellation, deadlines, and backpressure | C045,C069,C073 | Exercise partial I/O, cancellation-safe operations, and queue limits |
| C075 | Structured concurrency and graceful shutdown | C070,C074 | Verify a supervised service drains or cancels work by a defined deadline |

## Part XVI — Unsafe Rust and FFI

| ID | Chapter | Direct prerequisites | Evidence |
| --- | --- | --- | --- |
| C076 | Unsafe blocks, raw pointers, and safety contracts | C018,C024,C060 | Justify each access in a small checked safe abstraction |
| C077 | Initialization, aliasing, alignment, and provenance | C059,C068,C076 | Review initialization and pointer validity under the historical model |
| C078 | Unsafe traits, soundness, and abstraction boundaries | C035,C066,C077 | Audit invariants and unsafe trait obligations behind safe methods |
| C079 | C, ABIs, and ownership across FFI | C065,C078 | Specify ownership and layout for a cross-language resource |
| C080 | Error and panic boundaries across FFI | C044,C079 | Validate cleanup and document platform and unwind assumptions |

## Part XVII — Systems Programming

| ID | Chapter | Direct prerequisites | Evidence |
| --- | --- | --- | --- |
| C081 | OS interfaces, files, descriptors, and system calls | C020,C045 | Trace partial operations and compare file descriptors with Windows handles |
| C082 | Processes, signals, terminals, and environments | C075,C081 | Build a supervised child command with bounded output and portable cleanup |
| C083 | IPC, shared memory, and memory-mapped files | C068,C080,C081 | Compare pipe, socket, and mapping boundaries and crash behavior |
| C084 | Embedded systems, no_std, and hardware abstraction | C018,C076,C081 | Test a no_std component under host simulation and document hardware contracts |
| C085 | Portable system interfaces and resource limits | C082,C083,C084 | Design feature and platform boundaries with verified fallback behavior |

## Part XVIII — Networking

| ID | Chapter | Direct prerequisites | Evidence |
| --- | --- | --- | --- |
| C086 | OSI, TCP/IP, IP addresses, TCP, UDP, and sockets | C018,C081 | Trace addressing, streams, datagrams, ports, and packet loss |
| C087 | DNS, serialization, framing, and timeouts | C055,C074,C086 | Test bounded decoding under segmentation and malformed input |
| C088 | HTTP semantics, TLS, and connection reuse | C087 | Trace HTTP message boundaries and verify trust and hostname assumptions |
| C089 | WebSockets, reliability, and connection management | C075,C088 | Test lifetime, protocol transitions, partial I/O, and backpressure |
| C090 | Load balancing, rate limiting, and network capacity | C009,C070,C089 | Derive a limit policy and verify capacity under burst and slow-client load |

## Part XIX — Backend Engineering

| ID | Chapter | Direct prerequisites | Evidence |
| --- | --- | --- | --- |
| C091 | REST, HTTP servers, Axum, and Actix Web | C034,C075,C088 | Compare historically pinned routers and validate method semantics |
| C092 | Middleware, serialization, and request validation | C045,C087,C091 | Design a bounded request path and test error responses |
| C093 | Authentication, authorization, and sessions | C090,C092 | Trace identity and permissions through request and session lifecycles |
| C094 | Pagination, caching, versioning, and OpenAPI | C049,C091,C093 | Verify stable ordering and schema compatibility across API revisions |
| C095 | Background jobs and service architecture | C075,C090,C094 | Implement retries, cancellation, and observable service boundaries |

## Part XX — Database Engineering

| ID | Chapter | Direct prerequisites | Evidence |
| --- | --- | --- | --- |
| C096 | Relational modeling, normalization, and SQL | C007,C029 | Derive keys and dependencies, then verify equivalent SQL results |
| C097 | PostgreSQL, SQLite, indexes, and query optimization | C049,C096 | Compare plans and state the effect of index order and selectivity |
| C098 | Transactions, ACID, and isolation levels | C069,C097 | Construct interleavings and verify isolation assumptions per engine |
| C099 | SQLx, pools, migrations, and service integration | C095,C098 | Verify a pinned schema migration and bound pool acquisition |
| C100 | Persistent storage, WAL, and crash recovery | C083,C098,C099 | Derive flush ordering and verify recovery from a truncated log |

## Part XXI — Distributed Systems

| ID | Chapter | Direct prerequisites | Evidence |
| --- | --- | --- | --- |
| C101 | Partial failures, retries, backoff, and circuit breakers | C009,C090,C095 | Simulate dropped replies and justify bounded retries and idempotency |
| C102 | Replication, partitioning, and consistency models | C098,C101 | Enumerate legal observations and demonstrate a failure under weaker consistency |
| C103 | CAP assumptions, consensus, and leader election | C007,C102 | State the formal CAP setting and test quorum and leader assumptions in a model |
| C104 | Distributed messaging and event-driven architecture | C095,C100,C103 | Validate ordered and duplicate delivery assumptions with recovery tests |
| C105 | Observability and failure simulations | C090,C104 | Correlate traces and use controlled faults to test service hypotheses |

## Part XXII — WebAssembly and Cross-Platform Development

| ID | Chapter | Direct prerequisites | Evidence |
| --- | --- | --- | --- |
| C106 | WebAssembly execution and linear memory | C018,C065 | Trace Wasm validation, bounds, imports, and host capabilities |
| C107 | Rust targets, portability, and cross-compilation | C084,C085,C106 | Compare browser, embedded, and native target constraints |
| C108 | wasm-bindgen and JavaScript interoperation | C079,C107 | Audit memory, string conversion, and ownership across the browser boundary |
| C109 | Browser integration and restricted environments | C074,C089,C108 | Test initialization, data transfer, and error recovery using browser APIs |
| C110 | Portable package and integration boundaries | C085,C109 | Verify capability-based interfaces in native and WebAssembly builds |

## Part XXIII — Security Engineering

| ID | Chapter | Direct prerequisites | Evidence |
| --- | --- | --- | --- |
| C111 | Threat models, memory safety, and defensive boundaries | C078,C092,C105 | Map assets and trust boundaries and review properties of a bounded parser |
| C112 | Authentication security, authorization, and password hashing | C093,C111 | Test authorization and compare documented password-hash parameters |
| C113 | Established cryptography, secure randomness, and secrets | C088,C112 | Use a historical reviewed library and justify nonce, key and secret handling |
| C114 | Dependency and supply-chain security | C063,C099,C113 | Review lockfile, release evidence, advisory scope, and dependency updates |
| C115 | Defensive fuzzing, resource exhaustion, and architecture | C090,C111,C114 | Check parsing invariants and enforce time, depth, byte, and allocation limits |

## Part XXIV — Professional Software Engineering

| ID | Chapter | Direct prerequisites | Evidence |
| --- | --- | --- | --- |
| C116 | Clean interfaces, SOLID in Rust, and design patterns | C030,C045,C095 | Refactor a modular domain model and articulate coupling and API trade-offs |
| C117 | Unit, integration, property, documentation, and model tests | C069,C115,C116 | Explain evidence from test, differential, exhaustive, and model checks |
| C118 | Debugging, profiling, and benchmarking | C065,C070,C117 | Report reproducible performance and diagnose a workload-specific bottleneck |
| C119 | Dependencies, CI/CD, Docker, and deployment | C099,C114,C117 | Reproduce release and rollback with locked dependencies and explicit migrations |
| C120 | Logging, tracing, observability, and maintenance | C105,C118,C119 | Derive operational signals, run a failure drill, and document handover |

## Part XXV — Professional Capstone Projects

| ID | Chapter | Direct prerequisites | Evidence |
| --- | --- | --- | --- |
| C121 | Production-oriented REST API | C099,C112,C119,C120 | Deploy a versioned service with migrations, authorization, and recovery tests |
| C122 | Distributed job queue | C104,C115,C120,C121 | Verify bounded retries, lease expiry, and end-to-end idempotency |
| C123 | Educational distributed key-value store | C100,C103,C122 | Verify replication and failure behavior with a deterministic schedule |
| C124 | Compiler for a small programming language | C055,C065,C116,C117 | Relate grammar, type checking, and code generation through a differential oracle |
| C125 | High-performance concurrent application | C070,C115,C118,C120 | Report throughput, latency, and memory under reproducible workloads |

## Engineering projects

Each project starts after the named chapter prerequisites. A project
prerequisite reserves an existing architecture or test oracle to reuse.
The progression goes from an explicit contract to a working vertical
slice, error handling, tests, and a review of resources and failure.

| ID | Project | Chapters | Project prerequisite | Acceptance focus |
| --- | --- | --- | --- | --- |
| P01 | Command-line calculator | C015,C045 | — | Defined operators, checked arithmetic, exit behavior |
| P02 | Number guessing game | C015,C028 | — | Input validation, reproducible game tests, bounded rounds |
| P03 | Unit conversion utility | C013,C029,C045 | — | Explicit units, rounding, and rejection of invalid measurements |
| P04 | Contact management application | C040,C045 | P03 | Duplicate handling, persistence format, and I/O error behavior |
| P05 | Task management CLI | C045,C048 | P04 | Stable identifiers, atomic update policy, integration tests |
| P06 | File search utility | C045,C081 | P05 | Path and permission behavior, encoding, and bounded traversal |
| P07 | Log analysis tool | C040,C051,C081 | P06 | Streaming aggregation, invalid records, and known memory cost |
| P08 | Markdown parser | C045,C061 | — | Documented subset, source spans, and differential fixtures |
| P09 | JSON parser | C055,C061 | P08 | Defined semantics for nesting, Unicode, numbers, and errors |
| P10 | Compression utility | C050,C055,C081 | P07 | Round trips, malformed framing, decompression output limits |
| P11 | Multithreaded HTTP server | C070,C087,C088 | — | Bounded pool, request limits, partial I/O, shutdown |
| P12 | Terminal text editor | C047,C082 | P06 | Editing cursor model, terminal portability, and edit recovery |
| P13 | Asynchronous network server | C075,C087 | P11 | Backpressure, deadlines, bounded connections, shutdown |
| P14 | Reverse proxy | C089,C090 | P13 | Configured upstreams, bounded forwarding, and failure behavior |
| P15 | Key-value database | C048,C097 | P09 | CRUD semantics, error recovery, and complexity checks |
| P16 | Persistent storage engine | C083,C100 | P15 | Versioned on-disk format, flush contract, crash fixtures |
| P17 | HTTP client | C088,C090 | P11 | Redirect policy, TLS trust, body limits, and timeouts |
| P18 | Secure file encryption utility | C081,C113 | — | Reviewed authenticated encryption and transactional writes |
| P19 | WebAssembly application | C110 | — | Verified browser build and typed JavaScript boundary |
| P20 | Programming language interpreter | C035,C061 | P09 | Explicit evaluation model, error spans, and resource limits |
| P21 | Production-oriented REST API | C121 | P05,P16 | Migration, roles, and deployment verification |
| P22 | Distributed job queue | C122 | P21 | Delivery semantics, idempotency, and replay tests |
| P23 | Educational distributed key-value store | C123 | P16,P22 | Consistency model with repeatable failure schedules |
| P24 | Small-language compiler | C124 | P20 | Executable backend and interpreter cross-check |
| P25 | High-performance concurrent application | C125 | P07,P13 | Workload design, resource bounds, scalability, and measured variation |

## Assessment checkpoints

| Checkpoint | Chapters | Work |
| --- | --- | --- |
| Fundamentals midterm | C001–C015 | State trace, direct proof, handwritten Rust trace, and diagnostic correction |
| Core Rust midterm | C016–C045 | Resource ownership trace, API repair, and design rationale |
| Algorithms practical | C046–C065 | Implementation, invariant proof, complexity derivation, and compiler observation |
| Systems practical | C066–C105 | Timeline analysis, integration debugging, and failure recovery |
| Final examination | C001–C120 | Design and review a bounded service with typed failures and independent tests |
| Capstone evaluation | C121–C125 | Demonstrate working behavior, audit constraints, reproduce tests, and defend the design |
