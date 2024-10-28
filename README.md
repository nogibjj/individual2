# Individual2: Rust Project with SQLite Integration

## **Overview**

This project is a **command-line application** built with **Rust** and **SQLite**. It demonstrates how to manage financial transactions using **SQLite as the database** backend, along with **Rust's CLI capabilities**. The project includes:
- **CRUD operations** for transactions.
- A **3-day rolling average** calculation of daily transaction totals.
- Integration with **GitHub Actions** for continuous integration (CI).
- Documentation of how **GitLab Copilot (LLM)** assisted during development.

---

## **Features**

1. **Add Transactions:** Insert new transactions into the SQLite database.
2. **Calculate Rolling Averages:** Calculate the **3-day rolling average** for transactions.
3. **Database Operations:** Store, query, and manage transactions with **SQLite**.
4. **CI Automation:** Automatically build, test, and release with **GitHub Actions**.
5. **LLM Integration:** Assisted by **GitLab Copilot**, which guided design, coding, and troubleshooting.

---

## **Project Dependencies**

Make sure you have **Rust** and **SQLite** installed.

### **`Cargo.toml` Dependencies:**

```toml
[dependencies]
rusqlite = "0.29.0"
```

- **`rusqlite`**: A library to interact with **SQLite databases** in Rust.

---

## **Setup and Installation**

1. **Clone the Repository:**

```bash
git clone <repository-url>
cd individual2
```

2. **Install Rust (if not installed):**

Follow instructions from [Rust's official website](https://www.rust-lang.org/tools/install).

3. **Build the Project:**

```bash
cargo build --release
```

4. **Create the SQLite Database:**

The program will automatically create `my_database.db` inside the `database/` directory during the first run.

---

## **How to Run the Program**

### **1. Add a Transaction**

```bash
cargo run --release add "2021-01-31 00:09:01" 23.10
```

**Output:**
```
✅ Transaction added: 2021-01-31 00:09:01 - $23.10
```

### **2. Calculate 3-Day Rolling Average for January 31, 2021**

```bash
cargo run --release avg
```

**Output:**
```
📊 3-Day Rolling Average for 2021-01-31: $21.19
```

---

## **How to Use SQLite CLI to Verify Data**

```bash
sqlite3 database/my_database.db
```

**Query the Data:**
```sql
SELECT * FROM transactions;
```

---

## **GitHub Actions Workflow**

We use **GitHub Actions** to:
- **Build and test** the project on every push or pull request.
- **Upload a release binary** as an artifact for download.

### **Workflow File: `.github/workflows/rust_ci.yml`**


---

## **How GitLab Copilot Helped in the Process**

During development, **GitLab Copilot** (LLM) played a crucial role by:
1. **Project Structure Guidance**: Suggested how to separate logic between `main.rs` and `db.rs`.
2. **Code Snippet Generation**: Helped generate **SQLite CRUD operations** and window function queries for the **3-day rolling average**.
3. **Troubleshooting Assistance**: Provided solutions for **type mismatches** and **path errors**.
4. **Workflow Automation**: Generated the **GitHub Actions workflow** to automate builds, tests, and artifact uploads.
5. **Documentation Support**: Assisted with creating clear and detailed documentation for the **README** and **Makefile**.

---

## **Makefile for Convenience**

We added a **Makefile** to simplify common tasks:

```Makefile
build:
	cargo build --release

run:
	./target/release/individual2

test:
	cargo test
```

Use the following commands for convenience:

- **Build:** `make build`
- **Run:** `make run`
- **Test:** `make test`

---

## **Video Demo**

Watch the demo video here: 

---

## **Conclusion**

This project showcases how **Rust** and **SQLite** can be used together to build a robust command-line application. By leveraging **LLM assistance** from GitLab Copilot, we were able to:
- Speed up development.
- Automate CI workflows.
- Generate clean, maintainable code.



## References
* [rust-cli-template](https://github.com/kbknapp/rust-cli-template)
* https://github.com/nogibjj/rust-data-engineering
* https://docs.rs/sqlite/latest/sqlite/
* https://github.com/fivethirtyeight/data


