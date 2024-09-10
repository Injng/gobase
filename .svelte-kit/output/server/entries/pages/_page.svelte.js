import { c as create_ssr_component, d as compute_rest_props, f as spread, h as escape_object, i as escape_attribute_value, j as each, v as validate_component, k as add_attribute, e as escape } from "../../chunks/ssr.js";
import "@tauri-apps/api/tauri";
import "@tauri-apps/api/dialog";
const void_element_names = /^(?:area|base|br|col|command|embed|hr|img|input|keygen|link|meta|param|source|track|wbr)$/;
function is_void(name) {
  return void_element_names.test(name) || name.toLowerCase() === "!doctype";
}
/**
 * @license lucide-svelte v0.439.0 - ISC
 *
 * This source code is licensed under the ISC license.
 * See the LICENSE file in the root directory of this source tree.
 */
const defaultAttributes = {
  xmlns: "http://www.w3.org/2000/svg",
  width: 24,
  height: 24,
  viewBox: "0 0 24 24",
  fill: "none",
  stroke: "currentColor",
  "stroke-width": 2,
  "stroke-linecap": "round",
  "stroke-linejoin": "round"
};
const Icon = create_ssr_component(($$result, $$props, $$bindings, slots) => {
  let $$restProps = compute_rest_props($$props, ["name", "color", "size", "strokeWidth", "absoluteStrokeWidth", "iconNode"]);
  let { name = void 0 } = $$props;
  let { color = "currentColor" } = $$props;
  let { size = 24 } = $$props;
  let { strokeWidth = 2 } = $$props;
  let { absoluteStrokeWidth = false } = $$props;
  let { iconNode = [] } = $$props;
  const mergeClasses = (...classes) => classes.filter((className, index, array) => {
    return Boolean(className) && array.indexOf(className) === index;
  }).join(" ");
  if ($$props.name === void 0 && $$bindings.name && name !== void 0)
    $$bindings.name(name);
  if ($$props.color === void 0 && $$bindings.color && color !== void 0)
    $$bindings.color(color);
  if ($$props.size === void 0 && $$bindings.size && size !== void 0)
    $$bindings.size(size);
  if ($$props.strokeWidth === void 0 && $$bindings.strokeWidth && strokeWidth !== void 0)
    $$bindings.strokeWidth(strokeWidth);
  if ($$props.absoluteStrokeWidth === void 0 && $$bindings.absoluteStrokeWidth && absoluteStrokeWidth !== void 0)
    $$bindings.absoluteStrokeWidth(absoluteStrokeWidth);
  if ($$props.iconNode === void 0 && $$bindings.iconNode && iconNode !== void 0)
    $$bindings.iconNode(iconNode);
  return `<svg${spread(
    [
      escape_object(defaultAttributes),
      escape_object($$restProps),
      { width: escape_attribute_value(size) },
      { height: escape_attribute_value(size) },
      { stroke: escape_attribute_value(color) },
      {
        "stroke-width": escape_attribute_value(absoluteStrokeWidth ? Number(strokeWidth) * 24 / Number(size) : strokeWidth)
      },
      {
        class: escape_attribute_value(mergeClasses("lucide-icon", "lucide", name ? `lucide-${name}` : "", $$props.class))
      }
    ],
    {}
  )}>${each(iconNode, ([tag, attrs]) => {
    return `${((tag$1) => {
      return tag$1 ? `<${tag}${spread([escape_object(attrs)], {})}>${is_void(tag$1) ? "" : ``}${is_void(tag$1) ? "" : `</${tag$1}>`}` : "";
    })(tag)}`;
  })}${slots.default ? slots.default({}) : ``}</svg>`;
});
const File_text = create_ssr_component(($$result, $$props, $$bindings, slots) => {
  const iconNode = [
    [
      "path",
      {
        "d": "M15 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7Z"
      }
    ],
    ["path", { "d": "M14 2v4a2 2 0 0 0 2 2h4" }],
    ["path", { "d": "M10 9H8" }],
    ["path", { "d": "M16 13H8" }],
    ["path", { "d": "M16 17H8" }]
  ];
  return `${validate_component(Icon, "Icon").$$render($$result, Object.assign({}, { name: "file-text" }, $$props, { iconNode }), {}, {
    default: () => {
      return `${slots.default ? slots.default({}) : ``}`;
    }
  })}`;
});
const Rewind = create_ssr_component(($$result, $$props, $$bindings, slots) => {
  const iconNode = [
    ["polygon", { "points": "11 19 2 12 11 5 11 19" }],
    ["polygon", { "points": "22 19 13 12 22 5 22 19" }]
  ];
  return `${validate_component(Icon, "Icon").$$render($$result, Object.assign({}, { name: "rewind" }, $$props, { iconNode }), {}, {
    default: () => {
      return `${slots.default ? slots.default({}) : ``}`;
    }
  })}`;
});
const Save = create_ssr_component(($$result, $$props, $$bindings, slots) => {
  const iconNode = [
    [
      "path",
      {
        "d": "M15.2 3a2 2 0 0 1 1.4.6l3.8 3.8a2 2 0 0 1 .6 1.4V19a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2z"
      }
    ],
    [
      "path",
      {
        "d": "M17 21v-7a1 1 0 0 0-1-1H8a1 1 0 0 0-1 1v7"
      }
    ],
    ["path", { "d": "M7 3v4a1 1 0 0 0 1 1h7" }]
  ];
  return `${validate_component(Icon, "Icon").$$render($$result, Object.assign({}, { name: "save" }, $$props, { iconNode }), {}, {
    default: () => {
      return `${slots.default ? slots.default({}) : ``}`;
    }
  })}`;
});
const Page = create_ssr_component(($$result, $$props, $$bindings, slots) => {
  let savedStates = [];
  return `<div class="grid grid-rows-[100px_200px_auto]"><div class="text-center p-6 bg-slate-500 text-white" data-svelte-h="svelte-1aw2kl3"><div class="text-3xl">Gobase</div></div> <div class="grid grid-cols-[10%_auto_10%] bg-slate-500 text-white"><div class="grid grid-rows-[200px_auto]"><div class="p-4"><svg width="50px" height="50px" role="button" fill="none" tabindex="-1"><path d="M13 3H8.2C7.0799 3 6.51984 3 6.09202 3.21799C5.71569 3.40973 5.40973 3.71569 5.21799 4.09202C5 4.51984 5 5.0799 5 6.2V17.8C5 18.9201 5 19.4802 5.21799 19.908C5.40973 20.2843 5.71569 20.5903 6.09202 20.782C6.51984 21 7.0799 21 8.2 21H12M13 3L19 9M13 3V7.4C13 7.96005 13 8.24008 13.109 8.45399C13.2049 8.64215 13.3578 8.79513 13.546 8.89101C13.7599 9 14.0399 9 14.6 9H19M19 9V12M17 19H21M19 17V21" stroke="#FFFFFF" stroke-linecap="round" stroke-linejoin="round"></path></svg></div> <div class="p-4"><button class="bg-blue-500 hover:bg-blue-600 text-white p-2 rounded flex items-center justify-center">${validate_component(Save, "Save").$$render($$result, { class: "mr-2", size: 16 }, {}, {})}
                    Save State</button> <div class="bg-gray-700 p-2 mt-2 rounded"><div class="flex items-center justify-between mb-2"><span data-svelte-h="svelte-sn0glt">Saved States</span> ${validate_component(File_text, "FileText").$$render($$result, { size: 16 }, {}, {})}</div> ${each(savedStates, (state) => {
    return `<button class="w-full bg-gray-600 hover:bg-gray-500 text-white p-2 rounded mb-1 flex items-center justify-start">${validate_component(Rewind, "Rewind").$$render($$result, { class: "mr-2", size: 16 }, {}, {})} ${escape(state.name)} </button>`;
  })}</div></div></div> <div class="relative"><canvas class="absolute left-1/2 transform -translate-x-1/2"${add_attribute()}${add_attribute()}${add_attribute()}></canvas> <canvas${add_attribute()}${add_attribute()} class="absolute left-1/2 transform -translate-x-1/2"${add_attribute()}></canvas> <canvas${add_attribute()}${add_attribute()} class="absolute left-1/2 transform -translate-x-1/2"${add_attribute()}></canvas></div> <div class="grid grid-rows-[200px_auto]"><div><svg width="50px" height="40px" role="button" tabindex="-1"><circle style="fill:#000000;fill-opacity:0;stroke:#fffbfb;stroke-width:1.565;stroke-dasharray:none;stroke-opacity:1" id="path1" cx="20" cy="20" r="10"></circle></svg> <svg width="50px" height="40px" role="button" tabindex="-1"><circle style="fill:#fffbfb;fill-opacity:1;stroke:#fffbfb;stroke-width:1.565;stroke-dasharray:none;stroke-opacity:1" id="path1" cx="20" cy="20" r="10"></circle></svg> <svg width="50px" height="40px" role="button" tabindex="-1"><circle style="fill:#000000;fill-opacity:0;stroke:#fffbfb;stroke-width:1.565;stroke-dasharray:none;stroke-opacity:1" id="path1" cx="20" cy="20" r="10"></circle><rect style="fill:#000000;fill-opacity:0;stroke:#fffbfb;stroke-width:1.41226" id="rect1" width="0.012675161" height="6.5395594" x="33.037266" y="7.2474785"></rect><rect style="fill:#000000;fill-opacity:0;stroke:#fffbfb;stroke-width:1.41226" id="rect1-5" width="0.012675161" height="6.5395594" x="10.456588" y="-36.335133" transform="rotate(90)"></rect></svg> <svg width="50px" height="40px" role="button" tabindex="-1"><circle style="fill:#fffbfb;fill-opacity:1;stroke:#fffbfb;stroke-width:1.565;stroke-dasharray:none;stroke-opacity:1" id="path1" cx="20" cy="20" r="10"></circle><rect style="fill:#000000;fill-opacity:0;stroke:#fffbfb;stroke-width:1.41226" id="rect1" width="0.012675161" height="6.5395594" x="33.037266" y="7.2474785"></rect><rect style="fill:#000000;fill-opacity:0;stroke:#fffbfb;stroke-width:1.41226" id="rect1-5" width="0.012675161" height="6.5395594" x="10.456588" y="-36.335133" transform="rotate(90)"></rect></svg></div> <div class="p-4"><button class="bg-blue-500 hover:bg-blue-600 text-white p-2 rounded flex items-center justify-center">${validate_component(Save, "Save").$$render($$result, { class: "mr-2", size: 16 }, {}, {})}
                    Save Game</button> <button class="mt-2 bg-blue-500 hover:bg-blue-600 text-white p-2 rounded flex items-center justify-center">${validate_component(File_text, "FileText").$$render($$result, { class: "mr-2", size: 16 }, {}, {})}
                    Load Game</button> <button class="mt-2 bg-gray-600 hover:bg-gray-700 text-white p-2 rounded flex items-center justify-center">${validate_component(Save, "Save").$$render($$result, { class: "mr-2", size: 16 }, {}, {})}
                    Save SGF</button></div></div></div></div> `;
});
export {
  Page as default
};
