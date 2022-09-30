window.document.body.classList.add("dark");

for (let e of window.document.querySelectorAll(".bg-gray-light")) {
    e.classList.add("dark:bg-blue-1000");
}

for (let e of window.document.querySelectorAll(".bg-white")) {
    e.classList.add("dark:bg-blue-900");
}

for (let e of window.document.querySelectorAll(".font-display")) {
    e.classList.add("dark:text-white");
}
for (let e of window.document.querySelectorAll("legend")) {
    e.classList.add("dark:text-white");
}

for (let e of window.document.querySelectorAll("input")) {
    e.classList.add("dark:bg-blue-900", "dark:text-white");
}

for (let e of window.document.querySelectorAll("button .bg-orange-800")) {
    e.classList.add("dark:ext-bg-orange-dark");
    e.classList.add("dark:ext-border-orange-dark");
    e.classList.add("dark:group-hover:ext-bg-orange-800");
    e.classList.add("dark:group-hover:ext-border-orange-800");
}
