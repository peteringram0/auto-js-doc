import { readableStreamToText } from "bun"

const allowedDomain = ["localhost:3000", "auto-js-doc.fly.dev"]
  
Bun.serve({
  port: 3000,
  async fetch(request) {
    const url = new URL(request.url)

    // Only allow my instance to call the endpoint
    const reqOrigin = request.headers.get('host')
    if (!reqOrigin || !allowedDomain.includes(reqOrigin)) return new Response("Error", {status: 401})
    
    switch (url.pathname) {
      case '/run-cli': {
        const subprocess = Bun.spawn({ cmd: [`./auto-js-doc`], stdin: "pipe", stdout: "pipe", stderr: 'pipe' })
        subprocess.stdin.write(await request.text())
        subprocess.stdin.end()
        const stdout = await readableStreamToText(subprocess.stdout);
        return new Response(stdout, {
          headers: {
            "Content-Type": "text/plain"
          },
        });
      }
      case '/':
        return new Response(Bun.file("./index.html"), {
          headers: {
            "Content-Type": "text/html"
          },
        });
      default:
        return new Response("Not Found", { status: 404 })
    }
  }
})
