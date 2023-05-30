import { writeFile } from "node:fs/promises";
import { join } from "node:path";

const licenses = await (await fetch("https://api.github.com/licenses")).json();

for (const license of licenses) {
  const data = await (await fetch(license.url)).json();

  await writeFile(
    `${data.key}.json`,
    JSON.stringify({
      permissions: data.permissions,
      conditions: data.conditions,
      limitations: data.limitations,
      body: data.body,
    })
  );
}
