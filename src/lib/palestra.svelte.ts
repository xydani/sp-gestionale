import { leggiImpostazioni } from "./api";

/** Dati della palestra mostrati nell'interfaccia (es. nel menu). */
export const palestra = $state({ nome: "" });

export async function caricaNomePalestra(): Promise<void> {
  palestra.nome = (await leggiImpostazioni()).nomePalestra;
}
