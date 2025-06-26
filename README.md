# 🚀 High-Speed Anchoring with PACK CDC Algorithm (Rust)

This repository provides a **Rust implementation of the PACK CDC algorithm**, a high-speed anchoring algorithm for efficient content-defined chunking in data streams.

📄 **PACK CDC** was introduced in the [SIGCOMM 2011 paper](https://dl.acm.org/doi/10.1145/2043164.2018447)  
_“The Power of Prediction: Cloud Bandwidth and Cost Reduction”_  
by my team and myself.

Written in Rust, this implementation takes full advantage of Rust’s zero-cost abstractions and memory safety, yielding performance that surpasses even highly optimized solutions such as **Intel ISA-L**.

---

## 📌 What Is Anchoring?

In data stream processing, **anchoring** refers to the process of identifying stable boundaries (or "anchors") in the stream to divide it into chunks. These anchors serve as fingerprints, enabling:

- Efficient **content-based deduplication**
- **Stream verification** without storing full data
- **Chunk-level integrity** checks in dynamic or distributed systems

---

## ⚡ Why PACK CDC in Rust?

- 🏎️ **Extreme Speed**  
  This implementation outperforms the original Java version and even optimized native libraries, thanks to Rust’s performance and inlining.

- 🧱 **Scalable and Real-Time**  
  Designed for **real-time processing of large data streams**, PACK CDC scales easily across threads and workloads.

- 💡 **Efficient and Predictive**  
  The algorithm was designed to **predict and locate anchors** at high throughput with low CPU and memory usage.

---

## 📦 Installation

```bash
git clone https://github.com/eyalzo/packcdc-rs.git
cd packcdc-rs
cargo build --release
```

Make sure you have [Rust installed](https://www.rust-lang.org/tools/install).

---

## 🧪 Example Usage

Run the included demo:

```bash
cargo run --example packcdc_example
```

This will:

- Generate a random data buffer
- Apply PACK CDC to find anchors
- Print performance metrics and slice samples using `colored` output

---

## 🔍 Potential Applications

- 🔐 **Secure Data Auditing** in cloud storage systems
- 🌐 **Integrity Verification** in content delivery networks (CDNs)
- 📊 **Network Traffic Analysis** using stream-based fingerprinting
- 🧩 **Chunk-based Deduplication** in backup and archival systems

---

## 🧪 Further Research & Contribution Ideas

We welcome ideas, feedback, and pull requests. This project can serve as a solid foundation for further development:

- 🧵 Integration with streaming data frameworks (e.g., Apache Kafka, Fluvio)
- 📈 Comparative benchmarks with other CDC/anchoring algorithms
- 🔎 Exploring novel use cases in **data forensics**, **edge computing**, or **blockchain**

---

## 🤝 Contributing

Contributions are welcome! Feel free to open an issue or a pull request for:

- Bug fixes
- Performance enhancements
- Feature suggestions

---

## 📄 License

MIT License. See [`LICENSE`](./LICENSE) for details.

---

## 🧠 Citation

If you use PACK CDC or this implementation in research or production, please consider citing the original paper:

> E. Zohar, I. Cidon, O. Mokryn  
> “The Power of Prediction: Cloud Bandwidth and Cost Reduction,” SIGCOMM 2011  
> [https://doi.org/10.1145/2043164.2018447](https://doi.org/10.1145/2043164.2018447)

---

## 🙋 Contact

Maintained by [Eyal Zohar](https://github.com/eyalzo)  
Feel free to reach out via GitHub Issues for discussions or questions.