export async function fingerprint(keyBytes: Uint8Array) {
  const hash = await crypto.subtle.digest("SHA-256", keyBytes);
  const hashArray = Array.from(new Uint8Array(hash));
  return hashArray.map((b) => b.toString(16).padStart(2, "0")).join(":");
}
