import { emit, listen } from '@tauri-apps/api/event';
import { load, type Store } from '@tauri-apps/plugin-store';
import { create } from 'zustand';

const setupMap = new Map<string, boolean>();
const pendingInits = new Map<string, Promise<void>>();

export function createSync<V extends object>(key: string, initialValues: V) {
  type StoreType = {
    data: V;
    initialized: boolean;
    sync: ((patch: Partial<V>) => Promise<void>) &
      ((key: keyof V, value: V[keyof V]) => Promise<void>);
    syncAll: (next: V, persistLocal?: boolean) => Promise<void>;
    reset: () => Promise<void>;
  };

  const shouldPersist = !!key?.trim();
  const store = create<StoreType>((set, get) => {
    async function apply(partial: Partial<StoreType>, persistLocal: boolean = true): Promise<void> {
      set(partial);
      await emit(`sync:${key}`, partial);
      await saveLocal(key, shouldPersist && persistLocal, { data: get().data });
    }

    async function syncImpl(
      patchOrKey: Partial<V> | keyof V,
      maybeValue?: V[keyof V],
    ): Promise<void> {
      let delta: Partial<V>;
      let next: V;
      if (patchOrKey !== null && typeof patchOrKey === 'object' && !Array.isArray(patchOrKey)) {
        delta = patchOrKey as Partial<V>;
        next = { ...get().data, ...delta };
      } else {
        const k = patchOrKey as keyof V;
        delta = { [k]: maybeValue } as Partial<V>;
        next = { ...get().data, [k]: maybeValue };
      }
      set({ data: next });
      await emit(`sync:${key}`, { delta });
      await saveLocal(key, shouldPersist, { data: next });
    }

    return {
      data: initialValues,
      initialized: !shouldPersist,
      sync: syncImpl as StoreType['sync'],
      syncAll: async (next: V, persistLocal: boolean = true) => {
        await apply({ data: next }, persistLocal);
      },
      reset: async () => {
        await apply({ data: initialValues });
      },
    };
  });

  if (!setupMap.get(key)) {
    setupMap.set(key, true);
    void listen(`sync:${key}`, async (event) => {
      const payload = event.payload as Record<string, unknown>;
      if (payload.delta && typeof payload.delta === 'object' && !Array.isArray(payload.delta)) {
        const current = store.getState().data as Record<string, unknown>;
        store.setState({
          data: { ...current, ...(payload.delta as Record<string, unknown>) } as V,
        });
      } else if (payload.data) {
        store.setState({ data: payload.data as V });
      }
    });

    if (shouldPersist) {
      const initPromise = (async function initFromFile() {
        const obj = await getLocal(key);
        if (obj) {
          store.setState({ data: { ...initialValues, ...obj } as V, initialized: true });
        } else {
          store.setState({ initialized: true });
        }
      })();
      pendingInits.set(key, initPromise);
    }
  }

  return store;
}

export async function prepareSync(): Promise<void> {
  await Promise.all(pendingInits.values());
}

function getStore(key: string): Promise<Store> {
  return load(`${key}.json`);
}

async function saveLocal(key: string, persist: boolean, payload: { data: unknown }): Promise<void> {
  if (!persist) return;
  const inst = await getStore(key);
  const v = payload.data;
  if (v && typeof v === 'object' && !Array.isArray(v)) {
    const record = v as Record<string, unknown>;
    for (const nk of Object.keys(record)) {
      await inst.set(nk, record[nk]);
    }
    await inst.save();
  }
}

async function getLocal(key: string): Promise<Record<string, unknown> | null> {
  const inst = await getStore(key);
  const entries = await inst.entries<[string, unknown]>();
  if (!(entries && entries.length > 0)) {
    return null;
  }
  return Object.fromEntries(entries) as Record<string, unknown>;
}
