# Iris.browser — Browser Automation

> A high-level browser automation module for the Iris framework.  
> Powered by **DrissionPage** for reliable and fast headless/headed browser control.

---

## Overview

Iris.browser provides a simplified interface to control web browsers (Chromium-based like Chrome, Edge, and Ungoogled Chromium). It integrates seamlessly with the **Zapper** MITM proxy for traffic interception and modification.

Instead of traditional Selenium or Playwright, it uses [DrissionPage](https://github.com/g1879/DrissionPage), which combines the power of raw CDP (Chrome DevTools Protocol) with the ease of `requests`.

---

## Core Logic Engine: `blueprint-rs`

The underlying node-based logic engine for Iris is powered by a high-performance Rust core: **`blueprint-rs`**.

### Documentation

For detailed information on the Rust internal logic and classes, refer to the following documentation files in the `docs/` folder:

- [Python API Entry Point (`lib.rs`)](file:///c:/Users/Azzo/Documents/My%20Projects/blueprint-rs/docs/lib.md)
- [Node Implementation (`node.rs`)](file:///c:/Users/Azzo/Documents/My%20Projects/blueprint-rs/docs/node.md)
- [Port Identification (`port.rs`)](file:///c:/Users/Azzo/Documents/My%20Projects/blueprint-rs/docs/port.md)
- [Relationship Mapping (`connection.rs`)](file:///c:/Users/Azzo/Documents/My%20Projects/blueprint-rs/docs/connection.md)
- [Type System (`datatype.rs`)](file:///c:/Users/Azzo/Documents/My%20Projects/blueprint-rs/docs/datatype.md)

---

## Requirements & Prerequisites

### Systems
- **OS**: Windows, macOS, or Linux.
- **Browser**: Chrome, Edge, or Any Chromium-based browser.

### Build Tools & Libraries
- **Rust Toolchain**: To compile the `blueprint-rs` core.
- **Python 3.10+**: Main environment for Iris orchestration.
- **DrissionPage**: Required for browser control.
- **Maturin**: Recommended for building and installing the Rust extension.

---

## Installation

### 1. Building the Rust Core
Navigate to the root of `blueprint-rs` and build the Python extension:
```bash
maturin develop
```

### 2. Python Dependencies
Ensure your virtual environment is active:
```bash
# Windows
.venv\Scripts\activate

# Linux / Mac
source .venv/bin/activate

pip install drissionpage
```

---

## Architecture

```
┌──────────────────────────────────────────────────────────┐
│                      Iris.browser                        │
│                                                          │
│   Browser (browser.py)        Proxy (proxy.py)           │
│   ├── start()                 ├── host                   │
│   ├── stop()                  ├── port                   │
│   └── page (ChromiumPage)     └── protocol               │
│               │                                          │
│               │                                          │
└───────────────┼──────────────────────────────────────────┘
                │
┌───────────────▼──────────────────────────────────────────┐
│                    DrissionPage                          │
│                                                          │
│   ChromiumPage                ChromiumOptions            │
│               │                                          │
└───────────────┼──────────────────────────────────────────┘
                │
┌───────────────▼──────────────────────────────────────────┐
│                    Chromium Browser                      │
│                                                          │
│   Chrome / Edge / Ungoogled Chromium                     │
│                                                          │
└──────────────────────────────────────────────────────────┘
```

---

## Usage Examples

### Simple Navigation

```python
from Iris.browser import Browser

with Browser() as browser:
    page = browser.page
    page.get("https://httpbin.org/get")
    print(page.html)
```

### Integrated with Zapper Proxy

```python
from Iris.core.zapper import Zapper
from Iris.browser import Browser, Proxy

# 1. Start Zapper
with Zapper(port=8080) as zapper:
    print(f"Zapper listening on {zapper.proxy_url}")
    
    # 2. Configure Browser
    proxy_config = Proxy(host="127.0.0.1", port=8080)
    
    # 3. Start Browser
    with Browser(proxy=proxy_config) as browser:
        browser.page.get("https://example.com")
```

---

## Further Reading

- [DrissionPage Documentation](https://drissionpage.cn/en/) (Comprehensive API reference).
- [Zapper Documentation](zapper.md) (Detailed MITM proxy engine info).
