# auto-pagefind

Use [Pagefind](https://pagefind.app/) with any live server.

## Insallation

Install the modules required for the program.

1. `cargo install spider_cli`
1. `cargo install pagefind`
1. `cargo install auto-pagefind`

## Getting Started

1. Start the dev or prod instance on port 3000 and run `auto-pagefind` at the root directory of your next application to create your search index's and output the content into the `public` folder.

2. Optional: Add `data-pagefind-meta="url[href]"` on your meta `hrefLang` links example: `<link rel="alternate" hrefLang="en" href="https://a11ywatch.com/blog/version-your-proto-definitions-for-stablity" data-pagefind-meta="url[href]"/>` to replace the location of the links.

3. Setup pagefind client loading in the your project. The example before uses the layout to setup the PageFindUI wasm module once for next.js.

```tsx
// pagefind.tsx
import { useEffect, useRef } from "react";

export const PageFind = () => {
  const loaded = useRef<boolean>(false);

  useEffect(() => {
    if (!loaded.current) {
      loaded.current = true;
      let observer: MutationObserver;

      const PagefindUI =
        // @ts-ignore
        typeof window.PagefindUI !== "undefined" && window.PagefindUI;

      if (PagefindUI) {
        try {
          new PagefindUI({
            element: "#search",
            resetStyles: false,
            showImages: false,
            showEmptyFilters: false,
          });

          // delete the observer code below if you did step 2 and target production websites
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
        } catch (e) {
          console.error(e);
        }
      }

      return () => {
        if (observer) {
          observer.disconnect();
        }
      };
    }
  }, [loaded]);

  return <div id="search"></div>;
};
```

```tsx
// layout.tsx
import React from "react";

// use layout for scripts _pagefind
export function Layout({ children }) {
  return (
    <>
      <Head>
        <link
          href="/_pagefind/pagefind-ui.css"
          rel="stylesheet"
          key="pagefind-ui-css"
        />
        <script
          src="/_pagefind/pagefind-ui.js"
          type="text/javascript"
          key="pagefind-ui-js"
        />
      </Head>
      <main id="main-content" data-pagefind-body>
        {children}
      </main>
    </>
  );
}
```

```tsx
// pages/index.tsx
import React from "react";
import { Layout } from "./layout";
import { PageFind } from "./pagefind";

export function Home({ Component, pageProps }) {
  return (
    <>
      <h1>Home Page</h1>
      <PageFind />
    </>
  );
}

Home.getLayout = Layout;
```

```tsx
// _app.js

export default function MyApp({ Component, pageProps }) {
  // Use the layout defined at the page level, if available
  const getLayout = Component.getLayout || ((page) => page);

  return getLayout(<Component {...pageProps} />);
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

## Options

Some optional options below.

```
auto-pagefind --help
Pagefind for next.js non output export applications. Fully crawl and index your app in one command.

Usage: auto-pagefind [OPTIONS]

Options:
  -d, --download-dir <DOWNLOAD_DIR>  The download directory for storing the static.html files
  -u, --url <URL>                    The website url
  -h, --help                         Print help
  -V, --version                      Print version
```

## Todo

1. Allow passing in custom port output.
2. Allow custom locale building folders for pagefind. In the meantime you can manually stitch the folders with `spider` and simply use `npx -y pagefind --source _temp_spider_downloads --bundle-dir public/_pagefind && cp -R _temp_spider_downloads/public/_pagefind public/` to perform the conversion.
