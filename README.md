# next-pagefind

Use [Pagefind](https://pagefind.app/) with next.js non static exports.

## Insallation

Install the modules required for the program.

1. `cargo install spider_cli`
1. `cargo install pagefind`
1. `cargo install next-pagefind`

## Getting Started

1. Run `next-pagefind` at the root directory of your next application to create your search index's and output the content into the `public` folder.

2. Optional: Add `data-pagefind-meta="url[href]"` on your meta `hrefLang` links example: `<link rel="alternate" hrefLang="en" href="https://a11ywatch.com/blog/version-your-proto-definitions-for-stablity" data-pagefind-meta="url[href]"/>` to replace the location of the links.

3. Setup pagefind in the your next.js project. The example before uses the layout to setup the PageFindUI wasm module once.

```tsx
import { useEffect } from "react";
import { useRouter } from 'next/router'

export function Layout({ Component, pageProps }) {
  const router = useRouter()
  const lastPath = useRef("");

  const pathName = router.pathname

  useEffect(() => {
    // init the script on each new route change you can replace the router usage by creating a component wrapping the useEffect and <div id="search" /> to init the ui
    if (pathName !== lastPath.current) {
      lastPath.current = pathName;
      const PagefindUI =
        // @ts-ignore
        typeof window.PagefindUI !== "undefined" && window.PagefindUI;

      let observer: MutationObserver;

      if (PagefindUI) {
        new PagefindUI({
          element: "#search",
          resetStyles: false,
          showImages: false,
          showEmptyFilters: false,
        });

        // Optional: if you skipped step 2 add an observer that can replace the relative links with the actual path
        const pagefindDrawer = document.querySelector(".pagefind-ui__drawer");

        // replace the .html from links with path
        if (pagefindDrawer) {
          const callback = () => {
            const links: NodeListOf<HTMLAnchorElement> =
              document.querySelectorAll(".pagefind-ui__result-link");

            for (const link of links) {
              link.href = link.href.replace(".html", "");
            }
          };

          observer = new MutationObserver(callback);

          observer.observe(pagefindDrawer, {
            attributes: false,
            childList: true,
            subtree: true,
          });
        }
      }

      return () => {
        // remove the observer
        if (observer) {
          observer.disconnect();
        }
      };
    }

  }, [lastPath, pathName]);

  return (
    <>
      <Head>
        <link href="/_pagefind/pagefind-ui.css" rel="stylesheet" key="pagefind-ui-css" />
        <script src="/_pagefind/pagefind-ui.js" type="text/javascript" key="pagefind-ui-js" />
      </Head>
      <main className="blog-main" id="main-content" data-pagefind-body>
        <div id="search"></div>
        <Component {...pageProps} />
      </main>
    </>
  );
}
```

## Dark Mode

If you need to handle themes include the following css before `_/pagefind/pagefind-ui.css`.

Inside your public folder add the following file `css/_pagefind.css` with the markup.

```css
:root {
    --pagefind-ui-scale: 1;
    --pagefind-ui-primary: #034ad8;
    --pagefind-ui-text: #393939;
    --pagefind-ui-background: #ffffff;
    --pagefind-ui-border: #eeeeee;
    --pagefind-ui-tag: #eeeeee;
    --pagefind-ui-border-width: 2px;
    --pagefind-ui-border-radius: 8px;
    --pagefind-ui-image-border-radius: 8px;
    --pagefind-ui-image-box-ratio: 3 / 2;
    --pagefind-ui-font: sans-serif;
    --pagefind-ui-placeholder: #000;
}

:root.dark {
    --pagefind-ui-primary: #eeeeee;
    --pagefind-ui-text: #eeeeee;
    --pagefind-ui-background: #152028;
    --pagefind-ui-border: #152028;
    --pagefind-ui-tag: #152028;
    --pagefind-ui-placeholder: #fff;
}

.pagefind-ui__search-input::placeholder {
    color: var(--pagefind-ui-placeholder);
    opacity: 1;
}
```

Next include the css before pagefind `<link href="/css/_pagefind.css" rel="stylesheet" />`.

![Example of pagefind being used with the A11yWatch blog](example.png)

## Todo

1. Allow passing in custom port output.
2. Allow custom locale building folders for pagefind. In the meantime you can manually stitch the folders with `spider` and simply use `npx -y pagefind --source _temp_spider_downloads --bundle-dir public/_pagefind && cp -R _temp_spider_downloads/public/_pagefind public/` to perform the conversion.
