import { useClient } from "$lib/contexts/client";
import { onMount, unmount } from "svelte";
import type { Land, PonziLandSchemaType } from "$lib/models.gen";
import { useDojo } from "$lib/contexts/dojo";
import {
  createDojoStore,
  QueryBuilder,
  type SubscribeParams,
} from "@dojoengine/sdk";
import { derived, get, writable, type Readable } from "svelte/store";

export function useLands(): Readable<Land[]> | undefined {
  // Get all lands in the store
  if (typeof window === "undefined") {
    // We are on the server. Return nothing.
    return undefined;
  }

  const { store, client: sdk, account } = useDojo();

  const landStore = derived([store], ([actualStore]) => actualStore);

  (async () => {
    await sdk.subscribeEntityQuery({
      query: new QueryBuilder<PonziLandSchemaType>()
        .namespace("ponzi_land", (ns) => {
          ns.entity("Land", (e) => e.build());
        })
        .build(),
      callback: (response) => {
        if (response.error || response.data == null) {
          console.log("Got an error!", response.error);
        } else {
          console.log("Setting entities :)");
          console.log("Data!");
          get(landStore).setEntities(response.data);
        }
      },
      options: {},
    } as SubscribeParams<PonziLandSchemaType>);
  })();

  const landEntityStore = derived([landStore], ([s]) => {
    return s
      .getEntitiesByModel("ponzi_land", "Land")
      .map((e) => e.models["ponzi_land"]["Land"] as Land);
  });

  return landEntityStore;
}
