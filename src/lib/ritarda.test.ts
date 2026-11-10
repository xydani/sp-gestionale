import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { ritarda } from "./ritarda";

describe("ritarda", () => {
  beforeEach(() => vi.useFakeTimers());
  afterEach(() => vi.useRealTimers());

  it("esegue solo l'ultima chiamata dopo la pausa", () => {
    const azione = vi.fn();
    const azioneRitardata = ritarda(azione, 200);

    azioneRitardata("r");
    azioneRitardata("ro");
    vi.advanceTimersByTime(199);
    azioneRitardata("ros");
    vi.advanceTimersByTime(200);

    expect(azione).toHaveBeenCalledTimes(1);
    expect(azione).toHaveBeenCalledWith("ros");
  });
});
