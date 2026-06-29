let currentFilter = "all";
let editingTodoId = null;
let cachedTodos = [];

function setFilter(filter) {
    currentFilter = filter;
    loadTodos();
}

function showToast(message) {
    const toast = document.getElementById("toast");

    toast.textContent = message;
    toast.classList.add("show");

    setTimeout(() => {
        toast.classList.remove("show");
    }, 2500);
}

function loadCachedTodos() {
    const cached = localStorage.getItem("todos");

    if (!cached) {
        return;
    }

    const todos = JSON.parse(cached);
    cachedTodos = todos;

    renderTodos(todos);
}

function renderTodos(todos) {
    const todoList = document.getElementById("todoList");
    const count = document.getElementById("count");

    const completedCount = todos.filter(todo => todo.completed).length;
    const remainingCount = todos.length - completedCount;
    const percent = todos.length === 0
        ? 0
        : Math.round((completedCount / todos.length) * 100);

    count.textContent = todos.length;
    document.getElementById("totalStats").textContent = `${todos.length} Tasks`;
    document.getElementById("completedStats").textContent = `${completedCount} Completed`;
    document.getElementById("remainingStats").textContent = `${remainingCount} Remaining`;
    document.getElementById("progressBar").style.width = `${percent}%`;

    todoList.innerHTML = "";

    if (todos.length === 0) {
        todoList.innerHTML = `
            <div class="empty-state">
                <h3>No todos yet</h3>
                <p>Add your first Rust-powered task above.</p>
            </div>
        `;
        return;
    }

    const search = document.getElementById("searchInput").value.toLowerCase();

    const filteredTodos = todos.filter(todo => {
        const matchesSearch =
            todo.title.toLowerCase().includes(search) ||
            todo.description.toLowerCase().includes(search);

        if (!matchesSearch) {
            return false;
        }

        if (currentFilter === "active" && todo.completed) {
            return false;
        }

        if (currentFilter === "completed" && !todo.completed) {
            return false;
        }

        return true;
    });

    if (filteredTodos.length === 0) {
        todoList.innerHTML = `
            <div class="empty-state">
                <h3>No matching todos</h3>
                <p>Try changing your search or filter.</p>
            </div>
        `;
        return;
    }

    for (const todo of filteredTodos) {
        const item = document.createElement("div");
        item.className = `todo-item ${todo.completed ? "completed" : ""} priority-${todo.priority || "medium"}`;

        item.innerHTML = `
            <label class="todo-checkbox">
                <input
                    type="checkbox"
                    ${todo.completed ? "checked" : ""}
                    onchange="toggleTodo(${todo.id}, this.checked)">
                <span></span>
            </label>

            <div class="todo-content">
                <div class="todo-title">${todo.title}</div>
                <div class="todo-description">${todo.description}</div>

                <div class="todo-meta">
                    <span class="priority-tag priority-${todo.priority || "medium"}">
                        ${(todo.priority || "medium").toUpperCase()}
                    </span>

                    ${
                        todo.category && todo.category.trim() !== ""
                            ? `<span class="category-tag">${todo.category}</span>`
                            : ""
                    }
                </div>
            </div>

            <div class="todo-right">
                <div class="badge ${todo.completed ? "done" : "not-done"}">
                    ${todo.completed ? "Completed" : "Not completed"}
                </div>

                <button class="edit-btn" onclick="openEdit(${todo.id})">
                    Edit
                </button>

                <button class="delete-btn" onclick="deleteTodo(${todo.id})">
                    Delete
                </button>
            </div>
        `;

        todoList.appendChild(item);
    }
}

async function loadTodos() {
    const todoList = document.getElementById("todoList");

    try {
        const response = await fetch("/todos");

        if (!response.ok) {
            throw new Error("Could not load todos");
        }

        const todos = await response.json();

        cachedTodos = todos;
        localStorage.setItem("todos", JSON.stringify(todos));

        renderTodos(todos);
    } catch (error) {
        showToast("Server unavailable, showing cached data");

        if (cachedTodos.length > 0) {
            renderTodos(cachedTodos);
            return;
        }

        todoList.innerHTML = `
            <div class="empty-state">
                <h3>Could not load todos</h3>
                <p>Check if your Rust server is running.</p>
            </div>
        `;
    }
}

async function createTodo() {
    const titleInput = document.getElementById("title");
    const descriptionInput = document.getElementById("description");
    const priorityInput = document.getElementById("priority");
    const categoryInput = document.getElementById("category");

    const title = titleInput.value.trim();
    const description = descriptionInput.value.trim();
    const priority = priorityInput.value;
    const category = categoryInput.value.trim();

    if (title.length === 0) {
        showToast("Title is required");
        return;
    }

    try {
        const response = await fetch("/todos", {
            method: "POST",
            headers: {
                "Content-Type": "application/json"
            },
            body: JSON.stringify({
                title,
                description,
                priority,
                category
            })
        });

        if (!response.ok) {
            showToast("Could not create todo");
            return;
        }

        titleInput.value = "";
        descriptionInput.value = "";
        priorityInput.value = "medium";
        categoryInput.value = "";

        showToast("Todo created");
        await loadTodos();
    } catch (error) {
        showToast("Server unavailable");
    }
}

async function toggleTodo(id, completed) {
    try {
        const response = await fetch(`/todos/${id}`, {
            method: "PUT",
            headers: {
                "Content-Type": "application/json"
            },
            body: JSON.stringify({
                completed
            })
        });

        if (!response.ok) {
            showToast("Could not update todo");
            return;
        }

        showToast(completed ? "Todo completed" : "Todo reopened");
        await loadTodos();
    } catch (error) {
        showToast("Server unavailable");
    }
}

async function deleteTodo(id) {
    const confirmed = confirm("You really want to delete this todo?");

    if (!confirmed) {
        return;
    }

    try {
        const response = await fetch(`/todos/${id}`, {
            method: "DELETE"
        });

        if (!response.ok) {
            showToast("Could not delete todo");
            return;
        }

        showToast("Todo deleted");
        await loadTodos();
    } catch (error) {
        showToast("Server unavailable");
    }
}

function openEdit(id) {
    const todo = cachedTodos.find(todo => todo.id === id);

    if (!todo) {
        showToast("Todo not found");
        return;
    }

    editingTodoId = id;

    document.getElementById("editTitle").value = todo.title;
    document.getElementById("editDescription").value = todo.description;
    document.getElementById("editPriority").value = todo.priority || "medium";
    document.getElementById("editCategory").value = todo.category || "";

    document.getElementById("editModal").classList.remove("hidden");
}

function closeEdit() {
    editingTodoId = null;
    document.getElementById("editModal").classList.add("hidden");
}

async function saveEdit() {
    const title = document.getElementById("editTitle").value.trim();
    const description = document.getElementById("editDescription").value.trim();
    const priority = document.getElementById("editPriority").value;
    const category = document.getElementById("editCategory").value.trim();

    if (title.length === 0) {
        showToast("Title is required");
        return;
    }

    try {
        const response = await fetch(`/todos/${editingTodoId}`, {
            method: "PUT",
            headers: {
                "Content-Type": "application/json"
            },
            body: JSON.stringify({
                title,
                description,
                priority,
                category
            })
        });

        if (!response.ok) {
            showToast("Could not update todo");
            return;
        }

        closeEdit();
        showToast("Todo updated");
        await loadTodos();
    } catch (error) {
        showToast("Server unavailable");
    }
}

loadCachedTodos();
loadTodos();