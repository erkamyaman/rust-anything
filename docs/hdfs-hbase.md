# Apache HDFS and HBase: what rust-anything can and cannot do today

Answer to [#472](https://github.com/erkamyaman/rust-anything/issues/472). Back to the
[whole-apps guide](whole-apps.md). Nothing here was run on HDFS or HBase: this says which
existing commands apply, from what the tool does, and where it stops. The first measured result
belongs in [STATUS.md](STATUS.md) once there is one.

## The short answer

rust-anything does not write a distributed filesystem or a database. It orders a port, records
what the original does, and checks every Rust unit against that recording; the agent writes the
Rust. A rewrite of HDFS or HBase as a whole is far beyond one port: they are Java systems of many
processes (NameNode, DataNode, HMaster, RegionServer), with consensus, replication and failure
handling that no recording of a few thousand requests proves. What can be done is to port **one
piece at a time**, starting with pieces that are pure functions of their input.

## What applies today

| Piece | How it is checked | Status in rust-anything |
| --- | --- | --- |
| A pure Java function or class (a checksum, a block or key encoding, a path or name normaliser, a comparator) | `bridge` (JNI), `record`, `diff`, `gate`, `bench` | Java units are supported ([STATUS.md](STATUS.md)); this is the place to start |
| A command-line tool run against one local input (a file-format dump, a key-range calculator) | `app record`, `app replay`, `front-door` | CLIs are supported |
| The HTTP surfaces: WebHDFS on the NameNode and DataNode, the HBase REST gateway | `app record --http`, `app replay`, `app shadow` | HTTP servers are supported, with `--fresh` for state kept in files and `--ready-timeout` for a slow JVM start |
| A running cluster's behaviour across nodes (replication, failover, region splits, the write-ahead log under crashes) | nothing in rust-anything | Not covered |

## Where it stops

- **The native protocol.** HDFS and HBase clients talk protobuf RPC over TCP. `app record` and
  `app shadow` record CLIs and HTTP only, so the RPC path cannot be recorded or replayed yet.
  Supporting it would be new work in the tool, not a port.
- **Cluster state.** A recording starts the original fresh for every case. A NameNode's namespace,
  block reports from DataNodes and HBase's regions live across processes and across time; one
  process started per case does not exercise them.
- **Timing and failure.** Equal answers on recorded requests say nothing about what happens when a
  node dies mid-write. `gate` and `diff` check recorded behaviour, not correctness under faults.
- **Size.** Each unit has to be bridged, recorded and ported; for a codebase this large that is
  many units over a long time, and the "port it" list comes from `analyze`, not from this page.

## A path that fits the tool

1. Run `analyze` on the Hadoop or HBase source and read the ranked units; take the leaves with no
   I/O.
2. Port two or three of them (an encoding or a checksum is a good first pick), each with
   `bridge`, `record`, `diff`, `gate` and `bench`, and keep the Java caller working through the
   bridge.
3. Stand up one NameNode with WebHDFS, or one HBase REST gateway, record a workload with
   `app record --http` and `--fresh`, and check a Rust front door against it with `app replay`.
4. Only then consider the RPC path and multi-node behaviour, which need tool work first.

Each step gives numbers (cases matched, `gate` result, `bench` ratio) that can be published as
they are; none of them is a claim that "HDFS is in Rust".
