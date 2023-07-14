import { useEffect, useRef } from "react";

// A generic pagefind Component
export const PageFind = ({ id }: {id?: string}) => {
  const loaded = useRef<boolean>(false);

  // the search input to replace with the pagefind elements
  const targetID = id ?? "search"

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
            element: `#${targetID}`,
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
  }, [loaded, targetID]);

  return <div id={targetID}></div>;
};