import { $ } from "bun";

Bun.serve({
  port: 3000,
  async fetch(request) {
    const url = new URL(request.url);

    if (url.pathname === "/run-cli") {
      const result = (await $`./auto-js-doc`).stdout;
      return new Response(result, {
        headers: {
          "Content-Type": "text/plain",
        },
      });
    }

    // Serve static files (like your HTML page)
    if (url.pathname === "/") {
      return new Response(Bun.file("./index.html"), {
        headers: {
          "Content-Type": "text/html",
        },
      });
    }

    return new Response("Not Found", { status: 404 });
  },
});
