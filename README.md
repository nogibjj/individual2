[![Rust CI](https://github.com/nogibjj/individual2/actions/workflows/rust_ci.yml/badge.svg)](https://github.com/nogibjj/individual2/actions/workflows/rust_ci.yml)
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

## **Video Demo**

- Documentation 
---

## **Project Dependencies**

 **Rust** and **SQLite** installed and in Cargo.toml

```toml
[dependencies]
rusqlite = "0.29.0"
```
---

## **Run the Program**


**Add a Transaction**

```bash
./target/release/individual2 add "2021-01-31 00:09:01" 23.10
```

**Update a Transaction**
```
./target/release/individual2 add "2021-01-31 00:09:01" 23.10

```
**Delete a Transaction**

```bash
./target/release/individual2 delete 1

```

**List Transactions**
```
./target/release/individual2 list

```

---

## **Use SQLite CLI to Verify Data**

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

 **Workflow File: `.github/workflows/rust_ci.yml`**


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


