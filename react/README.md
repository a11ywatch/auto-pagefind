# pagefind-react

A pagefind react Component.

## Getting Started

`npm i pagefind-react --save`

Setup the pagefind bundles in your layout or component.

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
import { PageFind } from "pagefind-react";

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