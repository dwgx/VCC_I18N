const CATALOG = __CATALOG_JSON__;

const DAYJS_EN_RELATIVE_TIME =
  '{future:"in %s",past:"%s ago",s:"a few seconds",m:"a minute",mm:"%d minutes",h:"an hour",hh:"%d hours",d:"a day",dd:"%d days",M:"a month",MM:"%d months",y:"a year",yy:"%d years"}';
const DAYJS_ZH_RELATIVE_TIME =
  '{future:"%s后",past:"%s前",s:"几秒",m:"1 分钟",mm:"%d 分钟",h:"1 小时",hh:"%d 小时",d:"1 天",dd:"%d 天",M:"1 个月",MM:"%d 个月",y:"1 年",yy:"%d 年"}';

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
  const keys = [
    "children",
    "content",
    "title",
    "placeholder",
    "label",
    "text",
    "header",
    "description",
    "alt",
    "aria-label",
  ];
  for (const key of keys) {
    if (!(key in node)) continue;
    const current = node[key];
    if (typeof current === "string") node[key] = t(current);
    else translate(current);
  }
  if (node.props) translate(node.props);
}

function proxy(fn, translateChildren) {
  return new Proxy(fn, {
    apply(target, thisArg, args) {
      if (args.length > 1) translate(args[1]);
      if (translateChildren) {
        for (let i = 2; i < args.length; i += 1) {
          if (typeof args[i] === "string") args[i] = t(args[i]);
          else translate(args[i]);
        }
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
        `${obj}.${prop}=${fn};${obj}.${prop}=globalThis.__vcc_i18n_proxy__(${obj}.${prop},false)`
    )
    .replace(
      /([A-Za-z_$][\w$]*)\.createElement=([A-Za-z_$][\w$]*)/g,
      (_, obj, fn) =>
        `${obj}.createElement=${fn};${obj}.createElement=globalThis.__vcc_i18n_proxy__(${obj}.createElement,true)`
    )
    .split(DAYJS_EN_RELATIVE_TIME)
    .join(DAYJS_ZH_RELATIVE_TIME);
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
