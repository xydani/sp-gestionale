import { describe, expect, it } from "vitest";
import { schedaDaIndirizzo } from "./schede-impostazioni";

describe("schedaDaIndirizzo", () => {
  it("apre la scheda indicata nell'indirizzo", () => {
    expect(schedaDaIndirizzo("backup")).toBe("backup");
  });

  it("senza indicazione o con una scheda sconosciuta apre la prima", () => {
    expect(schedaDaIndirizzo(null)).toBe("palestra");
    expect(schedaDaIndirizzo("inesistente")).toBe("palestra");
  });
});
