import hljs from "highlight.js";
import json from "highlight.js/lib/languages/json";

hljs.registerLanguage("json", json);

function isHTMLElement(node: Node): node is HTMLElement {
  return node.nodeType === Node.ELEMENT_NODE;
}

function isHTMLElementMutationRecord(
  mutation: MutationRecord
): mutation is MutationRecord & { target: HTMLElement } {
  return isHTMLElement(mutation.target);
}

function createObserver(): MutationObserver {
  return new MutationObserver((mutations) => {
    mutations.filter(isHTMLElementMutationRecord).forEach((mutation) => {
      switch (mutation.type) {
        case "attributes":
          switch (mutation.attributeName) {
            case "data-json":
              const el = mutation.target;

              const jsonString = el.getAttribute("data-json");

              if (jsonString) {
                const jsRepr = JSON.parse(jsonString);
                const pretty = JSON.stringify(jsRepr, null, 4);

                el.innerHTML = pretty;
                el.dataset.highlighted = "";

                hljs.highlightElement(el);
              }

              break;
          }

          break;
      }
    });
  });
}

function createHook(observer: MutationObserver) {
  return {
    mounted() {
      observer.observe(this.el, {
        attributes: true,
        attributeFilter: ["data-json"],
        attributeOldValue: false,
        subtree: false,
        characterData: true,
      });
    },
  };
}

export function createObserverAndHook() {
  const observer = createObserver();
  const hook = createHook(observer);

  return [observer, hook];
}
