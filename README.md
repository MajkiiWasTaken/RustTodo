# 🦀 RustTodo

<p align="center">
  <img src="https://img.shields.io/badge/Rust-2024-orange?style=for-the-badge&logo=rust" />
  <img src="https://img.shields.io/badge/Axum-0.8-blue?style=for-the-badge" />
  <img src="https://img.shields.io/badge/Tokio-Async-success?style=for-the-badge" />
  <img src="https://img.shields.io/badge/Frontend-Vanilla%20JS-yellow?style=for-the-badge&logo=javascript" />
  <img src="https://img.shields.io/badge/License-MIT-green?style=for-the-badge" />
</p>

<p align="center">
  <b>A lightweight Todo REST API built with Rust, Axum and Tokio.</b><br>
  Includes a simple web interface for managing tasks.
</p>

---

### Features

- Create todos
- View all todos
- Get todo by ID
- Update existing todos
- Delete todos
- Persistent local storage
- Simple HTML/CSS/JavaScript frontend
- Asynchronous server powered by Tokio

---

### Tech Stack

| Technology | Purpose |
|------------|---------|
| Rust 2024 | Backend |
| Axum | Web framework |
| Tokio | Async runtime |
| Serde | JSON serialization |
| UUID | Unique identifiers |
| HTML/CSS/JavaScript | Frontend |

---

### Getting Started

### Clone the repository

```bash
git clone https://github.com/MajkiiWasTaken/RustTodo.git
cd RustTodo
```

### Run the application

```bash
cargo run
```

The server starts on:

```
http://127.0.0.1:3000
```

Open it in your browser to use the web interface.

---

### API Endpoints

| Method | Endpoint | Description |
|---------|----------|-------------|
| GET | `/todos` | Get all todos |
| GET | `/todos/{id}` | Get a todo by ID |
| POST | `/todos` | Create a new todo |
| PUT | `/todos/{id}` | Update a todo |
| DELETE | `/todos/{id}` | Delete a todo |

---

### Preview

<img width="1121" height="1027" alt="image" src="https://github.com/user-attachments/assets/73a7c708-ce71-4a7e-b81c-9589807903d7" />


---

### Dependencies

- Axum
- Tokio
- Serde
- Serde JSON
- UUID
- Tower HTTP

---

### Future Improvements

- User authentication
- SQLite/PostgreSQL support
- Docker deployment
- Filtering and search
- Due dates
- Categories
- Responsive UI
- Unit & integration tests

---

### Author: **Michal Švrček**

This project is licensed under the MIT License.
