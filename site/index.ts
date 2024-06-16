import { readableStreamToText } from "bun"

Bun.serve({
  port: 3000,
  async fetch(request) {
    const url = new URL(request.url)

    switch (url.pathname) {
      case '/run-cli': {

        // console.log(await request.text())
        const subprocess = Bun.spawn({
          cmd: [`./auto-js-doc`],
          stdin: "pipe",
          stdout: "pipe",
          stderr: 'pipe',
        });
        // subprocess.stdin.write(`
        //     function testNoExport(param1: string, param2?: boolean) {

        //     }
        //   `)
        subprocess.stdin.write(await request.text())
        subprocess.stdin.end()
        const text = await readableStreamToText(subprocess.stdout);
        return new Response(text, {
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
