const CATALOG = __CATALOG_JSON__;

function t(value) {
  if (typeof value !== "string") return value;
  if (Object.prototype.hasOwnProperty.call(CATALOG, value)) return CATALOG[value];
  return value;
}

function translate(node) {
  if (!node) return;
  if (typeof node === "string") return;
  if (Array.isArray(node)) {
    for (let i = 0; i < node.length; i += 1) {
      if (typeof node[i] === "string") node[i] = t(node[i]);
      else translate(node[i]);
    }
    return;
  }
  if (typeof node !== "object") return;
  const keys = ["children", "content", "title", "placeholder", "label", "text"];
  for (const key of keys) {
    if (!(key in node)) continue;
    const current = node[key];
    if (typeof current === "string") node[key] = t(current);
    else translate(current);
  }
  if (node.props) translate(node.props);
}

function proxy(fn) {
  return new Proxy(fn, {
    apply(target, thisArg, args) {
      if (args.length > 1) translate(args[1]);
      for (let i = 2; i < args.length; i += 1) {
        if (typeof args[i] === "string") args[i] = t(args[i]);
        else translate(args[i]);
      }
      return target.apply(thisArg, args);
    },
  });
}

function patchCode(code) {
  return code
    .replace(
      /([A-Za-z_$][\w$]*)\.(jsx|jsxs)=([A-Za-z_$][\w$]*)/g,
      (_, obj, prop, fn) =>
        `${obj}.${prop}=${fn};${obj}.${prop}=globalThis.__vcc_i18n_proxy__(${obj}.${prop})`
    )
    .replace(
      /([A-Za-z_$][\w$]*)\.createElement=([A-Za-z_$][\w$]*)/g,
      (_, obj, fn) =>
        `${obj}.createElement=${fn};${obj}.createElement=globalThis.__vcc_i18n_proxy__(${obj}.createElement)`
    );
}

async function main() {
  const index = document.querySelector('meta[name="vcc-i18n-index"]')?.content;
  if (!index) return;
  globalThis.__vcc_i18n_proxy__ = proxy;
  const url = new URL(index, location.origin);
  const code = await fetch(url.href).then((res) => res.text());
  const script = document.createElement("script");
  script.type = "module";
  script.textContent = patchCode(code);
  document.body.appendChild(script);
}

main();
