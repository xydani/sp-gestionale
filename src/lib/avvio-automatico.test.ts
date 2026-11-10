import { beforeEach, describe, expect, it, vi } from "vitest";

const plugin = vi.hoisted(() => ({ enable: vi.fn(), disable: vi.fn(), isEnabled: vi.fn() }));
vi.mock("@tauri-apps/plugin-autostart", () => plugin);

import { avvioAutomaticoAttivo, impostaAvvioAutomatico } from "./avvio-automatico";

describe("avvio automatico", () => {
  beforeEach(() => vi.clearAllMocks());

  it("attiva e disattiva tramite il plugin", async () => {
    await impostaAvvioAutomatico(true);
    expect(plugin.enable).toHaveBeenCalledOnce();
    await impostaAvvioAutomatico(false);
    expect(plugin.disable).toHaveBeenCalledOnce();
  });

  it("legge lo stato dal plugin", async () => {
    plugin.isEnabled.mockResolvedValue(true);
    expect(await avvioAutomaticoAttivo()).toBe(true);
  });
});
