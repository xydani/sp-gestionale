import { leggiImpostazioni, leggiLogoPalestra } from "./api";

/** Dati della palestra mostrati nell'interfaccia (es. nel menu). */
export const palestra = $state<{ nome: string; logo: string | null }>({ nome: "", logo: null });

export async function caricaNomePalestra(): Promise<void> {
  palestra.nome = (await leggiImpostazioni()).nomePalestra;
}

/** Il logo scelto dallo staff; se non c'è o non si legge si mostra quello predefinito. */
export async function caricaLogoPalestra(): Promise<void> {
  try {
    palestra.logo = await leggiLogoPalestra();
  } catch {
    palestra.logo = null;
  }
}
