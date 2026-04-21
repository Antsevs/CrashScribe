# CrashScribe

**CrashScribe** is a lightweight Windows system monitoring and crash analysis tool that records system behavior leading up to failures and generates human-readable root cause reports after reboot.

## 🚀 Vision

Modern systems provide logs, metrics, and crash dumps—but they rarely answer the most important question:

> **“Why did my system crash?”**

CrashScribe bridges that gap by:
- passively monitoring system health
- recording a rolling history of telemetry
- detecting abnormal events (crashes, reboots, failures)
- analyzing system behavior leading up to the incident
- generating clear, actionable explanations

---

## 🧠 Core Concept

CrashScribe acts as a **local crash forensics assistant**.

### Before a crash
- Runs in the background
- Samples key system metrics
- Stores a rolling window of recent activity

### After a crash
- Detects abnormal shutdown or failure
- Reconstructs a timeline of events
- Applies diagnostic rules
- Outputs a structured report with likely causes

---

## ⚙️ Key Features (Planned)

### 1. Telemetry Monitoring
Continuously records system state, including:
- CPU usage and temperature
- Memory usage
- Disk activity
- GPU utilization and thermals (future)
- System load patterns

### 2. Event Tracking
Integrates with OS-level signals:
- unexpected shutdowns
- system errors
- hardware faults
- driver issues
- application crashes

### 3. Incident Detection
Identifies events such as:
- hard reboot
- system crash (BSOD)
- application crash
- system instability patterns

### 4. Root Cause Analysis
Uses rule-based inference to determine likely causes:
- thermal issues
- driver instability
- hardware faults
- memory instability
- power-related shutdowns

Outputs:
- ranked hypotheses
- confidence levels
- supporting evidence

### 5. Human-Readable Reports
Generates clear summaries like:

**Likely Cause:** GPU Driver Instability
**Confidence:** High
*Reasoning*
* Driver reset event detected before crash
* GPU temperature spike observed
* Recent driver update detected
*Suggested Fixes*
* Roll back GPU Driver
* Check cooling
* Distable overclocking
---

## 🏗️ Architecture Overview

CrashScribe is designed as a modular Rust application:
```
crashscribe/
│
├── agent/
│ ├── collectors/ # telemetry collection
│ ├── eventlog/ # OS event integration
│ ├── storage/ # SQLite + ring buffer
│ ├── analysis/ # rules + scoring engine
│ └── main.rs # entry point
│
├── shared/
│ └── models/ # data structures
│
├── reports/ # generated reports
└── README.md
```

---

## 🧩 System Design

### Telemetry Model
CrashScribe stores time-series data:

- timestamp
- cpu usage
- memory usage
- temperature (when available)
- disk I/O

Uses a **rolling buffer strategy**:
- high-resolution recent data
- lower-resolution historical summaries

### Storage
- SQLite database
- bounded size (automatic pruning)
- structured event + telemetry tables

### Analysis Engine
- rule-based scoring system
- correlates telemetry + events
- produces ranked hypotheses (not absolute answers)

---

## 🛠️ Tech Stack

- **Language:** Rust
- **Database:** SQLite
- **Platform:** Windows (initial target)
- **UI (initial):** CLI / generated reports
- **Future UI:** Tauri or web-based dashboard

---

## 📦 MVP Scope

The initial version focuses on:

- [ ] Basic telemetry sampling (CPU, memory)
- [ ] Persistent storage (SQLite)
- [ ] Detection of abnormal shutdown
- [ ] Event log parsing (basic)
- [ ] Simple rule-based diagnosis
- [ ] Text-based crash report

---

## 🧪 Future Enhancements

- GPU telemetry integration
- Advanced hardware diagnostics
- Pattern recognition across multiple crashes
- Local LLM-powered explanations
- Cloud-based anonymized crash pattern database (opt-in)
- Full desktop UI
- Linux support

---

## 🔐 Privacy Principles

CrashScribe is designed to be:
- **local-first**
- **transparent**
- **non-invasive**

It does **not**:
- capture keystrokes
- inspect personal files
- monitor network content
- track user behavior

It only collects system health and crash-related metadata.

---

## 🎯 Goals

- Make crash diagnostics understandable
- Reduce time spent debugging system issues
- Provide actionable insights, not raw logs
- Remain lightweight and unobtrusive

---

## 🧠 Learning Goals (Project Context)

This project is also a systems engineering exercise focused on:
- Rust development
- OS-level interaction
- telemetry and monitoring design
- data modeling and storage
- building reliable background services

---

## 🚧 Status

**Early development (MVP phase)**

---

## 🤝 Contributing

Currently a personal project. Structure and contribution guidelines may evolve over time.