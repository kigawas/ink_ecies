import { ECIES_CONFIG, PrivateKey, PublicKey, decrypt, utils } from "eciesjs";
import { useEffect, useMemo, useState } from "react";
import {
  Card,
  CardContent,
  CardFooter,
  CardHeader,
  CardTitle,
} from "./components/ui/card";
import { Input } from "./components/ui/input";

ECIES_CONFIG.symmetricAlgorithm = "xchacha20";

function App() {
  const sk = useMemo(() => new PrivateKey(), []);

  const [pk, setPk] = useState<PublicKey | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [text, setText] = useState("");

  useEffect(() => {
    setPk(sk.publicKey);
  }, [sk]);

  return (
    <main className="grid grid-cols-1 place-items-center max-w-8xl">
      <Card className="w-1/2">
        <CardHeader>
          <CardTitle>ECIES Frontend</CardTitle>
        </CardHeader>

        <CardContent>
          <p className="break-all">Your public key: {"0x" + pk?.toHex(true)}</p>
        </CardContent>

        {error !== null ? (
          <CardContent>
            <p className="text-red-600">Error: {error}</p>
          </CardContent>
        ) : (
          <CardContent>
            <p>Decrypted: {text}</p>
          </CardContent>
        )}

        <CardContent>
          <div className="flex flex-col px-8 space-y-4 ">
            <Input
              placeholder="Input encrypted data"
              type="text"
              onChange={(e) => {
                const decoded = utils.decodeHex(e.target.value);

                try {
                  const decrypted = decrypt(sk.toHex(), decoded);
                  setText(decrypted.toString());
                  setError(null);
                } catch (e) {
                  if (e instanceof Error) {
                    setText("");
                    setError("invalid input");
                    console.error(e.message);
                  }
                }
              }}
            />
          </div>
        </CardContent>
        <CardFooter></CardFooter>
      </Card>
    </main>
  );
}

export default App;
