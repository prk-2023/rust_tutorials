# Observability:

In the observability world, **Prometheus** and **Grafana** are like a "Librarian" and an "Artist." 

They serve different purposes but are almost always used together.

---

## 1. Prometheus: The "Librarian" (Data Collector)

Prometheus is a **Time-Series Database (TSDB)**. 
Its primary job is to collect and store numbers over time.

* **How it works**: 

    It uses a **"Pull" model**. 
    It reaches out to your application (or a "exporter") every few seconds and asks: 
    * *"What are your current stats?"* 

    * **What it stores**: 
        It stores **Metrics** (numerical data). 
        For your SSH project, this would be things like `ssh_login_attempts_total` or `blocked_ips_count`.

    * **Key Feature**: 
        It has its own query language called **PromQL**, which allows you to do math on your data 
        (e.g., *"Show me the rate of login attempts per second over the last 5 minutes"*).

## 2. Grafana: The "Artist" (Visualizer)

Grafana is a **Visualization Platform**. 
It doesn't store any data itself; it just knows how to talk to databases like Prometheus and draw pretty 
pictures.

* **How it works**: 
    You connect Grafana to Prometheus as a **Data Source**. 
    You then write a PromQL query inside Grafana, and it turns that raw data into a graph, gauge, or heat map.

* **Key Feature**: 
    **Dashboards**. Ex: You can build a single screen that shows your SSH server's health, a world map of
    where attacks are coming from, and a real-time list of recently banned IPs.

---

## How They Work Together (The Architecture)

1. **Your Rust App (The Exporter)**: 
    Your Rust code tracks how many IPs it has banned. 
    It "exposes" this number on a simple web page (usually `http://localhost:9090/metrics`).

2. **Prometheus**: 
    Every 15 seconds, Prometheus visits that page, grabs the numbers, and saves them with a timestamp.

3. **Grafana**: 
    When you open your browser, Grafana asks Prometheus for the last hour of data and draws a line graph 
    showing the attack spikes.

---

## Why this is perfect for "Rust+eBPF Twist":

Instead of just printing logs to a console (which disappear), you can:

1. **Instrument** your Rust code using the `prometheus` crate.
2. **Expose** a `/metrics` endpoint.
3. **Alert**: Use Prometheus to send you a Slack or Discord message automatically if the number of
   SSH attacks suddenly spikes to 100 per second.

-----------------------------------------------------------------------

##  1. Steps to install Prometheus and Grafana 

```bash
# 1. create the pod
podman pod create --name obs-stack -p 9090:9090 -p 3000:3000

# 2. Run Prometheus (with absolute path)
podman run -d --pod obs-stack \
  --name prometheus \
  -v /home/user/vork/ebpf-obs/prometheus/prometheus.yml:/etc/prometheus/prometheus.yml:Z \
  docker.io/prom/prometheus

# 3. Run Grafana WITHOUT a volume mount (for testing)
podman run -d --pod obs-stack \
  --name grafana \
  -e "GF_SECURITY_ADMIN_PASSWORD=admin" \
  docker.io/grafana/grafana

```

### 2. Diagnostic Check

Wait 5 seconds and run:

```bash
podman ps -a

```

**Is the STATUS "Up" for both?** * **If YES:** 

Try `curl -I http://127.0.0.1:3000`. If it says `HTTP/1.1 302 Found`, your browser can now reach it.

* **If NO (Exited):** Run `podman logs grafana`.

### 3. Why "localhost" might fail on some Linux setups

If the container is "Up" but your browser still says "Connection Refused," your Linux distribution might 
not be mapping `localhost` to `127.0.0.1` correctly for rootless containers. 

Try:

* `http://127.0.0.1:3000`
* `http://10.88.0.1:3000` (The Podman Gateway IP you found earlier)

**What does `podman logs grafana` say?** (If it says "Permission Denied," we need to fix the `chown` on your data folder).

-------------------------------------------------------------------------------------------------------
# debug ( on Failure )

## 1. If the above operations fail: 

If `http://localhost:3000` is still failing after a clean restart, it’s likely a 
**Rootless Podman Port Binding** issue or a **silent crash** due to permissions.

Let's use a "Hard Reset" strategy that bypasses common pitfalls.

### 1. The "Nuke" Command

Run this to ensure no ghost processes are holding the ports:

```bash
podman pod rm -f obs-stack || true
podman rm -f prometheus grafana || true

```

### 2. Check for Port Conflicts

Is something else already using port 3000 or 9090 on your host?

```bash
ss -tulpn | grep -E '3000|9090'

```

*If this returns anything, you must stop those services or change the ports in the `podman pod create` command.*

-------------------------------------------------------------------------------------------------------

## Configure both Prometheus and Grafana:


On Successfully setting up prometheus/grafana, next we need to bridge the gap between Grafana and the 
Prometheus instance living in the same pod.

Follow these steps to finish the "Observation" side of your eBPF development environment:

### 1. Connect Grafana to Prometheus

Since they share the same **Podman Pod**, they are on the same network loopback.

1. In the Grafana sidebar, click the **Connections** (plug icon) and select **Data Sources**.
2. Click **Add data source** and choose **Prometheus**.
3. In the **Connection** section, set the URL to: `http://localhost:9090`
* *Note: Do not use your host IP here. Inside the pod, Prometheus is at `localhost`.*


4. Scroll to the bottom and click **Save & Test**.
* You should see a green checkmark: *"Successfully queried the Prometheus API."*

---

### 2. Verify the eBPF Scrape Target

Now, let's see if Prometheus is ready to receive your XDP metrics.

1. In the Grafana sidebar, click the **Explore** (compass icon).
2. In the query box (Metric dropdown), type `up` and hit **Run query**.
3. You should see two results:
* `up{job="prometheus"}`: Value `1` (Healthy)
* `up{job="ebpf-xdp"}`: Value `0` (Down) — **This is expected!**

---

### 3. Why `ebpf-xdp` is "0" (Down)

Your Prometheus configuration is looking for an exporter at `localhost:9435`. 
Since you haven't started your eBPF code or an exporter yet, there is nothing to scrape.

### Next Step: Your XDP Development

How are you planning to export your XDP metrics?

* **Option A:** A custom Go/C/Python script using `prometheus-client`.
* **Option B:** The `ebpf_exporter` by Cloudflare (standard for production).
* **Option C:** Running a containerized exporter inside this same `obs-stack` pod.

----------------------------------------------------------------------------------------------------------

## Observability : Prometheus Metrics 

For XDP development natural evolution of an eBPF project. 

Moving from **Console Logging** (for debugging) to **Prometheus Metrics** (for observability) turns a script
into a professional tool.

To help you bridge this gap, here is the architecture of how your Rust user-space app will act as the 
"middleman."

---

### The "Metrics Bridge" Architecture

1. **Kernel Space (XDP)**: 
    Instead of just `bpf_printk!`, the XDP program updates a **BPF Map** (e.g., an `Array` or `PerCpuArray`)
    with counters like `dropped_packets` or `banned_ips`.

2. **User Space (Rust/Aya)**:
    * It periodically reads the values from that BPF Map.
    * It uses a Rust library (like `prometheus` or `aya-metrics`) to update an internal counter.
    * It runs a tiny web server (like `warp` or `axum`) that Prometheus can scrape.
---

### Step-by-Step Integration Plan

#### 1. The Kernel "Counter" (eBPF)

In your `ssh_defense.ebpf.rs` (or `.c`), you define a map to hold your stats.

```rust
#[map]
static STATS: Array<u64> = Array::with_max_entries(1, 0);

// Inside your XDP logic
let mut count = STATS.get_ptr_mut(0).ok_or(XDP_ABORTED)?;
*count += 1; // Increment packet drop counter

```

#### 2. The User-Space "Exporter" (Rust)

In your `main.rs`, you need to pull that data out and put it into a Prometheus-friendly format. 
You have two good options for the Rust "Twist":

* **Option A: `aya-metrics` (The Fast Way)**

    Aya has a dedicated crate called `aya-metrics` that automatically maps eBPF counters to the `metrics` 
    ecosystem.

* **Option B: `prometheus_exporter` (The Customizable Way)**

    Using the `prometheus` crate directly gives you more control over labels 
    (e.g., labeling by "Source Port" or "Interface").

#### 3. Pointing Grafana to your App

Once your Rust app is running, it will host a page at `http://your-ip:9100/metrics`.

* **Prometheus**: 

    You already set this up in your Pi's `prometheus.yml`. It will "scrape" this page every 5-15 seconds.

* **Grafana**: You will build a panel using a query like:

    `rate(ssh_defense_packets_dropped_total[1m])`

This will show a real-time line graph of how many attacks are being blocked per second.

---

### Recommended Tooling for Step 2

When you are ready to start coding the exporter, I recommend adding these to your `Cargo.toml`:

```toml
[dependencies]
aya = { version = "0.12", features = ["async_tokio"] }
prometheus = "0.13" # The "Artist's" data source
warp = "0.3"       # A lightweight web server to host the /metrics page
tokio = { version = "1", features = ["full"] }

```

---------------------------------------------------------------------------------------------------------

## Todo List:

To ensure your documentation is complete and your team of developers can transition from "Console Logs" to a
"Production Monitoring Stack," here is the checklist of integration points you should add to your document.

These points specifically address the **"How to make it all talk to each other"** gap.

---

### 1. Networking Strategy: The "Host-to-Pod" Bridge

Since your **Rust Loader** runs on the host (Threadripper) and **Prometheus** is in a Podman Pod, they are 
in different networking worlds.

* **Action Item:** 
    Update `prometheus.yml` to use `host.containers.internal` instead of `localhost`.

* **Why:** 
    Inside a Pod, `localhost` only sees other containers in that same Pod. 
    To reach the Rust app on the host machine, you must use the special Podman DNS bridge.

> **Note for Docs:** For this to work in rootless Podman, ensure the pod is created without the `--net=host`
> flag (which is the default).

### 2. eBPF State Management: Moving from Print to Maps

Console logging (`bpf_printk!`) is too slow for high-speed XDP. 
You need a data structure to hold your metrics.

* **Action Item:** Replace `aya_log` with an **eBPF Map** (e.g., `PER_CPU_ARRAY` or `HASH`).
* **Logic:** 1.  Kernel-space (XDP) increments a counter in the Map every time a packet is dropped.
2.  User-space (Rust) "scrapes" that Map every few seconds.

### 3. The "Exporter" Requirement in Rust

Your Rust loader needs to serve as a web server for Prometheus.

* **Action Item:** Add a Prometheus Registry and an HTTP handler to your Rust code.
* **Libraries Needed:** * `prometheus` (for the registry and metrics types).
* `warp` or `axum` (to serve the `/metrics` endpoint on port `9100`).

* **Note for Docs:** 
    Developers must ensure port `9100` is open in the host firewall so the Pi 4 (monitoring) can reach the
    Threadripper (XDP node).

### 4. SELinux & Permissions (The ":Z" Flag)

If your developers are using Fedora, RHEL, or any system with SELinux enabled, standard volume mounts will 
fail.

* **Action Item:** 

    Explicitly mention the **`:Z`** flag in all `podman run` commands.

* **Why:** 

    This flag tells Podman to relabel the host files so the containerized Prometheus can read your 
    `prometheus.yml`.

### 5. Deployment Checklist for Team

Add this "Final Connectivity Test" to the end of your doc:

| Component | Test Command | Success Criteria |
| --- | --- | --- |
| **Rust Loader** | `curl http://localhost:9100/metrics` | Returns text starting with `# HELP...` |
| **Prometheus Pod** | `podman exec prometheus curl host.containers.internal:9100/metrics` | Prometheus can see the host metrics. |
| **Grafana** | Check "Data Sources" in UI | "Successfully queried the Prometheus API." |

---
